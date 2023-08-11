use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    // TODO: give the paths decent default values on the common platforms.
    /// Path to the game's data folder.  
    /// Ex: "C:/Steam/steamapps/common/Baldurs Gate 3/Data"
    pub game_data_path: PathBuf,
    /// Path to the game's executable.  
    /// Ex: "C:/Steam/steamapps/common/Baldurs Gate 3/bin/bg3.exe"
    pub game_executable_path: PathBuf,
    /// Folder name where load orders should be saved.
    pub saved_load_orders_path: PathBuf,
    // TODO: always disable telemetry setting? Is that for BG3MM or for BG3?
    pub enable_story_log: bool,
    pub auto_add_missing_dependencies_on_export: bool,
    // TODO: auto updates
    // pub enable_automatic_updates: bool,
    pub show_missing_mod_warnings: bool,
    pub shift_focus_on_swap: bool,
    pub save_window_location: bool,
    // TODO
    // pub on_game_launch
    pub enable_dx11_mode: bool,
    pub skip_launcher: bool,
    // TODO: linux specific settings? Like proton version to use?
    pub launch: LaunchConfig,
}
impl Config {
    /// Returns "" if the path is None
    pub fn game_data_path_str(&self) -> &str {
        self.game_data_path.as_os_str().to_str().unwrap_or_default()
    }

    /// Returns "" if the path is None
    pub fn game_executable_path_str(&self) -> &str {
        self.game_executable_path
            .as_os_str()
            .to_str()
            .unwrap_or_default()
    }

    pub fn saved_load_orders_path_str(&self) -> &str {
        self.saved_load_orders_path
            .as_os_str()
            .to_str()
            .unwrap_or_default()
    }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            // TODO: check the actual defaults
            game_data_path: PathBuf::new(),
            game_executable_path: PathBuf::new(),
            saved_load_orders_path: PathBuf::from("Orders/"),
            enable_story_log: false,
            auto_add_missing_dependencies_on_export: true,
            show_missing_mod_warnings: true,
            shift_focus_on_swap: false,
            save_window_location: true,
            enable_dx11_mode: false,
            skip_launcher: true,
            launch: LaunchConfig::default(),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LaunchConfig {
    /// Automatically load the last save when loading into the main menu.
    pub continue_game: bool,
    /// Enables the story log
    pub story_log: bool,
    /// A directory to write story logs to
    pub log_path: Option<PathBuf>,
    /// Limit the cpu to x amount of threads (unknown if this works)
    pub cpu_limit: Option<usize>,
    pub asserts: bool,
    pub stats: bool,
    pub dynamic_story: bool,
    // TODO: give these their actual types
    pub external_crash_handler: bool,
    pub name_tag: bool,
    pub module: bool,
    pub connect_lobby: bool,
    pub loca_updater: bool,
    pub media_path: bool,
}

// TODO: keybindings
