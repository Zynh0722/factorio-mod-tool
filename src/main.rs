use clap::Parser;
use std::{
    collections::HashMap,
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
    let mod_list_map = try_parse_mod_list_map(&mod_list_path).unwrap();

    for (name, enabled) in mod_list_map.iter() {
        println!("[{}] {name}", if *enabled { "X" } else { " " });
    }
}

fn try_parse_mod_list_map(path: &PathBuf) -> std::io::Result<ModListMap> {
    Ok(try_parse_mod_list(path)?.into())
}

fn try_parse_mod_list(path: &PathBuf) -> std::io::Result<ModList> {
    let contents = fs::read_to_string(path)?;

    Ok(serde_json::from_str(&contents)?)
}

type ModListMap = HashMap<String, bool>;

impl Into<ModListMap> for ModList {
    fn into(self) -> HashMap<String, bool> {
        let mut new = HashMap::new();

        for entry in self.mods {
            new.insert(entry.name, entry.enabled);
        }

        new
    }
}

impl Into<ModList> for ModListMap {
    fn into(self) -> ModList {
        let mut mods = Vec::new();

        for (name, enabled) in self.into_iter() {
            mods.push(ModListEntry { name, enabled })
        }

        ModList { mods }
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
