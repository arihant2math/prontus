use std::sync::OnceLock;
use log::Level;
use wasmtime::component::Linker;
use crate::wasm_host::WasmState;
use reqwest::{Client, Method};

wasmtime::component::bindgen!({
    async: true,
    trappable_imports: true,
    path: "../extension-api/wit/since_v0.1.0",
    with: {}
});


pub fn linker() -> &'static Linker<WasmState> {
    static LINKER: OnceLock<Linker<WasmState>> = OnceLock::new();
    LINKER.get_or_init(|| super::new_linker(Extension::add_to_linker))
}

#[wasmtime::component::__internal::async_trait]
impl ExtensionImports for WasmState {
    async fn get_settings(&mut self) -> wasmtime::Result<Result<String, ()>> {
        if self.extension_info.permissions.read_settings {
            let s = serde_json::to_string(&settings::Settings::load().await.unwrap()).unwrap();
            Ok(Ok(s))
        } else {
            Ok(Err(()))
        }
    }

    async fn set_settings(&mut self, settings: String) -> wasmtime::Result<Result<(), ()>> {
        if self.extension_info.permissions.write_settings {
            let settings: settings::Settings = serde_json::from_str(&settings).unwrap();
            settings.save().await.unwrap();
            Ok(Ok(()))
        } else {
            Ok(Err(()))
        }
    }

    async fn request_url(&mut self, method: String, url: String) -> wasmtime::Result<Result<NetworkResponse, ()>> {
        if self.extension_info.permissions.full_network {
            let client = Client::new();
            let request = client.request(match method.as_str() {
                "get" => Method::GET,
                "post" => Method::POST,
                "put" => Method::PUT,
                "patch" => Method::PATCH,
                "delete" => Method::DELETE,
                "head" => Method::HEAD,
                _ => return Ok(Err(())),
            }, url);
            let resp = request.send().await.unwrap();
            Ok(Ok(NetworkResponse {
                status: resp.status().as_u16() as u32,
                body: resp.text().await.unwrap(),
            }))
        } else {
            Ok(Err(()))
        }
    }

    async fn log_trace(&mut self, message: String) -> wasmtime::Result<()> {
        log::log!(Level::Trace, "{}", message);
        Ok(())
    }

    async fn log_debug(&mut self, message: String) -> wasmtime::Result<()> {
        log::log!(Level::Debug, "{}", message);
        Ok(())
    }

    async fn log_info(&mut self, message: String) -> wasmtime::Result<()> {
        log::log!(Level::Info, "{}", message);
        Ok(())
    }

    async fn log_warning(&mut self, message: String) -> wasmtime::Result<()> {
        log::log!(Level::Warn, "{}", message);
        Ok(())
    }

    async fn log_error(&mut self, message: String) -> wasmtime::Result<()> {
        log::log!(Level::Error, "{}", message);
        Ok(())
    }
}
