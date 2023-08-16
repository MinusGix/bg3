use std::{io::Write, path::PathBuf};

use serde::{Deserialize, Serialize};

// TODO: provide these descriptions in the ui

const SETTINGS_PATH: &str = "Data/settings.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Settings {
    // TODO: give the paths decent default values on the common platforms.
    /// Path to the game's data folder.  
    /// Ex: "C:/Steam/steamapps/common/Baldurs Gate 3/Data"
    pub game_data_path: PathBuf,
    /// Path to the game's executable.  
    /// Ex: "C:/Steam/steamapps/common/Baldurs Gate 3/bin/bg3.exe"
    pub game_executable_path: PathBuf,
    /// Override the default location to `%LOCALAPPDATA/Larian Studios/Baldur's Gate 3/`
    pub documents_folder_path_override: PathBuf,
    /// Whether Larian's telemetry options for BG3 will always be disabled, regardless of active
    /// mods. Telemetry is always disabled if mods are active.
    pub telemetry_disabled: bool,
    /// Folder name where load orders should be saved.
    pub saved_load_orders_path: PathBuf,
    /// Path to the workshop folder. Currently unused.
    pub workshop_path: PathBuf,
    /// When launching the game, enabled the Osiris story log (osiris.log)
    pub game_story_log_enabled: bool,
    pub auto_add_missing_dependencies_on_export: bool,
    // TODO: auto updates
    // pub enable_automatic_updates: bool,
    /// If a load order is missing mods, no warnings will be displayed
    pub disable_missing_mod_warnings: bool,
    /// The mod manager will try and find mod tags from the workshop by default
    pub disable_workshop_tag_check: bool,
    /// Export all values, even if it matches a default extender value
    pub export_default_extender_settings: bool,
    /// When moving selected mods to the opposite list with Enter, move focus to that list as well
    pub shift_focus_on_swap: bool,
    pub save_window_location: bool,
    #[serde(rename = "LaunchDX11")]
    pub launch_dx11: bool,
    /// Pass `--skip-launcher` when launching the game.
    pub skip_launcher: bool,
    /// Automatically check for updates when the program starts
    pub check_for_updates: bool,
    pub game_launch_params: String,
    // // TODO: linux specific settings? Like proton version to use?
    // pub launch: LaunchSettings,
}
impl Settings {
    pub fn load() -> std::io::Result<Settings> {
        todo!()
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let contents = serde_json::to_string_pretty(self)?;
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(SETTINGS_PATH)?;

        file.write_all(contents.as_bytes())?;

        Ok(())
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
impl Default for Settings {
    fn default() -> Self {
        Self {
            // TODO: check the actual defaults
            game_data_path: PathBuf::new(),
            game_executable_path: PathBuf::new(),
            saved_load_orders_path: PathBuf::from("Orders/"),
            workshop_path: PathBuf::new(),
            game_story_log_enabled: false,
            auto_add_missing_dependencies_on_export: true,
            telemetry_disabled: false,
            launch_dx11: false,
            documents_folder_path_override: PathBuf::new(),
            disable_missing_mod_warnings: false,
            shift_focus_on_swap: false,
            save_window_location: true,
            disable_workshop_tag_check: false,
            export_default_extender_settings: false,
            skip_launcher: true,
            check_for_updates: true,
            game_launch_params: String::new(),
            // launch: LaunchSettings::default(),
        }
    }
}

// #[derive(Debug, Default, Clone, Serialize, Deserialize)]
// pub struct LaunchSettings {
//     /// Automatically load the last save when loading into the main menu.
//     pub continue_game: bool,
//     /// Enables the story log
//     pub story_log: bool,
//     /// A directory to write story logs to
//     pub log_path: Option<PathBuf>,
//     /// Limit the cpu to x amount of threads (unknown if this works)
//     pub cpu_limit: Option<usize>,
//     pub asserts: bool,
//     pub stats: bool,
//     pub dynamic_story: bool,
//     // TODO: give these their actual types
//     pub external_crash_handler: bool,
//     pub name_tag: bool,
//     pub module: bool,
//     pub connect_lobby: bool,
//     pub loca_updater: bool,
//     pub media_path: bool,
// }

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
