use clap::Parser;

mod default_folder;

use default_folder::get_default_mods_folder;
use enum_as_inner::EnumAsInner;
use semver::Version;
use serde::{Deserialize, Serialize};

use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct ModFile {
    #[allow(dead_code)]
    pub file_name: String,
    pub path: PathBuf,
    pub data: ModFileData,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModListEntry {
    name: String,
    enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModList {
    mods: Vec<ModListEntry>,
}

impl Into<HashMap<String, bool>> for ModList {
    fn into(self) -> HashMap<String, bool> {
        let mut map = HashMap::new();

        for entry in self.mods {
            map.insert(entry.name, entry.enabled);
        }

        map
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, EnumAsInner)]
enum ModFileData {
    ModList,
    ModSettings,
    Mod(ModData),
    Uknown,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct ModData {
    name: String,
    version: Version,
    enabled: bool,
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
                    ModFileData::Mod(ModData {
                        name,
                        enabled: false,
                        version,
                    })
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

    parse_and_print_mods_folder(args);
}

fn parse_and_print_mods_folder(args: Args) {
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

    let mut parsed_mod_list: HashMap<String, bool> =
        serde_json::from_slice::<ModList>(&fs::read(mod_list.unwrap().path.clone()).unwrap())
            .unwrap()
            .into();

    parsed_mod_list.remove_entry("base");

    let mod_settings = mods_folder_contents
        .iter()
        .find(|f| f.data == ModFileData::ModSettings)
        .clone();

    let unknown_files: Vec<ModFile> = mods_folder_contents
        .iter()
        .filter(|f| f.data == ModFileData::Uknown)
        .cloned()
        .collect();

    let mut mod_list_map: HashMap<String, Vec<ModFile>> =
        mods.clone()
            .into_iter()
            .fold(HashMap::new(), |mut acc, mod_file| {
                acc.entry(mod_file.data.as_mod().unwrap().name.clone())
                    .or_default()
                    .push(mod_file);
                acc
            });

    for mods in mod_list_map.values_mut() {
        mods.sort_unstable_by(|a, b| {
            a.data
                .as_mod()
                .unwrap()
                .version
                .partial_cmp(&b.data.as_mod().unwrap().version)
                .unwrap()
        });
    }

    let mut sorted_mods: Vec<(&String, &bool)> = parsed_mod_list.iter().collect();
    sorted_mods.sort_unstable_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    for (mod_name, enabled) in sorted_mods.iter() {
        println!(
            " - [{}] {:40}latest {:8} - {} version(s)",
            if **enabled { "X" } else { " " },
            mod_name,
            // This is only latest because I only have one version
            mod_list_map.get(*mod_name).unwrap()[0]
                .data
                .as_mod()
                .unwrap()
                .version,
            mod_list_map.get(*mod_name).unwrap().iter().count(),
        );
    }

    println!("# Mods: {}", mods.iter().count());
    println!(
        "# Files - Mods = {} # should prolly be two",
        mods_folder_contents.iter().count() - mods.iter().count()
    );
    println!("# ModList: {}", mod_list.is_some());
    println!("# ModSettings: {}", mod_settings.is_some());
    println!("# Uknowns: {}", unknown_files.iter().count());
}
