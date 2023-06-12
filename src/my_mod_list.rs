use std::{collections::HashMap, path::PathBuf};
use serde::{Serialize, Deserialize};

pub type MyModListMap = HashMap<String, MyModListEntry>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MyModList {
    pub mods: Vec<MyModListEntry>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyModListEntry {
    pub name: String,
    pub version: String,
    pub enabled: bool,
}

impl Into<MyModListMap> for MyModList {
    fn into(self) -> MyModListMap {
        let mut new = HashMap::new();

        for entry in self.mods {
            new.insert(entry.name, MyModListEntry { name: entry.name, version: entry.version, enabled: entry.enabled });
        }

        new
    }
}

impl Into<MyModList> for MyModListMap {
    fn into(self) -> MyModList {
        let mut mods = Vec::new();

        for (name, data) in self.into_iter() {
            
        }

        MyModList { mods }
    }
}

// Parse my mod list
pub fn try_parse_my_mod_list(path: &PathBuf) -> std::io::Result<MyModListMap> {
    let contents = std::fs::read_to_string(path)?;

    Ok(make_my_mod_list(contents))
}


fn make_my_mod_list(contents: String) -> HashMap<String, MyModListEntry> {
    let modlist: MyModList = serde_json::from_str(&contents).unwrap();

    let mut mods = HashMap::new();

    for (name, version, enabled) in modlist.into_iter() {
        mods.insert(name, MyModListEntry{name: name, version: version, enabled: enabled,});
    };

    mods
}


