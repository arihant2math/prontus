use wasmtime::component::{Component, Linker};
use wasmtime::Store;
use wasmtime::component::__internal::anyhow;
use crate::wasm_host::{wasm_engine, WasmState};

mod since_v0_1_0;

use since_v0_1_0 as latest;

fn wasi_view(state: &mut WasmState) -> &mut WasmState {
    state
}

pub fn new_linker(
    f: impl Fn(&mut Linker<WasmState>, fn(&mut WasmState) -> &mut WasmState) -> anyhow::Result<()>,
) -> Linker<WasmState> {
    let mut linker = Linker::new(&wasm_engine());
    wasmtime_wasi::add_to_linker_async(&mut linker).unwrap();
    f(&mut linker, wasi_view).unwrap();
    linker
}

pub enum Extension {
    V010(since_v0_1_0::Extension)
}

impl Extension {
    pub async fn instantiate_async(
        store: &mut Store<WasmState>,
        component: &Component,
    ) -> anyhow::Result<Self> {
        Ok(latest::Extension::instantiate_async(store, component, latest::linker())
            .await
            .map(Extension::V010)?)
    }

    pub async fn init_extension(&self, store: &mut Store<WasmState>) -> anyhow::Result<()> {
        match self {
            Extension::V010(ext) => ext.call_init_extension(store).await,
        }
    }

    pub async fn run_task(&self, store: &mut Store<WasmState>) -> anyhow::Result<()> {
        match self {
            Extension::V010(ext) => ext.call_run_task(store).await,
        }
    }

    pub async fn shutdown_extension(&self, store: &mut Store<WasmState>) -> anyhow::Result<()> {
        match self {
            Extension::V010(ext) => ext.call_shutdown_extension(store).await,
        }
    }
}
