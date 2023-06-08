use clap::Parser;

mod default_folder;

use default_folder::get_default_mods_folder;

use std::{fs, path::PathBuf};

#[derive(Debug, Clone)]
struct ModFile {
    pub file_name: String,
    pub path: PathBuf,
    pub data: ModFileData,
}

#[derive(Debug, PartialEq, Clone)]
enum ModFileData {
    ModList,
    ModSettings,
    Mod(String),
    #[allow(dead_code)]
    Uknown,
}

impl ModFile {
    fn new(p: &PathBuf) -> Self {
        let file_name = p.file_name().unwrap().to_str().unwrap().to_string();
        let split_name_version = file_name.clone();
        let mut split_name_version = split_name_version.split("_");
        let name = split_name_version.next().unwrap();

        let data = match name {
            "mod-list.json" => ModFileData::ModList,
            "mod-settings.dat" => ModFileData::ModSettings,
            name => {
                if let Some(mod_name) = name.split("_").next() {
                    ModFileData::Mod(mod_name.to_owned())
                } else {
                    ModFileData::Uknown
                }
            }
        };

        Self {
            path: p.clone(),
            file_name,
            data,
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

    let mods_folder_contents: Vec<ModFile> = mods_folder.map(|p| ModFile::new(&p.path())).collect();

    let mods: Vec<ModFile> = mods_folder_contents
        .iter()
        .filter(|file| matches!(file.data, ModFileData::Mod(_)))
        .cloned()
        .collect();

    let _mod_list = mods_folder_contents
        .iter()
        .find(|f| f.data == ModFileData::ModList)
        .unwrap()
        .clone();

    let _mod_settings = mods_folder_contents
        .iter()
        .find(|f| f.data == ModFileData::ModSettings)
        .unwrap()
        .clone();

    let unknown_files: Vec<ModFile> = mods_folder_contents
        .iter()
        .filter(|f| f.data == ModFileData::Uknown)
        .cloned()
        .collect();

    println!("# Mods: {}", mods.iter().count());
    println!(
        "# Files - Mods = {} # should be two",
        mods_folder_contents.iter().count() - mods.iter().count()
    );
    println!("# ModList: Detected");
    println!("# ModSettings: Detected");
    println!("# Uknowns: {}", unknown_files.iter().count());
}
