//! https://github.com/LaughingLeader/BG3ModManager/blob/master/GUI/Resources/DefaultPathways.json

#[derive(Debug, Clone)]
pub struct StoreInfo {
    pub app_id: &'static str,
    pub registry_32: &'static str,
    pub registry_64: &'static str,
    pub root_folder_name: &'static str,
    pub exe_path: &'static str,
}

/*
{
    "Steam": {
        "AppID": "1086940",
        "Registry_32": "SOFTWARE\\Valve\\Steam\\Apps\\1086940",
        "Registry_64": "SOFTWARE\\Wow6432Node\\Valve\\Steam\\Apps\\1086940",
        "RootFolderName": "Baldurs Gate 3",
        "ExePath": "bin\\bg3.exe"
    },
    "GOG": {
        "AppID": "360014030297",
        "Registry_32": "SOFTWARE\\GOG.com\\Games\\360014030297",
        "Registry_64": "SOFTWARE\\Wow6432Node\\GOG.com\\Games\\360014030297",
        "RootFolderName": "Baldurs Gate 3",
        "ExePath": "bin\\bg3.exe"
    },
    "DocumentsGameFolder": "Larian Studios\\Baldur's Gate 3",
    "AppDataGameFolder": "%LOCALAPPDATA%\\Larian Studios\\Baldur's Gate 3",
    "GameDataFolder": "Data"
}
*/

pub const STEAM_INFO: StoreInfo = StoreInfo {
    app_id: "1086940",
    registry_32: "SOFTWARE\\Valve\\Steam\\Apps\\1086940",
    registry_64: "SOFTWARE\\Wow6432Node\\Valve\\Steam\\Apps\\1086940",
    root_folder_name: "Baldurs Gate 3",
    exe_path: "bin/bg3.exe",
};

pub const GOG_INFO: StoreInfo = StoreInfo {
    app_id: "360014030297",
    registry_32: "SOFTWARE\\GOG.com\\Games\\360014030297",
    registry_64: "SOFTWARE\\Wow6432Node\\GOG.com\\Games\\360014030297",
    root_folder_name: "Baldurs Gate 3",
    exe_path: "bin/bg3.exe",
};

pub const DOCUMENTS_GAME_FOLDER: &str = "Larian Studios\\Baldur's Gate 3";
/// Windows  
/// Implicitly should start with `%LOCALAPPDATA%`. Removed the beginning because I don't believe it
/// is automatically expanded in Rust.
pub const APP_DATA_GAME_FOLDER: &str = "Larian Studios\\Baldur's Gate 3";
// pub const APP_DATA_GAME_FOLDER: &str = "%LOCALAPPDATA%\\Larian Studios\\Baldur's Gate 3";
// TODO: Linux + MacOS default data game folder

pub const GAME_DATA_FOLDER: &str = "Data";
