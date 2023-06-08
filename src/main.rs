use clap::Parser;

mod default_folder;

use default_folder::get_default_mods_folder;

use std::{fs, path::PathBuf};

#[derive(Debug)]
struct ModFile {
    pub file_name: String,
    pub path: PathBuf,
}

impl ModFile {
    fn new(p: PathBuf) -> Self {
        Self {
            path: p.clone(),
            file_name: p.file_name().unwrap().to_str().unwrap().to_string(),
        }
    }
}

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

    let mods_folder = fs::read_dir(mods_folder).unwrap().flatten();

    let mods = mods_folder.map(|p| ModFile::new(p.path()));

    for module in mods {
        println!("{:?}\n\t{:?}\n", module.file_name, module.path);
    }
}
