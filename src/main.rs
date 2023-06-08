use clap::Parser;

mod default_folder;

use default_folder::get_default_mods_folder;
use enum_as_inner::EnumAsInner;
use semver::Version;

use std::{fs, iter::Cloned, path::PathBuf};

#[derive(Debug, Clone)]
struct ModFile {
    pub file_name: String,
    pub path: PathBuf,
    pub data: ModFileData,
}

#[derive(Debug, PartialEq, Clone, EnumAsInner)]
enum ModFileData {
    ModList,
    ModSettings,
    Mod(ModData),
    #[allow(dead_code)]
    Uknown,
}

#[derive(PartialEq, Debug, Clone)]
struct ModData {
    name: String,
    version: Version,
}

impl ModFile {
    fn new(p: &PathBuf) -> Self {
        let file_name = p.file_name().unwrap().to_str().unwrap().to_string();
        let split_name_version = file_name.rsplit("_");

        let name = if split_name_version.clone().count() > 2 {
            let mut name: Vec<String> = split_name_version
                .clone()
                .map(|s| s.to_owned())
                .skip(1)
                .collect();

            name.reverse();

            name.join("_")
        } else {
            split_name_version.clone().last().unwrap().to_owned()
        };

        let data = match name.as_str() {
            "mod-list.json" => ModFileData::ModList,
            "mod-settings.dat" => ModFileData::ModSettings,
            _ => {
                let version = split_name_version
                    .clone()
                    .next()
                    .unwrap()
                    .replace(".zip", "");

                let version = Version::parse(&version).ok();

                if let Some(version) = version {
                    ModFileData::Mod(ModData { name, version })
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

    let mod_list = mods_folder_contents
        .iter()
        .find(|f| f.data == ModFileData::ModList)
        .clone();

    let mod_settings = mods_folder_contents
        .iter()
        .find(|f| f.data == ModFileData::ModSettings)
        .clone();

    let unknown_files: Vec<ModFile> = mods_folder_contents
        .iter()
        .filter(|f| f.data == ModFileData::Uknown)
        .cloned()
        .collect();

    mods.iter()
        .for_each(|f| println!("{:#?}", f.data.as_mod().unwrap()));

    println!("# Mods: {}", mods.iter().count());
    println!(
        "# Files - Mods = {} # should prolly be two",
        mods_folder_contents.iter().count() - mods.iter().count()
    );
    println!("# ModList: {}", mod_list.is_some());
    println!("# ModSettings: {}", mod_settings.is_some());
    println!("# Uknowns: {}", unknown_files.iter().count());
}
