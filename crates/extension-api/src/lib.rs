mod wit {
    #![allow(clippy::too_many_arguments, clippy::missing_safety_doc)]

    wit_bindgen::generate!({
        skip: ["init-extension"],
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

    fn run_task(&mut self) {}
    fn shutdown_extension(&mut self) {}
}

/// Registers the provided type as a Prontus extension.
///
/// The type must implement the [`Extension`] trait.
#[macro_export]
macro_rules! register_extension {
    ($extension_type:ty) => {
        #[export_name = "init-extension"]
        pub extern "C" fn __init_extension() {
            extension_api::register_extension_fn(|| {
                Box::new(<$extension_type as extension_api::Extension>::new())
            });
        }
    };
}

#[doc(hidden)]
pub fn register_extension_fn(build_extension: fn() -> Box<dyn Extension>) {
    unsafe { EXTENSION = Some(build_extension()) }
}

fn extension() -> &'static mut dyn Extension {
    unsafe { EXTENSION.as_deref_mut().unwrap() }
}

// TODO: static mut is deprecated
static mut EXTENSION: Option<Box<dyn Extension>> = None;

#[cfg(target_arch = "wasm32")]
#[link_section = "prontus:api-version"]
#[doc(hidden)]
pub static PRONTUS_API_VERSION: [u8; 6] =
    *include_bytes!(concat!(env!("OUT_DIR"), "/version_bytes"));

export!(Component);

struct Component;

impl Guest for Component {
    fn run_task() {
        extension().run_task();
    }

    fn shutdown_extension() {
        extension().shutdown_extension();
    }
}
