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

use wasmer::{imports, wat2wasm, Cranelift, Instance, Module, Store, Value};

fn load(bytes: Vec<u8>) -> Result<Instance, Box<dyn std::error::Error>> {
    // Let's declare the Wasm module with the text representation.
    let wasm_bytes = wat2wasm(&bytes)?;

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

    // let sum = instance.exports.get_function("sum")?;
    //
    // println!("Calling `sum` function...");
    // // Let's call the `sum` exported function. The parameters are a
    // // slice of `Value`s. The results are a boxed slice of `Value`s.
    // let results = sum.call(&mut store, &[Value::I32(1), Value::I32(2)])?;
    //
    // println!("Results: {:?}", results);
    // assert_eq!(results.to_vec(), vec![Value::I32(3)]);

    Ok(instance)
}

pub struct ExtensionInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String
}

pub struct ExtensionPermissions {
    pub network: bool,
    pub filesystem: bool,
    pub full_settings: bool,
    pub extension_settings: bool,
}

pub struct Extension {
    pub info: ExtensionInfo,
    pub permissions: ExtensionPermissions,
    pub instance: Instance,
}

pub struct ExtensionManager {
    bots: Vec<Extension>,
    frontend_injectors: Vec<Extension>,
    tasks: Vec<Extension>,
}
