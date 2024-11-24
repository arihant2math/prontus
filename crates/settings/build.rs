fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    wit_gen::WitGenerator {
        files: vec!["src/lib.rs".into()],
        interface_name: "settings".into(),
        output: format!("{}/settings.wit", out_dir).into(),
    }.run();
}
