use clap::Parser;

use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'f', long = "file")]
    /// File path of factorio mods folder
    path: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let mods_folder = args.path.or_else(get_default_mods_folder);

    println!("{mods_folder:?}");
}

fn get_default_mods_folder() -> Option<PathBuf> {
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    return None;

    #[cfg(target_os = "linux")]
    return Some(dirs::home_dir().unwrap().join(".factorio/mods"));

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    let config_path = dirs::config_dir().unwrap();

    #[cfg(target_os = "macos")]
    let factorio_path = config_path.join("factorio");

    #[cfg(target_os = "windows")]
    let factorio_path = config_path.join("Factorio");

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    Some(factorio_path.join("mods"))
}
