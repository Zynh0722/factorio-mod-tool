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

    let mods_folder_path = mods_folder.as_ref().unwrap();

    println!("{mods_folder:?}");
    println!("exists? {:?}", mods_folder_path.exists());

    let mods_folder = fs::read_dir(mods_folder_path).unwrap();
    let mods_folder: Vec<DirEntry> = mods_folder.map(|dir| dir.unwrap()).collect();

    let contents = fs::read_to_string(mods_folder_path.join("mod-list.json"))
        .expect("Should have been able to read the file");

    // println!("With text:\n{contents}");
    let mod_list: ModList = serde_json::from_str(&contents).expect(r"This didn't work ¯\_(ツ)_/¯");

    println!("{:#?}", mod_list)
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
