use std::sync::OnceLock;
use log::Level;
use wasmtime::component::Linker;
use crate::wasm_host::WasmState;

wasmtime::component::bindgen!({
    async: true,
    trappable_imports: true,
    path: "../extension-api/wit/since_v0.1.0",
    with: {}
});

impl From<settings::Settings> for Settings {
    fn from(value: settings::Settings) -> Self {
        Settings {}
    }
}

pub fn linker() -> &'static Linker<WasmState> {
    static LINKER: OnceLock<Linker<WasmState>> = OnceLock::new();
    LINKER.get_or_init(|| super::new_linker(Extension::add_to_linker))
}

#[wasmtime::component::__internal::async_trait]
impl ExtensionImports for WasmState {
    async fn get_settings(&mut self) -> wasmtime::Result<Result<Settings, ()>> {
        if self.extension_info.permissions.read_settings {
            Ok(Ok(settings::Settings::load().await.unwrap().into()))
        } else {
            Ok(Err(()))
        }
    }

    async fn set_settings(&mut self, settings: Settings) -> wasmtime::Result<Result<(), ()>> {
        if self.extension_info.permissions.write_settings {
            todo!("set_settings");
            Ok(Ok(()))
        } else {
            Ok(Err(()))
        }
    }

    async fn request_url(&mut self, method: wasmtime::component::__internal::String, url: wasmtime::component::__internal::String) -> wasmtime::Result<Result<NetworkResponse, ()>> {
        if self.extension_info.permissions.full_network {
            todo!()
        } else {
            Ok(Err(()))
        }
    }

    async fn log_trace(&mut self, message: wasmtime::component::__internal::String) -> wasmtime::Result<()> {
        log::log!(Level::Trace, "{}", message);
        Ok(())
    }

    async fn log_debug(&mut self, message: wasmtime::component::__internal::String) -> wasmtime::Result<()> {
        log::log!(Level::Debug, "{}", message);
        Ok(())
    }

    async fn log_info(&mut self, message: wasmtime::component::__internal::String) -> wasmtime::Result<()> {
        log::log!(Level::Info, "{}", message);
        Ok(())
    }

    async fn log_warning(&mut self, message: wasmtime::component::__internal::String) -> wasmtime::Result<()> {
        log::log!(Level::Warn, "{}", message);
        Ok(())
    }

    async fn log_error(&mut self, message: wasmtime::component::__internal::String) -> wasmtime::Result<()> {
        log::log!(Level::Error, "{}", message);
        Ok(())
    }
}
