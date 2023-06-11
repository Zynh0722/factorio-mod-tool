use clap::Parser;
use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

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

    // TODO: handle this gracefully
    assert!(mods_folder_path.exists());

    let mods_folder = fs::read_dir(mods_folder_path).unwrap();
    let _mods_folder: Vec<DirEntry> = mods_folder.map(|dir| dir.unwrap()).collect();

    let mod_list_path = mods_folder_path.clone().join("mod-list.json");
    let mod_list: ModList = try_parse_mod_list(&mod_list_path).unwrap();

    println!("{:#?}", mod_list)
}

fn try_parse_mod_list(path: &PathBuf) -> std::io::Result<ModList> {
    let contents = fs::read_to_string(path)?;

    Ok(serde_json::from_str(&contents)?)
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
