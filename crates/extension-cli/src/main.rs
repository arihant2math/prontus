use anyhow::{bail, Context};
use clap::{Parser, Subcommand};
use extension::info::ExtensionInfo;
use extension::WasmExtension;
use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::path::PathBuf;
use std::sync::Arc;
use wit_component::ComponentEncoder;
use wasm_encoder::{ComponentSectionId, Encode as _, RawSection, Section as _};

const WASI_ADAPTER_URL: &str =
    "https://github.com/bytecodealliance/wasmtime/releases/download/v18.0.2/wasi_snapshot_preview1.reactor.wasm";

async fn install_wasi_preview1_adapter_if_needed() -> anyhow::Result<Vec<u8>> {
    let cache_path = current_dir()?.join("wasi_snapshot_preview1.reactor.wasm");
    if let Ok(content) = tokio::fs::read(&cache_path).await {
        if wasmparser::Parser::is_core_wasm(&content) {
            return Ok(content);
        }
    }

    let _ = tokio::fs::remove_file(&cache_path).await;

    println!(
        "downloading wasi adapter module to {}",
        cache_path.display()
    );
    let response = reqwest::get(WASI_ADAPTER_URL).await?;

    let content = response.bytes().await?;

    tokio::fs::write(&cache_path, &content)
        .await
        .with_context(|| format!("failed to save file {}", cache_path.display()))?;

    if !wasmparser::Parser::is_core_wasm(&content) {
        bail!("downloaded wasi adapter is invalid");
    }
    Ok(content.to_vec())
}

// This was adapted from:
// https://github.com/zed-industries/zed/blob/02cc0b9afa8eb6162b9e65d1801633cb6f38154e/crates/extension/src/extension_builder.rs#L456C5-L458C5
// which was adapted from:
// https://github.com/bytecodealliance/wasm-tools/blob/1791a8f139722e9f8679a2bd3d8e423e55132b22/src/bin/wasm-tools/strip.rs
fn strip_custom_sections(input: &[u8]) -> anyhow::Result<Vec<u8>> {
    use wasmparser::Payload::*;

    let strip_custom_section = |name: &str| name.starts_with(".debug");

    let mut output = Vec::new();
    let mut stack = Vec::new();

    for payload in wasmparser::Parser::new(0).parse_all(input) {
        let payload = payload?;
        let component_header = wasm_encoder::Component::HEADER;
        let module_header = wasm_encoder::Module::HEADER;

        // Track nesting depth, so that we don't mess with inner producer sections:
        match payload {
            Version { encoding, .. } => {
                output.extend_from_slice(match encoding {
                    wasmparser::Encoding::Component => &component_header,
                    wasmparser::Encoding::Module => &module_header,
                });
            }
            ModuleSection { .. } | ComponentSection { .. } => {
                stack.push(std::mem::take(&mut output));
                continue;
            }
            End { .. } => {
                let mut parent = match stack.pop() {
                    Some(c) => c,
                    None => break,
                };
                if output.starts_with(&component_header) {
                    parent.push(ComponentSectionId::Component as u8);
                    output.encode(&mut parent);
                } else {
                    parent.push(ComponentSectionId::CoreModule as u8);
                    output.encode(&mut parent);
                }
                output = parent;
            }
            _ => {}
        }

        if let CustomSection(c) = &payload {
            if strip_custom_section(c.name()) {
                continue;
            }
        }

        if let Some((id, range)) = payload.as_section() {
            RawSection {
                id,
                data: &input[range],
            }
                .append_to(&mut output);
        }
    }

    Ok(output)
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Permissions {
    pub read_settings: Option<bool>,
    pub write_settings: Option<bool>,
    pub full_network: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Manifest {
    pub identifier: String,
    pub permissions: Permissions,
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

fn get_extension_info_path(path: &PathBuf) -> Option<PathBuf> {
    if path.join("prontus_ext.toml").exists() {
        Some(path.join("prontus_ext.toml"))
    } else if path.join("prontus_ext.json").exists() {
        Some(path.join("prontus_ext.json"))
    } else if path.join("prontus_ext.yaml").exists() {
        Some(path.join("prontus_ext.yaml"))
    } else {
        None
    }
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
            let manifest_path = manifest_path.unwrap_or_else(|| {
                let current_dir = current_dir().unwrap();
                get_extension_info_path(&current_dir).expect("Could not find extension info file")
            });
        }
        Command::Compile { input, output, no_strip } => {
            compile_wasm(&input, &output, no_strip).await?;
        }
        Command::Build { output, no_strip } => {
            let path = current_dir()?;
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            // TODO: fix this
            let input = path.join("target").join("wasm32-unknown-unknown").join("release").join(format!("{}.wasm", name.replace("-", "_")));
            compile_wasm(&input, &output, no_strip).await?;
        }
        Command::TestLoad { path } => {
            WasmExtension::load(
                path,
                Arc::new(ExtensionInfo {
                    id: "test".to_string(),
                    name: "Test".to_string(),
                    version: "0.0.0".to_string(),
                    description: None,
                    authors: None,
                    license: None,
                    repository: None,
                    homepage: None,
                    documentation: None,
                    keywords: None,
                    permissions: Default::default(),
                }),
            )
            .await?;
        }
    };
    Ok(())
}

async fn compile_wasm(input_path: &PathBuf, output: &PathBuf, no_strip: bool) -> anyhow::Result<()>{
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

fn init(path: &PathBuf) {
    if !path.is_dir() {
        panic!("Path must be a directory");
    }
    if get_extension_info_path(&path).is_none() {
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
