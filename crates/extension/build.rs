use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    copy_extension_api_rust_files()
}

/// rust-analyzer doesn't support include! for files from outside the crate.
/// Copy them to the OUT_DIR, so we can include them from there, which is supported.
fn copy_extension_api_rust_files() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let input_dir = PathBuf::from("../extension-api/wit");
    let output_dir = PathBuf::from(out_dir);

    println!("cargo:rerun-if-changed={}", input_dir.display());

    for entry in fs::read_dir(&input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            println!("cargo:rerun-if-changed={}", path.display());

            for subentry in fs::read_dir(&path)? {
                let subentry = subentry?;
                let subpath = subentry.path();
                if subpath.extension() == Some(std::ffi::OsStr::new("rs")) {
                    let relative_path = subpath.strip_prefix(&input_dir)?;
                    let destination = output_dir.join(relative_path);

                    fs::create_dir_all(destination.parent().unwrap())?;
                    fs::copy(&subpath, &destination)?;
                }
            }
        } else if path.extension() == Some(std::ffi::OsStr::new("rs")) {
            let relative_path = path.strip_prefix(&input_dir)?;
            let destination = output_dir.join(relative_path);

            fs::create_dir_all(destination.parent().unwrap())?;
            fs::copy(&path, &destination)?;
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    Ok(())
}
