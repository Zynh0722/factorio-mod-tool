use clap::Parser;

use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'f', long = "file")]
    /// File path of factorio mods folder
    pub path: Option<PathBuf>,
}
