use clap::Parser;
use std::fs;

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

    for file in fs::read_dir(mods_folder).unwrap() {
        println!("{}", file.unwrap().path().display());
    }
}
