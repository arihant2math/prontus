
use extension_api as prontus;
use prontus::Extension;

struct ExampleTaskExtension {}

impl Extension for ExampleTaskExtension {
    fn new() -> Self
    where
        Self: Sized
    {
        ExampleTaskExtension {}
    }
}

prontus::register_extension!(ExampleTaskExtension);
