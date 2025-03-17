use anyhow::{Context, bail};
use std::env::current_dir;
use std::path::PathBuf;
use std::process::{Command as StdCommand, Stdio};
use wasm_encoder::{ComponentSectionId, Encode as _, RawSection, Section as _};

pub const RUST_TARGET: &str = "wasm32-wasip1";
const WASI_ADAPTER_URL: &str = "https://github.com/bytecodealliance/wasmtime/releases/download/v18.0.2/wasi_snapshot_preview1.reactor.wasm";

pub fn install_rust_wasm_target_if_needed() -> anyhow::Result<()> {
    let rustc_output = StdCommand::new("rustc")
        .arg("--print")
        .arg("sysroot")
        .output()
        .context("failed to run rustc")?;
    if !rustc_output.status.success() {
        bail!(
            "failed to retrieve rust sysroot: {}",
            String::from_utf8_lossy(&rustc_output.stderr)
        );
    }

    let sysroot = PathBuf::from(String::from_utf8(rustc_output.stdout)?.trim());
    if sysroot.join("lib/rustlib").join(RUST_TARGET).exists() {
        return Ok(());
    }

    let output = StdCommand::new("rustup")
        .args(["target", "add", RUST_TARGET])
        .stderr(Stdio::piped())
        .stdout(Stdio::inherit())
        .output()
        .context("failed to run `rustup target add`")?;
    if !output.status.success() {
        bail!(
            "failed to install the `{RUST_TARGET}` target: {}",
            String::from_utf8_lossy(&rustc_output.stderr)
        );
    }

    Ok(())
}

pub async fn install_wasi_preview1_adapter_if_needed() -> anyhow::Result<Vec<u8>> {
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
pub fn strip_custom_sections(input: &[u8]) -> anyhow::Result<Vec<u8>> {
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
