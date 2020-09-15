use std::path::PathBuf;
use std::collections::HashMap;

use fehler::{throws, throw};
use serde::Deserialize;

use crate::Error;

#[cfg(target_os = "windows")]
#[throws]
pub fn get_library_folders() -> PathBuf {
    let registry_key = registry::Hive::CurrentUser.open(r"Software\Valve\Steam", registry::Security::Read)
        .map_err(|err| Error::ConfigInstallError { description: "could not find Steam install path in Windows registry", cause: Some(Box::new(err)) })?;
    let steam_path = registry_key.value("SteamPath")
        .map_err(|err| Error::ConfigInstallError { description: "could not find Steam install path in Windows registry", cause: Some(Box::new(err)) })?;
    if let registry::Data::String(steam_path) = steam_path {
        let steam_path = steam_path.to_os_string();
        let mut steam_path = PathBuf::from(steam_path);
        steam_path.extend(&["steamapps", "libraryfolders.vdf"]);
        steam_path
    } else {
        throw!(Error::ConfigInstallError {
            description: "could not find Steam install path in Windows registry, had a weird type",
            cause: None,
        });
    }
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
#[throws]
pub fn get_library_folders() -> PathBuf {
    use std::env;
    let mut path = PathBuf::from(env::var("HOME").expect("could not find $HOME folder"));
    #[cfg(target_os = "macos")] path.extend(&["Library", "Application Support", "Steam"]);
    #[cfg(target_os = "linux")] path.extend(&[".local", "share", "Steam"]);
    path.extend(&["steamapps", "libraryfolders.vdf"]);
    path
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))] compile_error!("unsupported OS");

#[derive(Deserialize)]
struct LibraryFolders(HashMap<String, String>);

#[throws]
pub fn discover_cfg_folder() -> PathBuf {
    use std::iter;
    use std::fs::read_to_string;
    let library_folders_file = get_library_folders()?;
    let library_folders_data = read_to_string(&library_folders_file)
        .map_err(|err| Error::ConfigInstallError { description: "could not read libraryfolders.vdf file", cause: Some(Box::new(err)) })?;
    let LibraryFolders(library_folders_data) = vdf_serde::from_str(&library_folders_data)
        .map_err(|err| Error::ConfigInstallError { description: "could not parse libraryfolders.vdf file", cause: Some(Box::new(err)) })?;
    let mut library_folders = {
        let mut default_library_folder = library_folders_file;
        default_library_folder.pop();
        default_library_folder.pop();
        // warning: the following code is too clever for its own good
        iter::once(default_library_folder).chain((1usize..)
            .map(|i| i.to_string())
            .take_while(|i| library_folders_data.contains_key(i))
            .map(|i| &library_folders_data[&i])
            .map(PathBuf::from))
    };
    library_folders.find_map(|mut library| {
        library.extend(&["steamapps", "common", "Counter-Strike Global Offensive"]);
        if library.exists() {
            library.extend(&["csgo", "cfg"]);
            Some(library)
        } else {
            None
        }
    }).ok_or(Error::ConfigInstallError { description: "could not find CS:GO install directory", cause: None })?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_attributes)]
    #[test]
    #[ignore = "extremely dependent on my specific setup"]
    #[throws]
    fn test_discover_cfg_folder() {
        let cfg_folder = discover_cfg_folder()?;
        assert_eq!(cfg_folder, PathBuf::from(r"D:\SteamLibrary\steamapps\common\Counter-Strike Global Offensive\csgo\cfg"));
    }
}
