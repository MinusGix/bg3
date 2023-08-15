//! https://github.com/LaughingLeader/BG3ModManager/blob/ddc34d17ebf20f0d0fef720fc68fd00ba1a33f8d/DivinityModManagerCore/Util/DivinityRegistryHelper.cs  
//!   
//! We use steamlocate rather than reimplementing all of the logic for finding the path ourselves.  
//! TODO: is there a similar GOG version?

use std::path::PathBuf;

use steamlocate::SteamDir;

pub const PATH_STEAM_WORKSHOP_FOLDER: &str = "steamapps/workshop";

static STEAM_INSTALL_DIR: std::sync::OnceLock<Option<SteamDir>> = std::sync::OnceLock::new();

pub fn get_steam_install_dir() -> Option<SteamDir> {
    STEAM_INSTALL_DIR.get_or_init(|| SteamDir::locate()).clone()
}

pub fn get_steam_workshop_path() -> Option<PathBuf> {
    let steam_install_path = get_steam_install_dir()?.path;
    let workshop_folder = steam_install_path.join(PATH_STEAM_WORKSHOP_FOLDER);
    eprintln!("Looking for workshop folder at {workshop_folder:?}");

    if workshop_folder.is_dir() {
        Some(workshop_folder)
    } else {
        None
    }
}

// TODO: there's also a get_workshop_path?

pub fn get_gog_install_path(reg_gog_32: &str, reg_gog_64: &str) -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        use winreg::reg_key::RegKey;
        let reg = RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
        let install_path = reg
            .open_subkey(reg_gog_64)
            .or_else(|_| reg.open_subkey(reg_gog_32))
            .and_then(|key| key.get_value::<String, _>("path").ok())?;

        let install_path = PathBuf::from(install_path);
        eprintln!("Found gog install path at {install_path:?}");
        return Some(install_path);
    }

    // TODO: Linux locations
    // TODO: MacOs locations

    None
}

static LAST_GAME_PATH: std::sync::OnceLock<Option<PathBuf>> = std::sync::OnceLock::new();
static IS_GOG: std::sync::OnceLock<bool> = std::sync::OnceLock::new();

pub fn is_gog() -> bool {
    IS_GOG.get().copied().unwrap_or(false)
}

/// Unlike the C# version this just takes the game id rather than the path for it.
pub fn get_game_install_path(
    gog_reg_32: &str,
    gog_reg_64: &str,
    steam_game_id: u32,
) -> Option<PathBuf> {
    if let Some(mut steam_dir) = get_steam_install_dir() {
        // Note: we aren't really checking whether the directory exists. It should do that when it
        // is set. Also if we wanted to check that everytime this is called, we'd have to use
        // something other than oncelock.
        if LAST_GAME_PATH.get().is_some() {
            return LAST_GAME_PATH.get().unwrap().clone();
        }

        println!("Looking for game with app id: {steam_game_id}");
        if let Some(folder) = steam_dir.app(&steam_game_id) {
            let folder = folder.path.clone();
            eprintln!("Found game at {folder:?}");

            // We ignore nay failures to set it since it should be fine.
            let _ = IS_GOG.set(false);

            let _ = LAST_GAME_PATH.set(Some(folder));
            return LAST_GAME_PATH.get().unwrap().clone();
        }
        // We don't need to manually search the other library folders because steamlocate
        // handles that for us.
    }

    // Since we failed to find the steam folder, we now try to find the gog folder.

    let gog_game_path = get_gog_install_path(gog_reg_32, gog_reg_64)?;
    if gog_game_path.is_dir() {
        let _ = IS_GOG.set(true);
        eprintln!("Found game (GoG) install at {gog_game_path:?}");
        let _ = LAST_GAME_PATH.set(Some(gog_game_path));
        return LAST_GAME_PATH.get().unwrap().clone();
    }

    None
}
