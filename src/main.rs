use clap::Parser;

use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File path of factorio mods folder
    #[arg(short = 'f', long = "file")]
    path: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let mods_folder = args
        .path
        .or_else(get_default_mods_folder)
        .expect("Please include a file path. --help for more information");

    println!("{mods_folder:?}");
}

fn get_default_mods_folder() -> Option<PathBuf> {
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    return None;

    #[cfg(target_os = "linux")]
    return Ok(dirs::home_dir().unwrap().join(".factorio/mods"));

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    let config_path = dirs::config_dir().unwrap();

    #[cfg(target_os = "macos")]
    let factorio_path = config_path.join("factorio");

    #[cfg(target_os = "windows")]
    let factorio_path = config_path.join("Factorio");

    Some(factorio_path.join("mods"))
}
