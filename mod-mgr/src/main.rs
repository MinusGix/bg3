pub mod app_keys;
pub mod main_view;
mod mod_table_view;
pub mod resources;
pub mod settings_view;
pub mod tab_view;
pub mod table;
pub mod view_util;

use std::path::{Path, PathBuf};

use clap::Parser;
use floem::{
    peniko::Color,
    reactive::{create_rw_signal, RwSignal},
};
use main_view::{app_view, StartupStage};
use mod_mgr_lib::{
    config::Config,
    util::divinity_registry_helper::{self, get_game_install_path},
    BG3_STEAM_ID,
};
use resources::default_paths;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Run the application 'dry' without actually doing anything.
    #[clap(long)]
    dry: bool,
    // TODO(minor): custom config path, overwrite active game data path, etc.
}

// TODO: updater
// TODO: about window

fn main() {
    let args = Args::parse();

    // TODO: load config from a config file
    let settings_path = PathBuf::from("Data/settings.json");
    let config = load_config(&settings_path).expect("Failed to parse configuration file");

    // TODO: window title
    // TODO: on linux systems, alert that the window should be floating by default for tiling window managers. Or at least do that for settings/about

    // Start GUI
    let root_view = move || {
        let main_data = MainData::new(args.clone(), config.clone());
        let startup_stage = main_data.startup_stage.clone();

        main_data.config.with(move |config| {
            if !config.game_data_path.is_dir() || !config.game_executable_path.is_file() {
                eprintln!("Failed to find game data path. Asking user for the location");
                startup_stage.set(StartupStage::AskGamePath);
            } else {
                startup_stage.set(StartupStage::Ready);
            }
        });

        // let view = app_view(main_data);
        // let id = view.id();

        app_view(main_data)
    };

    floem::launch(root_view)
}

fn load_config(path: &Path) -> serde_json::Result<Config> {
    if let Ok(content) = std::fs::read_to_string(&path) {
        // TODO: we could show a dialog box to the user asking them if they want to
        // - replace their configuration with the default config
        // - or open it in a text editor to make it easy to share with me
        serde_json::from_str(&content)
    } else {
        // The file doesn't exist so we'll just use the default configuration.
        eprintln!("There was no configuration file at: {path:?}, using default configuration.");
        Ok(Config::default())
    }
}

#[derive(Debug, Clone)]
pub struct MainData {
    pub dry: bool,
    pub config: RwSignal<Config>,
    pub workshop_support_enabled: RwSignal<bool>,

    pub startup_stage: RwSignal<StartupStage>,
    pub pathway: RwSignal<PathwayData>,
}
impl MainData {
    // Roughly equivalent to C#'s LoadSettings, though it has the config/settings loaded outside it.
    pub fn new(args: Args, config: Config) -> MainData {
        // TODO: loadappconfig
        let config = create_rw_signal(config.clone());

        // TODO: documents folder path override
        let documents_folder_path_override = None;
        let game_data_path = config.with_untracked(|config| config.game_data_path.clone());
        let pathway_data = PathwayData::new(
            config.clone(),
            &game_data_path,
            documents_folder_path_override,
        );

        let pathway_data = create_rw_signal(pathway_data);

        // TODO: workshop support impl, though BG3 does not have it currently
        let workshop_support_enabled = create_rw_signal(false);

        let startup_stage = create_rw_signal(StartupStage::Loading);

        // TODO: there's various Keys that get actions added to them.

        MainData {
            dry: args.dry,
            config,
            workshop_support_enabled,
            startup_stage,
            pathway: pathway_data,
        }
    }

    // TODO: I'm currently assuming that our reactive tracking works the same as their
    // implementation.
    // That is, the current assumption is that it does not recheck path validity until the setting
    // is changed. I should check that just to make absolutely sure.

    // TODO: These functions would check more than they should because they're listening on changes to config rather than the necessary fields. We could make a second version of config that replicates the fields as RwSignals to have better granularity.

    /// Check if the workshop folder is available & exists.
    /// Requires that workshop support be enabled and that the path is valid.  
    /// Note: this will track the signals required
    pub fn can_open_workshop_folder(&self) -> bool {
        if !self.workshop_support_enabled.get() {
            return false;
        }

        self.config.with(|config| {
            let path = &config.workshop_path;
            !path.as_os_str().is_empty() && path.is_dir()
        })
    }

    /// Check if the game exe can be opened.  
    /// Requires that the path is valid.  
    /// Note: this will track the signals required
    pub fn can_open_game_exe(&self) -> bool {
        self.config.with(|config| {
            let path = &config.game_executable_path;
            !path.as_os_str().is_empty() && path.is_file()
        })
    }

    // TODO:
    // /// Check if the log directory can be updated.
    // pub fn can_open_log_directory(&self) -> bool {
    //     self.config.with(|config| {
    //         let path = &config.extender_log_directory;
    //         !path.as_os_str().is_empty() && path.is_dir()
    //     })
    // }

    /// Check if the mods folder is available & exists.  
    /// Note: this will track the signals required
    pub fn can_open_mods_folder(&self) -> bool {
        self.pathway.with(|pathway| {
            let path = &pathway.documents_mods_path;
            !path.as_os_str().is_empty() && path.is_dir()
        })
    }

    /// Save the config. This should be preferred over directly using [`Config::save`] as it
    /// does various minor fixups.  
    /// Note: this does not track any signals.
    pub fn save_config(&self) {
        let game_executable_path = self
            .config
            .with_untracked(|config| config.game_executable_path.clone());

        // If the user set the game executable path to a directory, we try to fix that by finding
        // the exe file in the dir and saving that instead.
        if game_executable_path.is_dir() {
            // TODO(minor): We don't really need to parse these.
            let exe_path = if !divinity_registry_helper::is_gog() {
                PathBuf::from(default_paths::STEAM_INFO.exe_path)
            } else {
                PathBuf::from(default_paths::GOG_INFO.exe_path)
            };
            // The name of the executable we expect. The default paths always have a file name so we
            // can unwrap without worry.
            let exe_name = exe_path.file_name().unwrap();

            let exe = game_executable_path.join(exe_name);
            if exe.exists() {
                if self.dry {
                    eprintln!("Would have updated game_executable_path to {exe:?}");
                } else {
                    self.config.update(|config| {
                        config.game_executable_path = exe;
                    })
                }
            }
        }

        // TODO: it runs export_extender_settings

        self.config.with_untracked(|config| {
            if self.dry {
                eprintln!("Would have saved config");
                return;
            }

            if let Ok(_) = config.save() {
                // TODO: It shows on the alert bar, 'saved settings to blahblah'
            }
        });
    }

    pub fn open_settings_folder(&self) {
        todo!()
    }

    pub fn export_extender_settings(&self) {
        let game_executable_path = self
            .config
            .with_untracked(|config| config.game_executable_path.clone());

        let dir_name = game_executable_path.parent().unwrap_or(&Path::new(""));
        let output_file = dir_name.join("ScriptExtenderSettings.json");

        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct PathwayData {
    /// Path to the root game folder.  
    /// i.e. `SteamLibrary/steamapps/common/Baldur's Gate 3`  
    /// i.e. `.steam/steamapps/common/Baldur's Gate 3`
    pub install_path: PathBuf,

    /// Windows: Path to `%LOCALAPPDATA%/Larian Studios/Baldur's Gate 3`
    /// Linux: TODO (pfx)
    /// MacOS: TODO
    pub larian_documents_folder: PathBuf,
    /// Windows: Path to `%LOCALAPPDATA%/Larian Studios/Baldur's Gate 3/Mods`
    pub documents_mods_path: PathBuf,
    /// Windows: Path to `%LOCALAPPDATA%/Larian Studios/Baldur's Gate 3/GMCampaigns`
    pub documents_gm_campaigns_path: PathBuf,
    /// Windows: Path to `%LOCALAPPDATA%/Larian Studios/Baldur's Gate 3/PlayerProfiles`
    pub documents_profiles_path: PathBuf,
    // TODO: last save file path
    // TODO: script extendere latest release url
    // TODO: script extender latest release version
}
impl PathwayData {
    // TODO(minor): Does current game data path really need to be a separate field in this impl? Or even in the original?
    /// Create a new [`PathwayData`] instance from the given information.  
    /// If the supplied `larian_documents_folder` argument is not set it will be inferred.
    pub fn new(
        config: RwSignal<Config>,
        current_game_data_path: &Path,
        larian_documents_folder: Option<PathBuf>,
    ) -> PathwayData {
        let (game_executable_path, game_data_path) = config.with_untracked(|config| {
            (
                config.game_executable_path.clone(),
                config.game_data_path.clone(),
            )
        });

        // TODO: some of this should be windows only
        let mut documents_folder = std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                PathBuf::from("C:/Users")
                    .join(std::env::var("USERNAME").expect("Failed to get username"))
                    .join("AppData/Local")
            });

        // TODO: I don't really know why they initialize documents game folder when they already have it set in a json. Just load it from there? I just skip that.

        let mut larian_documents_folder = larian_documents_folder
            .unwrap_or_else(|| documents_folder.join(default_paths::DOCUMENTS_GAME_FOLDER));

        if !larian_documents_folder.is_dir() {
            if let Some(user_folder) = dirs::home_dir() {
                if user_folder.is_dir() {
                    documents_folder = user_folder.join("AppData/Local");
                    larian_documents_folder =
                        documents_folder.join(default_paths::DOCUMENTS_GAME_FOLDER);
                }
            }
        }

        let mod_pak_folder = larian_documents_folder.join("MOds");
        let gm_campaign_folder = larian_documents_folder.join("GMCampaigns");
        let profile_folder = larian_documents_folder.join("PlayerProfiles");

        let mut pathway = PathwayData {
            larian_documents_folder,
            documents_mods_path: mod_pak_folder,
            documents_gm_campaigns_path: gm_campaign_folder,
            documents_profiles_path: profile_folder,

            install_path: PathBuf::default(),
        };

        if documents_folder.is_dir() {
            std::fs::create_dir(&pathway.larian_documents_folder)
                .expect("Failed to create larian game documents folder");
            if !pathway.documents_mods_path.is_dir() {
                eprintln!(
                    "No mods folder found at {:?}. Creating folder.",
                    pathway.documents_mods_path
                );
                std::fs::create_dir(&pathway.documents_mods_path)
                    .expect("Failed to create mods folder");
            }

            if !pathway.documents_gm_campaigns_path.is_dir() {
                eprintln!(
                    "No GM campaigns folder found at {:?}. Creating folder.",
                    pathway.documents_gm_campaigns_path
                );
                std::fs::create_dir(&pathway.documents_gm_campaigns_path)
                    .expect("Failed to create GM campaigns folder");
            }

            if !pathway.documents_profiles_path.is_dir() {
                eprintln!(
                    "No player profiles folder found at {:?}. Creating folder.",
                    pathway.documents_profiles_path
                );
                std::fs::create_dir(&pathway.documents_profiles_path)
                    .expect("Failed to create player profiles folder");
            }
        } else {
            // TODO: show the error dialog so it isn't hidden
            eprintln!("Failed to find %USERPROFILE%\\Documents folder. This is weird. Got path: {documents_folder:?}");
        }

        // If the current game data path isn't valid then we'll try to find it.
        if current_game_data_path.as_os_str().is_empty() || !current_game_data_path.is_dir() {
            let install_path = get_game_install_path(
                default_paths::GOG_INFO.registry_32,
                default_paths::GOG_INFO.registry_64,
                BG3_STEAM_ID,
            );

            if let Some(install_path) = install_path {
                if !install_path.as_os_str().is_empty() && install_path.is_dir() {
                    pathway.install_path = install_path;

                    if !game_executable_path.is_file() {
                        let exe_path = if !divinity_registry_helper::is_gog() {
                            pathway
                                .install_path
                                .join(default_paths::STEAM_INFO.exe_path)
                        } else {
                            pathway.install_path.join(default_paths::GOG_INFO.exe_path)
                        };

                        if exe_path.is_file() {
                            config.update(|config| {
                                config.game_executable_path = exe_path;
                                eprintln!("Exe path set to {:?}", config.game_executable_path);
                            })
                        }
                    }
                }
            }
        } else {
            // TODO: check how it is handling canonicalize errors in C# code
            let install_path = game_data_path.join("../..").canonicalize().unwrap();
            pathway.install_path = install_path;

            if !game_executable_path.is_file() {
                let exe_path = if !divinity_registry_helper::is_gog() {
                    pathway
                        .install_path
                        .join(default_paths::STEAM_INFO.exe_path)
                } else {
                    pathway.install_path.join(default_paths::GOG_INFO.exe_path)
                };

                if exe_path.is_file() {
                    config.update(|config| {
                        config.game_executable_path = exe_path;
                        eprintln!("Exe path set to {:?}", config.game_executable_path);
                    });
                }
            }

            todo!()
        }

        // The check at the end of the C# version of this function is not done here because we
        // can't (nicely) synchronously open a file dialogue. We have a separate view that is shown
        // on startup for that.

        pathway
    }
}

// TODO: move this to lib?
#[derive(Debug, Clone)]
pub struct GameDataInfo {
    pub install_path: PathBuf,
    /// If this is `None` then it seemed to be invalid.
    pub game_data_path: Option<PathBuf>,
    pub game_exe_path: Option<PathBuf>,
}
impl GameDataInfo {
    /// Get install information from the given path.  
    /// If both `game_data_path` and `game_exe_path` are `None` then the path was almost certainly
    /// invalid.
    pub fn from_path(path: &Path) -> GameDataInfo {
        let data_directory = path.join(default_paths::GAME_DATA_FOLDER);
        let mut exe_path = path.join(default_paths::STEAM_INFO.exe_path);
        if !exe_path.is_file() {
            exe_path = path.join(default_paths::GOG_INFO.exe_path);
        }

        let data_directory = if data_directory.is_dir() {
            Some(data_directory)
        } else {
            // TODO: show this as an alert.
            eprintln!("Failed to find Data folder within given installation directory");
            None
        };

        let exe_path = if exe_path.is_file() {
            Some(exe_path)
        } else {
            None
        };

        GameDataInfo {
            install_path: path.to_owned(),
            game_data_path: data_directory,
            game_exe_path: exe_path,
        }
    }
}

/// Headers/footers
pub const DARK0_BG: Color = Color::rgb8(42, 43, 52);
/// Inputs
pub const DARK1_BG: Color = Color::rgb8(51, 51, 64);
/// Main background
pub const DARK2_BG: Color = Color::rgb8(61, 61, 76);
/// Selected option background
pub const DARK3_BG: Color = Color::rgb8(76, 79, 98);
/// Text in the dark theme
pub const DARK_TEXT: Color = Color::rgb8(209, 209, 212);
