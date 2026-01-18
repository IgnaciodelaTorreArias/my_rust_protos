#[cfg(debug_assertions)]
extern crate cbindgen;
#[cfg(debug_assertions)]
use std::env;
use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["protos/module.proto"], &["protos/"])?;
    #[cfg(target_os = "windows")]
    {
        let mut res = tauri_winres::WindowsResource::new();
        res.compile()?;
    }
    #[cfg(debug_assertions)]
    {
        let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        cbindgen::generate(crate_dir)
            .expect("Unable to generate bindings")
            .write_to_file("my_rust_protos.h");
    }
    
    Ok(())
}
