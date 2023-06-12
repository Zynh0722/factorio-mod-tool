use clap::Parser;
use std::fs::{self, DirEntry};

mod args;
mod init;
mod mod_list;

use args::Args;
use init::get_default_mods_folder;

use mod_list::try_parse_mod_list_map;

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
