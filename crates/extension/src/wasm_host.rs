use crate::info::ExtensionInfo;
use std::error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use thiserror::Error;
use wasmtime::component::{Component, ResourceTable};
use wasmtime_wasi::WasiCtxBuilder;

pub(crate) mod wit;

fn wasm_engine() -> wasmtime::Engine {
    static WASM_ENGINE: OnceLock<wasmtime::Engine> = OnceLock::new();

    WASM_ENGINE
        .get_or_init(|| {
            let mut config = wasmtime::Config::new();
            config.wasm_component_model(true);
            config.async_support(true);
            wasmtime::Engine::new(&config).expect("Failed to create Wasmtime Engine")
        })
        .clone()
}

pub struct WasmState {
    extension_info: Arc<ExtensionInfo>,
    ctx: wasmtime_wasi::WasiCtx,
    table: ResourceTable,
}

#[derive(Debug, Error)]
pub enum WasmExtensionError {
    #[error("I/O Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Wasmtime Error: {0}")]
    WasmtimeError(#[from] wasmtime::Error),
}

pub struct WasmExtension {
    #[allow(dead_code)]
    engine: wasmtime::Engine,
    extension: wit::Extension,
    store: wasmtime::Store<WasmState>,
    pub info: Arc<ExtensionInfo>,
}

impl WasmExtension {
    pub async fn load(
        extension_path: PathBuf,
        info: Arc<ExtensionInfo>,
    ) -> Result<Self, WasmExtensionError> {
        let path = extension_path;

        let mut wasm_file = File::open(path)?;

        let mut wasm_bytes = Vec::new();
        wasm_file.read_to_end(&mut wasm_bytes)?;

        let engine = wasm_engine();
        let ctx = { WasiCtxBuilder::new().inherit_stdio().build() };
        let mut store = wasmtime::Store::new(
            &engine,
            WasmState {
                extension_info: info.clone(),
                ctx,
                table: ResourceTable::new(),
            },
        );

        let component = Component::from_binary(&store.engine(), &wasm_bytes)?;

        let extension = wit::Extension::instantiate_async(&mut store, &component).await?;

        extension.init_extension(&mut store).await?;

        Ok(Self {
            engine,
            extension,
            store,
            info,
        })
    }

    pub async fn run_task(&mut self) -> anyhow::Result<()> {
        Ok(self.extension.run_task(&mut self.store).await?)
    }
}

impl Drop for WasmExtension {
    fn drop(&mut self) {
        // TODO: Wait for async drop to be stabilized
        let _ = self.extension.shutdown_extension(&mut self.store);
    }
}

impl wasmtime_wasi::WasiView for WasmState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut wasmtime_wasi::WasiCtx {
        &mut self.ctx
    }
}
