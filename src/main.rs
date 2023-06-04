use clap::{Parser, Subcommand};

use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,

    #[arg(short = 'f', long = "file")]
    /// File path of factorio mods folder
    path: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
enum Command {
    Extract {
        /// Fetch both enabled and disabled mods
        ///
        /// By default extract only grabs enabled mods
        #[arg(short, long, default_value_t = true)]
        all: bool,

        /// Fetch the json mod list used by factorio
        ///
        /// This effectively just cats the mod-list.json file
        /// from the mods folder
        ///
        /// This ignores --all as the mod list contains the
        /// enabled metadata within it
        #[arg(short, long, default_value_t = false)]
        list: bool,

        /// Set the target file for the extracted mods or mod list
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
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
    return Ok(dirs::home_dir().unwrap().join(".factorio/mods"));

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    let config_path = dirs::config_dir().unwrap();

    #[cfg(target_os = "macos")]
    let factorio_path = config_path.join("factorio");

    #[cfg(target_os = "windows")]
    let factorio_path = config_path.join("Factorio");

    return Some(factorio_path.join("mods"));
}
