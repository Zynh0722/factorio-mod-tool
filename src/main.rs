use clap::Parser;
use std::fs::{self, DirEntry};

mod args;
mod init;

use args::Args;
use init::get_default_mods_folder;

fn main() {
    let args = Args::parse();

    let mods_folder = args.path.or_else(get_default_mods_folder);

    if mods_folder.is_none() {
        println!("Unable to find mods folder. (see --help for more info)");
        return;
    }

    let mods_folder = mods_folder.unwrap();

    println!("{mods_folder:?}");
    println!("exists? {:?}", mods_folder.exists());

    let mods_folder = fs::read_dir(mods_folder).unwrap();
    let mods_folder: Vec<DirEntry> = mods_folder.map(|dir| dir.unwrap()).collect();

    for mod_f in mods_folder.iter() {
        println!("{:#?}", mod_f.file_name());
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ModList {
    mods: Vec<ModListEntry>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ModListEntry {
    name: String,
    enabled: bool,
}
