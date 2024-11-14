//! A Wasm module can be compiled with multiple compilers.
//!
//! This example illustrates how to use the Cranelift compiler.
//!
//! You can run the example directly by executing in Wasmer root:
//!
//! ```shell
//! cargo run --example compiler-cranelift --release --features "cranelift"
//! ```
//!
//! Ready?

use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use std::string::FromUtf8Error;
use thiserror::Error;
use wasmer::{
    imports, wat2wasm, Cranelift, ExportError, Instance, Module, RuntimeError, Store, Value,
};

fn load(bytes: Vec<u8>) -> Result<Instance, WasmModuleLoadError> {
    // Let's declare the Wasm module with the text representation.
    // TODO: include more detailed error information
    let wasm_bytes = wat2wasm(&bytes).map_err(|e| WasmModuleLoadError::ModuleParseError)?;

    // Use Cranelift compiler with the default settings
    let compiler = Cranelift::default();

    // Create the store
    let mut store = Store::new(compiler);

    println!("Compiling module...");
    // Let's compile the Wasm module.
    let module = Module::new(&store, wasm_bytes)?;

    // Create an empty import object.
    let import_object = imports! {};

    println!("Instantiating module...");
    // Let's instantiate the Wasm module.
    let instance = Instance::new(&mut store, &module, &import_object)?;

    Ok(instance)
}

#[derive(Error, Debug)]
pub enum WasmParseError {
    CouldNotConvertValue,
    CouldNotGetValue,
    StringDecode(#[from] FromUtf8Error),
}

impl Display for WasmParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Error, Debug)]
pub enum WasmFunctionError {
    #[error("Function export error: {0}")]
    FunctionExport(#[from] ExportError),
    #[error("Function runtime error: {0}")]
    FunctionRuntime(#[from] RuntimeError),
    #[error("Function result parse error")]
    FunctionResultParse(#[from] WasmParseError),
}

#[derive(Error, Debug)]
pub enum WasmModuleLoadError {
    ModuleParseError,
    ModuleCompileError(#[from] wasmer::CompileError),
    InstanceCreationError(#[from] wasmer::InstantiationError),
}

impl Display for WasmModuleLoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

trait WasmOutput {
    fn convert(result: Box<[Value]>) -> Result<Self, WasmParseError>
    where
        Self: Sized;
}

impl WasmOutput for i32 {
    fn convert(result: Box<[Value]>) -> Result<Self, WasmParseError> {
        Ok(result
            .get(0)
            .ok_or(WasmParseError::CouldNotGetValue)?
            .i32()
            .ok_or(WasmParseError::CouldNotConvertValue)?)
    }
}

impl WasmOutput for i64 {
    fn convert(result: Box<[Value]>) -> Result<Self, WasmParseError> {
        Ok(result
            .get(0)
            .ok_or(WasmParseError::CouldNotGetValue)?
            .i64()
            .ok_or(WasmParseError::CouldNotConvertValue)?)
    }
}

impl WasmOutput for f64 {
    fn convert(result: Box<[Value]>) -> Result<Self, WasmParseError> {
        Ok(result
            .get(0)
            .ok_or(WasmParseError::CouldNotGetValue)?
            .f64()
            .ok_or(WasmParseError::CouldNotConvertValue)?)
    }
}

impl WasmOutput for bool {
    fn convert(result: Box<[Value]>) -> Result<Self, WasmParseError> {
        Ok(result
            .get(0)
            .ok_or(WasmParseError::CouldNotGetValue)?
            .i32()
            .ok_or(WasmParseError::CouldNotConvertValue)?
            != 0)
    }
}

impl WasmOutput for String {
    fn convert(result: Box<[Value]>) -> Result<String, WasmParseError> {
        // Preallocate the data vector to avoid reallocations
        let mut data = Vec::with_capacity(result.len() * 4);
        for v in &result {
            let int_value = v.i32().ok_or(WasmParseError::CouldNotConvertValue)?;
            let split = int_value.to_le_bytes();
            data.extend_from_slice(&split);
        }
        let text = String::from_utf8(data)?;
        Ok(text)
    }
}

impl WasmOutput for Vec<String> {
    fn convert(result: Box<[Value]>) -> Result<Vec<String>, WasmParseError> {
        let data: Vec<i32> = result
            .iter()
            // TODO: handle error
            .map(|v| v.i32().ok_or(WasmParseError::CouldNotConvertValue).unwrap())
            .collect();
        let mut strings = Vec::new();
        let mut string = Vec::new();
        for i in data {
            if i == -1 {
                strings.push(String::from_utf8(string.clone())?);
                string.clear();
            } else {
                let split = i.to_le_bytes();
                string.extend_from_slice(&split);
            }
        }
        Ok(strings)
    }
}

pub fn call_fn_raw(
    instance: &Instance,
    store: &mut Store,
    name: &str,
    params: &[Value],
) -> Result<Box<[Value]>, WasmFunctionError> {
    let result = instance.exports.get_function(name)?.call(store, params)?;
    Ok(result)
}

pub fn call_fn<T: WasmOutput>(
    instance: &Instance,
    store: &mut Store,
    name: &str,
    params: &[Value],
) -> Result<T, WasmFunctionError> {
    let result = instance.exports.get_function(name)?.call(store, params)?;
    Ok(T::convert(result)?)
}

pub struct ExtensionInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub authors: Vec<String>,
}

pub struct ExtensionPermissions {
    pub network: bool,
    pub filesystem: bool,
    pub full_settings: bool,
    pub extension_settings: bool,
    pub extension_hooks: Vec<String>,
}

pub struct Extension {
    pub info: ExtensionInfo,
    pub permissions: ExtensionPermissions,
    pub instance: Instance,
}

impl Extension {
    fn load(store: &mut Store, path: impl AsRef<Path>) -> Self {
        let bytes = std::fs::read(path).unwrap();
        let instance = load(bytes).unwrap();
        let info = ExtensionInfo {
            name: call_fn(&instance, store, "info_name", &[]).unwrap(),
            version: call_fn(&instance, store, "info_version", &[]).unwrap(),
            description: call_fn(&instance, store, "info_description", &[]).unwrap(),
            authors: call_fn(&instance, store, "info_authors", &[]).unwrap(),
        };
        // TODO: split later
        let permissions = ExtensionPermissions {
            network: call_fn(&instance, store, "permissions_network", &[]).unwrap(),
            filesystem: call_fn(&instance, store, "permissions_filesystem", &[]).unwrap(),
            full_settings: call_fn(&instance, store, "permissions_full_settings", &[]).unwrap(),
            extension_settings: call_fn(&instance, store, "permissions_extension_settings", &[])
                .unwrap(),
            extension_hooks: call_fn(&instance, store, "permissions_extension_hooks", &[]).unwrap(),
        };
        Self {
            info,
            permissions,
            instance,
        }
    }
}

#[derive(Default)]
pub struct ExtensionManager {
    extensions: Vec<Extension>,
    store: Store,
}

impl ExtensionManager {
    pub fn load(paths: Vec<PathBuf>) -> Self {
        let compiler = Cranelift::default();
        let mut store = Store::new(compiler);
        let extensions: Vec<Extension> = paths
            .iter()
            .map(|path| Extension::load(&mut store, path))
            .collect();
        Self { extensions, store }
    }

    pub fn frontend_text(&mut self) -> Vec<String> {
        // Over allocate, but it doesn't matter too much
        let mut text = Vec::with_capacity(self.extensions.len());
        for extension in &self.extensions {
            if extension
                .permissions
                .extension_hooks
                .contains(&"frontend_text".to_string())
            {
                let inject_text =
                    call_fn(&extension.instance, &mut self.store, "frontend_text", &[]).unwrap();
                text.push(inject_text);
            }
        }
        text
    }
}
