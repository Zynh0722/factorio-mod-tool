pub fn get_default_mods_folder() -> Option<PathBuf> {
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    return None;

    #[cfg(target_os = "linux")]
    return Some(dirs::home_dir().unwrap().join(".factorio").join("mods"));

    #[cfg(target_os = "macos")]
    return Some(dirs::config_dir().unwrap().join("factorio").join("mods"));

    #[cfg(target_os = "windows")]
    return Some(dirs::config_dir().unwrap().join("Factorio").join("mods"));
}
