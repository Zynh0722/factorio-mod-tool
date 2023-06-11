use clap::Parser;

mod init;
mod args;

use init::get_default_mods_folder;
use args::Args;

// this is a comment

fn main() {
    let args = Args::parse();

    let mods_folder = args.path.or_else(get_default_mods_folder);

    if mods_folder.is_none() {
        println!("Unable to find mods folder. (see --help for more info)");
        return;
    }

    let mods_folder = mods_folder.unwrap();

    let mods_folder_exists = mods_folder.exists();

    println!("{mods_folder:?}");
    println!("exists? {mods_folder_exists:?}");
}
