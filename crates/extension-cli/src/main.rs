use anyhow::{bail, Context};
use clap::{Parser, Subcommand};
use extension::info::ExtensionInfo;
use extension::{WasmExtension, EXTENSION_FILE_NAME, MANIFEST_FILE_NAME};
use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::path::PathBuf;
use std::process::Command as StdCommand;
use std::sync::Arc;
use tokio::fs::File;
use wit_component::ComponentEncoder;

mod wasm_compile;
use crate::wasm_compile::RUST_TARGET;
use wasm_compile::{install_rust_wasm_target_if_needed, install_wasi_preview1_adapter_if_needed, strip_custom_sections};

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
        let manifest_text = std::fs::read_to_string(path)?;
        match anyhow::Context::context(path.extension(), "No file extension found for manifest")?.to_str().unwrap() {
            "toml" => {
                let manifest: Manifest = toml::from_str(&manifest_text)?;
                Ok(manifest)
            }
            "json" => {
                let manifest: Manifest = serde_json::from_str(&manifest_text)?;
                Ok(manifest)
            }
            "yaml" => {
                let manifest: Manifest = serde_yaml::from_str(&manifest_text)?;
                Ok(manifest)
            }
            "yml" => {
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
    let cargo_toml_path = anyhow::Context::context(path.parent(), "Could not get parent (programmer error)")?.join("Cargo.toml");
    let cargo_info = cargo_toml::Manifest::from_path(&cargo_toml_path).context(format!("Could not find Cargo.toml at {}", cargo_toml_path.display()))?;
    let package = cargo_info.package.context("Could not find package info in Cargo.toml")?;
    let ext_info = ExtensionInfo {
        id: manifest.identifier,
        name: package.name,
        version: package.version.get().context("Version in Cargo.toml is inherited, please specify a raw version")?.to_string(),
        description: package.description.map(|s| s.get().context("Description in Cargo.toml is inherited, please specify a raw description").map(String::to_string).unwrap_or("".to_string())),
        authors: Some(package.authors.get().context("Authors in Cargo.toml is inherited, please specify raw authors")?.into_iter().map(|a| a.to_string()).collect()),
        license: package.license.map(|l| l.get().context("License in Cargo.toml is inherited, please specify a raw license").map(String::to_string).unwrap_or("".to_string())),
        repository: package.repository.map(|r| r.get().context("Repository in Cargo.toml is inherited, please specify a raw repository").map(String::to_string).unwrap_or("".to_string())),
        homepage: package.homepage.map(|h| h.get().context("Homepage in Cargo.toml is inherited, please specify a raw homepage").map(String::to_string).unwrap_or("".to_string())),
        documentation: package.documentation.map(|d| d.get().context("Documentation in Cargo.toml is inherited, please specify a raw documentation").map(String::to_string).unwrap_or("".to_string())),
        keywords: Some(package.keywords.get().context("Keywords in Cargo.toml is inherited, please specify raw keywords")?.into_iter().map(|k| k.to_string()).collect()),
        permissions: manifest.permissions.into(),
    };
    Ok(ext_info)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    color_eyre::install().expect("Failed to initialize color_eyre");
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .context("Failed to initialize logger")?;
    let args = Arguments::parse();
    match args.command {
        Command::Init { path } => {
            init(&path.canonicalize().unwrap())?;
        }
        Command::Package {
            path,
            manifest_path,
            output_dir,
        } => {
            let ext_info = get_extension_info(&path, manifest_path.as_ref())?;
            let _ = tokio::fs::create_dir(&output_dir).await;
            let main_output_dir = output_dir.join(format!("{}_{}", ext_info.name, ext_info.version));
            let _ = tokio::fs::create_dir(&main_output_dir).await;
            let wasm_output = main_output_dir.join(EXTENSION_FILE_NAME);
            println!("Compiling wasm ...");
            build_wasm(&path, &wasm_output, true, false).await?;

            println!("Compiling manifest ...");
            let manifest_output = main_output_dir.join(MANIFEST_FILE_NAME);
            let manifest_text = toml::to_string(&ext_info)?;
            tokio::fs::write(manifest_output, &manifest_text).await?;

            println!("Creating tarball ...");
            let tar_output = output_dir.join(format!("{}_{}.tar", ext_info.name, ext_info.version));
            let file = std::fs::File::create(&tar_output)?;
            let mut archive = tar::Builder::new(file);
            archive.append_dir_all(".", &main_output_dir)?;
            let _ = archive.into_inner()?;

            println!("Compressing tarball ...");
            let gz_output = output_dir.join(format!("{}_{}.tar.gz", ext_info.name, ext_info.version));
            let tar_file = File::open(&tar_output).await?;
            let _ = File::create(&gz_output).await?;
            let gz_file = File::options().write(true).open(&gz_output).await?;
            let mut encoder = async_compression::tokio::write::GzipEncoder::with_quality(gz_file, async_compression::Level::Best);
            tokio::io::copy(&mut tokio::io::BufReader::new(tar_file), &mut encoder).await?;

            // Clean up the tar file
            println!("Cleaning up ...");
            tokio::fs::remove_dir_all(&main_output_dir).await.context(format!("Failed to remove output directory: {}", &main_output_dir.display()))?;
            tokio::fs::remove_file(&tar_output).await.context(format!("Failed to remove tar file: {}", &tar_output.display()))?;
        }
        Command::Compile { input, output, no_strip } => {
            compile_wasm(&input, &output, no_strip).await?;
        }
        Command::Build { output, release, no_strip } => {
            build_wasm(&current_dir()?, &output, release, no_strip).await?;
        }
        Command::TestLoad { path } => {
            WasmExtension::load(
                path.clone(),
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
        .module(&wasm_bytes)?
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
    let input = cwd.join("target").join(RUST_TARGET).join(if release {
        "release"
    } else {
        "debug"
    }).join(format!("{}.wasm", name.replace("-", "_")));
    compile_wasm(&input, &output, no_strip).await?;
    Ok(())
}

fn init(path: &PathBuf) -> anyhow::Result<()> {
    if !path.is_dir() {
        panic!("Path must be a directory");
    }
    if get_manifest_path(&path, None).is_none() {
        let manifest_path = path.join("prontus_ext.toml");
        let default_ident = path.file_name().unwrap().to_str().unwrap();
        let mut ident = inquire::Text::new(&format!("What should the id be ({default_ident})?"))
            .prompt()
            .context("Failed to prompt for id")?;
        if ident.is_empty() {
            ident = default_ident.to_string();
        }
        let permissions = Permissions::default();
        let manifest = Manifest {
            identifier: ident,
            permissions,
        };
        let manifest_text = toml::to_string(&manifest)?;
        std::fs::write(manifest_path, manifest_text)?;
    }
    Ok(())
}
