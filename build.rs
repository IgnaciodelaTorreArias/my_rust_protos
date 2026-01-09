use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["protos/module.proto"], &["protos/"])?;
    #[cfg(target_os = "windows")]
    {
        let mut res = tauri_winres::WindowsResourse::new();
        res.compile()?;
    }
    Ok(())
}
