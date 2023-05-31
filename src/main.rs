use clap::Parser;

use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    #[arg(short = 'f', long = "file")]
    /// File path of factorio mods folder
    path: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let mods_folder = args
        .path
        .unwrap_or_else(|| get_default_mods_folder().unwrap());

    println!("{mods_folder:?}");
}

#[derive(Debug, Clone)]
struct NoDefaultPathError {}

fn get_default_mods_folder() -> Result<PathBuf, NoDefaultPathError> {
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    return NoDefaultPathError;

    #[cfg(target_os = "linux")]
    return Ok(dirs::home_dir().unwrap().join(".factorio/mods"));

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    let config_path = dirs::config_dir().unwrap();

    #[cfg(target_os = "macos")]
    return Ok(config_path.join("factorio/mods"));

    #[cfg(target_os = "windows")]
    return Ok(config_path.join("Factorio/mods"));
}
