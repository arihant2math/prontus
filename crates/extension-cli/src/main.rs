use anyhow::{bail, Context};
use clap::{Parser, Subcommand};
use extension::info::ExtensionInfo;
use extension::{WasmExtension, EXTENSION_FILE_NAME, MANIFEST_FILE_NAME};
use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::sync::Arc;
use wit_component::ComponentEncoder;
use wasm_encoder::{ComponentSectionId, Encode as _, RawSection, Section as _};
use std::process::{Stdio, Command as StdCommand};
use color_eyre::eyre::ContextCompat;

mod wasm_compile;
use wasm_compile::{strip_custom_sections, install_wasi_preview1_adapter_if_needed, install_rust_wasm_target_if_needed};
use crate::wasm_compile::RUST_TARGET;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Permissions {
    pub read_settings: Option<bool>,
    pub write_settings: Option<bool>,
    pub full_network: Option<bool>,
}

impl Into<extension::info::Permissions> for Permissions {
    fn into(self) -> extension::info::Permissions {
        extension::info::Permissions {
            read_settings: self.read_settings.unwrap_or(false),
            write_settings: self.write_settings.unwrap_or(false),
            full_network: self.full_network.unwrap_or(false),
        }
    }
}

impl From<extension::info::Permissions> for Permissions {
    fn from(permissions: extension::info::Permissions) -> Self {
        Self {
            read_settings: Some(permissions.read_settings),
            write_settings: Some(permissions.write_settings),
            full_network: Some(permissions.full_network),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Manifest {
    pub identifier: String,
    pub permissions: Permissions,
}

impl Manifest {
    pub fn open(path: &PathBuf) -> anyhow::Result<Self> {
        match path.extension().context("No file extension found for manifest")?.to_str().unwrap() {
            "toml" => {
                let manifest_text = std::fs::read_to_string(path)?;
                let manifest: Manifest = toml::from_str(&manifest_text)?;
                Ok(manifest)
            }
            "json" => {
                let manifest_text = std::fs::read_to_string(path)?;
                let manifest: Manifest = serde_json::from_str(&manifest_text)?;
                Ok(manifest)
            }
            "yaml" => {
                let manifest_text = std::fs::read_to_string(path)?;
                let manifest: Manifest = serde_yaml::from_str(&manifest_text)?;
                Ok(manifest)
            }
            "yml" => {
                let manifest_text = std::fs::read_to_string(path)?;
                let manifest: Manifest = serde_yaml::from_str(&manifest_text)?;
                Ok(manifest)
            }
            _ => bail!("Unsupported file extension for manifest"),
        }
    }
}

fn default_path() -> PathBuf {
    current_dir().unwrap()
}

fn default_output_path() -> PathBuf {
    default_path().join("bundle")
}

#[derive(Clone, Subcommand)]
enum Command {
    Init {
        path: PathBuf,
    },
    Package {
        #[arg(default_value=default_path().into_os_string())]
        path: PathBuf,
        #[arg(short, long)]
        manifest_path: Option<PathBuf>,
        #[arg(default_value=default_output_path().into_os_string())]
        output_dir: PathBuf,
    },
    Build {
        output: PathBuf,
        #[clap(long, short, action)]
        release: bool,
        #[clap(long, short, action)]
        no_strip: bool,
    },
    Compile {
        input: PathBuf,
        output: PathBuf,
        #[clap(long, short, action)]
        no_strip: bool,
    },
    TestLoad {
        path: PathBuf
    },
}

#[derive(Clone, Parser)]
struct Arguments {
    #[command(subcommand)]
    command: Command,
}

fn get_manifest_path(root: &PathBuf, custom: Option<&PathBuf>) -> Option<PathBuf> {
    if let Some(path) = custom {
        return if path.exists() {
            return Some(path.clone());
        } else {
            None
        };
    }
    if root.join("prontus_ext.toml").exists() {
        Some(root.join("prontus_ext.toml"))
    } else if root.join("prontus_ext.json").exists() {
        Some(root.join("prontus_ext.json"))
    } else if root.join("prontus_ext.yaml").exists() {
        Some(root.join("prontus_ext.yaml"))
    } else {
        None
    }
}

fn get_extension_info(path: &PathBuf, custom_manifest_path: Option<&PathBuf>) -> anyhow::Result<ExtensionInfo> {
    let path = get_manifest_path(path, custom_manifest_path).context("Could not find extension info file")?;
    let manifest = Manifest::open(&path)?;
    let cargo_info = cargo_toml::Manifest::from_path(path.join("Cargo.toml"))?;
    let package = cargo_info.package.context("Could not find package info in Cargo.toml")?;
    let ext_info = ExtensionInfo {
        id: manifest.identifier,
        name: package.name,
        version: package.version.context("Version in Cargo.toml is inherited, please specify a raw version")?,
        description: package.description,
        authors: package.authors,
        license: package.license,
        repository: package.repository,
        homepage: package.homepage,
        documentation: package.documentation,
        keywords: package.keywords,
        permissions: manifest.permissions.into(),
    };
    Some(ext_info)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    color_eyre::install().unwrap();
    let args = Arguments::parse();
    match args.command {
        Command::Init { path } => {
            init(&path);
        }
        Command::Package {
            path,
            manifest_path,
            output_dir,
        } => {
            let ext_info = get_extension_info(&path, manifest_path.as_ref())?;
            std::fs::create_dir(&output_dir).context("Failed to create output directory")?;
            let main_output_dir = output_dir.join(format!("{}_{}", ext_info.name, ext_info.version));
            std::fs::create_dir(&main_output_dir).context("Failed to create output directory")?;
            let wasm_output = main_output_dir.join(EXTENSION_FILE_NAME);
            build_wasm(&path, &wasm_output, true, false).await?;
            let manifest_output = main_output_dir.join(MANIFEST_FILE_NAME);
            let manifest_text = toml::to_string(&ext_info)?;
            std::fs::write(manifest_output, &manifest_text)?;
        }
        Command::Compile { input, output, no_strip } => {
            compile_wasm(&input, &output, no_strip).await?;
        }
        Command::Build { output, release, no_strip } => {
            build_wasm(&current_dir()?, &output, release, no_strip).await?;
        }
        Command::TestLoad { path } => {
            WasmExtension::load(
                path,
                Arc::new(get_extension_info(&path, None)?),
            ).await?;
        }
    };
    Ok(())
}

async fn compile_wasm(input_path: &PathBuf, output: &PathBuf, no_strip: bool) -> anyhow::Result<()> {
    let adapter_bytes = install_wasi_preview1_adapter_if_needed().await?;
    println!("Reading wasm module from {}", input_path.display());
    let wasm_bytes = std::fs::read(input_path)?;
    println!("Encoding wasm component ...");
    let mut encoder = ComponentEncoder::default()
        .module(&wasm_bytes)
        .unwrap()
        .adapter("wasi_snapshot_preview1", &adapter_bytes)
        .context("failed to load adapter module")?
        .validate(true);
    let component_bytes = if !no_strip {
        println!("Stripping debug sections ...");
        let component_bytes = encoder.encode().context("failed to encode wasm component")?;
        strip_custom_sections(&component_bytes)?
    } else {
        let component_bytes = encoder.encode().context("failed to encode wasm component")?;
        component_bytes
    };
    println!("Writing wasm component to {}", output.display());
    std::fs::write(output, &component_bytes)?;
    Ok(())
}

async fn build_wasm(cwd: &PathBuf, output: &PathBuf, release: bool, no_strip: bool) -> anyhow::Result<()> {
    install_rust_wasm_target_if_needed()?;
    StdCommand::new("cargo")
        .current_dir(cwd)
        .arg("build")
        .arg("--target")
        .arg(RUST_TARGET)
        .arg(if release { "--release" } else { "--debug" })
        .output()
        .context("failed to run cargo build")?;
    let name = cwd.file_name().unwrap().to_str().unwrap().to_string();
    // TODO: fix this
    let input = cwd.join("target").join(RUST_TARGET).join(if release {
        "release"
    } else {
        "debug"
    }).join(format!("{}.wasm", name.replace("-", "_")));
    compile_wasm(&input, &output, no_strip).await?;
    Ok(())
}

fn init(path: &PathBuf) {
    if !path.is_dir() {
        panic!("Path must be a directory");
    }
    if get_manifest_path(&path, None).is_none() {
        let manifest_path = path.join("prontus_ext.toml");
        let default_ident = path.file_name().unwrap().to_str().unwrap();
        let mut ident = inquire::Text::new(&format!("What should the id be ({default_ident})?"))
            .prompt()
            .unwrap();
        if ident.is_empty() {
            ident = default_ident.to_string();
        }
        let permissions = Permissions::default();
        let manifest = Manifest {
            identifier: ident,
            permissions,
        };
        let manifest_text = toml::to_string(&manifest).unwrap();
        std::fs::write(manifest_path, manifest_text).unwrap();
    }
}
