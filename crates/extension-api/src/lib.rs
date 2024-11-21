mod wit {
    #![allow(clippy::too_many_arguments, clippy::missing_safety_doc)]

    wit_bindgen::generate!({
        skip: [],
        path: "./wit/since_v0.1.0",
        world: "extension"
    });
}

use wit::*;

pub trait Extension: Send + Sync {
    /// Returns a new instance of the extension.
    fn new() -> Self
    where
        Self: Sized;

    fn init_extension(&mut self) {}
    fn run_task(&mut self) {}
    fn shutdown_extension(&mut self) {}
}

/// Registers the provided type as a Zed extension.
///
/// The type must implement the [`Extension`] trait.
#[macro_export]
macro_rules! register_extension {
    ($extension_type:ty) => {
        #[export_name = "init-extension"]
        pub extern "C" fn __init_extension() {
            std::env::set_current_dir(std::env::var("PWD").unwrap()).unwrap();
            zed_extension_api::register_extension(|| {
                Box::new(<$extension_type as zed_extension_api::Extension>::new())
            });
        }
    };
}

#[doc(hidden)]
pub fn register_extension(build_extension: fn() -> Box<dyn Extension>) {
    unsafe { EXTENSION = Some(build_extension()) }
}

fn extension() -> &'static mut dyn Extension {
    unsafe { EXTENSION.as_deref_mut().unwrap() }
}

static mut EXTENSION: Option<Box<dyn Extension>> = None;

#[cfg(target_arch = "wasm32")]
#[link_section = "zed:api-version"]
#[doc(hidden)]
pub static ZED_API_VERSION: [u8; 6] = *include_bytes!(concat!(env!("OUT_DIR"), "/version_bytes"));

wit::export!(Component);

struct Component;

impl wit::Guest for Component {
    fn init_extension() {
        extension().init_extension();
    }

    fn run_task() {
        extension().run_task();
    }

    fn shutdown_extension() {
        extension().shutdown_extension();
    }
}