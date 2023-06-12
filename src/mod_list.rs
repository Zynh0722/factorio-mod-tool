use std::{collections::HashMap, path::PathBuf};

pub fn try_parse_mod_list_map(path: &PathBuf) -> std::io::Result<ModListMap> {
    Ok(try_parse_mod_list(path)?.into())
}

pub fn try_parse_mod_list(path: &PathBuf) -> std::io::Result<ModList> {
    let contents = std::fs::read_to_string(path)?;

    Ok(serde_json::from_str(&contents)?)
}

pub type ModListMap = HashMap<String, bool>;

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
pub struct ModList {
    pub mods: Vec<ModListEntry>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ModListEntry {
    pub name: String,
    pub enabled: bool,
}
