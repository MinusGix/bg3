use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

// TODO: rename everything to Settings to match.
// TODO: ideally keep all field names the same so that we can just directly load/save the same as C# BG3MM.

/// Configuration items.  
/// If you add a configuration item, then there is several places that need to be modified.  
/// - The default implementation below.
/// - `mod-mgr`
///   - `settings_view.rs` has `save_config` and various structures for holding signals for the
///     configuration items that are changeable in the manager.
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
    /// Path to the workshop folder. Currently unused.
    pub workshop_path: PathBuf,
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
    pub fn load() -> std::io::Result<Config> {
        todo!()
    }

    pub fn save(&self) -> std::io::Result<()> {
        todo!()
    }

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
            workshop_path: PathBuf::new(),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScriptExtenderSettings {
    /// Make the Osiris extension functionality available ingame or in the editor
    pub enable_extensions: bool,
    pub create_console: bool,
    pub log_failed_compile: bool,
    pub enable_logging: bool,
    pub log_compile: bool,
    pub log_directory: PathBuf,
    pub log_runtime: bool,
    pub disable_mod_validation: bool,
    pub enable_achievements: bool,
    pub send_crash_reports: bool,
    pub enable_debugger: bool,
    pub debugger_port: u32,
    pub dump_network_strings: bool,
    pub debugger_flags: u32,
    pub developer_mode: bool,
    pub enable_lua_debugger: bool,

    pub lua_builtin_resource_directory: PathBuf,
    pub default_to_client_console: bool,
    pub show_perf_warnings: bool,
}
impl Default for ScriptExtenderSettings {
    fn default() -> Self {
        Self {
            enable_extensions: true,
            create_console: false,
            log_failed_compile: true,
            enable_logging: false,
            log_compile: false,
            log_directory: PathBuf::default(),
            log_runtime: false,
            disable_mod_validation: true,
            enable_achievements: true,
            send_crash_reports: true,
            enable_debugger: false,
            debugger_port: 9999,
            dump_network_strings: false,
            debugger_flags: 0,
            developer_mode: false,
            enable_lua_debugger: false,
            lua_builtin_resource_directory: PathBuf::default(),
            default_to_client_console: false,
            show_perf_warnings: false,
        }
    }
}
