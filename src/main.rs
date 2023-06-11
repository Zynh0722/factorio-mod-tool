use clap::Parser;

mod default_folder;

use default_folder::get_default_mods_folder;
use enum_as_inner::EnumAsInner;
use semver::Version;
use serde::{Deserialize, Serialize};

use std::{collections::HashMap, fs, path::PathBuf};

// TODO: I need to cull these traits eventually
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct ModFile {
    pub file_name: String,
    pub path: PathBuf,
    pub data: ModFileData,
}

impl ModFile {
    fn get_version(&self) -> Option<&Version> {
        Some(&self.data.as_mod()?.version)
    }

    fn get_mod_name(&self) -> Option<&String> {
        Some(&self.data.as_mod()?.name)
    }
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

trait Returns<T> {
    fn returns(&self, t: T) -> T {
        t
    }
}

impl<T> Returns<T> for () {}

trait Reversed {
    fn reversed(self) -> Self;
}

impl<T> Reversed for Vec<T> {
    #[inline]
    fn reversed(mut self) -> Self {
        self.reverse();
        self
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
            split_name_version
                .clone()
                .map(|s| s.to_owned())
                .skip(1)
                .collect::<Vec<String>>()
                .reversed()
                .join("_")
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
        .filter(|file| file.data.is_mod())
        .cloned()
        .collect();

    let mod_list_file = mods_folder_contents
        .iter()
        .find(|f| f.data.is_mod_list())
        .cloned();

    let mod_list: ModList =
        serde_json::from_slice(&fs::read(mod_list_file.as_ref().unwrap().path.clone()).unwrap())
            .unwrap();
    let mut parsed_mod_list: HashMap<String, bool> = mod_list.into();

    parsed_mod_list.remove_entry("base");

    let mod_settings = mods_folder_contents
        .iter()
        .find(|f| f.data.is_mod_settings())
        .cloned();

    let unknown_files: Vec<ModFile> = mods_folder_contents
        .iter()
        .filter(|f| f.data == ModFileData::Uknown)
        .cloned()
        .collect();

    let mut mod_list_map: HashMap<String, Vec<ModFile>> =
        mods.clone()
            .into_iter()
            .fold(HashMap::new(), |mut acc, mod_file| {
                acc.entry(mod_file.get_mod_name().unwrap().clone())
                    .or_default()
                    .push(mod_file)
                    .returns(acc)
            });

    // Sort mods by version
    for mods in mod_list_map.values_mut() {
        mods.sort_unstable_by(|a, b| {
            let a_v = a.get_version().unwrap();
            let b_v = b.get_version().unwrap();

            a_v.cmp(b_v)
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
    println!("# ModList: {}", mod_list_file.is_some());
    println!("# ModSettings: {}", mod_settings.is_some());
    println!("# Uknowns: {}", unknown_files.iter().count());
}
