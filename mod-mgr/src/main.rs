pub mod app_keys;
pub mod main_view;
mod mod_table_view;
pub mod resources;
pub mod settings_view;
pub mod tab_view;
pub mod table;
pub mod ui_mod_data;
pub mod util;
pub mod view_util;

use std::path::{Path, PathBuf};

use clap::Parser;
use floem::{
    peniko::Color,
    reactive::{create_rw_signal, RwSignal},
};
use main_view::{app_view, StartupStage};
use mod_mgr_lib::{
    settings::{ScriptExtenderSettings, Settings},
    util::divinity_registry_helper::{self, get_game_install_path},
    BG3_STEAM_ID,
};
use resources::default_paths;
use ui_mod_data::UIModData;
use util::space_replace;

use crate::util::space_replace_front;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Run the application 'dry' without actually doing anything.
    #[clap(long)]
    dry: bool,
    // TODO(minor): custom settings path, overwrite active game data path, etc.
}

// TODO: updater
// TODO: about window

fn main() {
    let args = Args::parse();

    // TODO: load settings from a settings file
    let settings_path = PathBuf::from("Data/settings.json");
    let settings = load_settings(&settings_path).expect("Failed to parse settingsuration file");

    // TODO: window title
    // TODO: on linux systems, alert that the window should be floating by default for tiling window managers. Or at least do that for settings/about

    // Start GUI
    let root_view = move || {
        let main_data = MainData::new(args.clone(), settings.clone());
        let startup_stage = main_data.startup_stage.clone();

        main_data.settings.with(move |settings| {
            if !settings.game_data_path.is_dir() || !settings.game_executable_path.is_file() {
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

fn load_settings(path: &Path) -> serde_json::Result<Settings> {
    if let Ok(content) = std::fs::read_to_string(&path) {
        // TODO: we could show a dialog box to the user asking them if they want to
        // - replace their settingsuration with the default settings
        // - or open it in a text editor to make it easy to share with me
        serde_json::from_str(&content)
    } else {
        // The file doesn't exist so we'll just use the default settingsuration.
        eprintln!("There was no settingsuration file at: {path:?}, using default settingsuration.");
        Ok(Settings::default())
    }
}

#[derive(Debug, Clone)]
pub struct MainData {
    pub dry: bool,
    pub settings: RwSignal<Settings>,
    pub workshop_support_enabled: RwSignal<bool>,

    pub startup_stage: RwSignal<StartupStage>,
    pub pathway: RwSignal<PathwayData>,
    // TODO: is this ever initialized?
    pub extender_settings: RwSignal<Option<ScriptExtenderSettings>>,
    pub mods: im::Vector<UIModData>,
}
impl MainData {
    // Roughly equivalent to C#'s LoadSettings, though it has the settings/settings loaded outside it.
    pub fn new(args: Args, settings: Settings) -> MainData {
        // TODO: loadappsettings?
        let settings = create_rw_signal(settings.clone());

        let (game_data_path, doc_path_override) = settings.with_untracked(|settings| {
            (
                settings.game_data_path.clone(),
                settings.documents_folder_path_override.clone(),
            )
        });
        let doc_path_override = if doc_path_override.as_os_str().is_empty() {
            None
        } else {
            Some(doc_path_override)
        };

        let pathway_data = PathwayData::new(settings.clone(), &game_data_path, doc_path_override);

        let pathway_data = create_rw_signal(pathway_data);

        // TODO: workshop support impl, though BG3 does not have it currently
        let workshop_support_enabled = create_rw_signal(false);

        let startup_stage = create_rw_signal(StartupStage::Loading);

        // TODO: there's various Keys that get actions added to them.

        let extender_settings = create_rw_signal(None);

        // TODO: log enabled listener
        // TODO: theme change listener
        // TODO: extender settings enable extension listener
        // TODO: action on game launch changed listener
        // TODO: display filenames header change. Though this should be done in that area of code.
        // TODO: listen for documents folder path override change?
        // TOOD: keep track of window location if that setting is set.

        // if settings.with_untracked(|settings| settings.log_enabled) {}

        MainData {
            dry: args.dry,
            settings,
            workshop_support_enabled,
            startup_stage,
            pathway: pathway_data,
            extender_settings,
            mods: im::Vector::new(),
        }
    }

    // The original implementation register a bunch of their reactive library's signals for when a
    // value changes as variables. We just have them as functions which serve the same purpose,
    // where you will listen to what they depend on. And when that changes, that will trigger
    // functions that can depend on signals (like views and such) to be recomputed.
    //
    // However that is for ones that are side-effect free. They're essentially just altering the
    // specific value that they are.
    // Other listeners, their entire point side effects rather than being data, and so we construct
    // those with create effect above.

    // TODO: These functions would check more than they should because they're listening on changes to settings rather than the necessary fields. We could make a second version of settings that replicates the fields as RwSignals to have better granularity.

    /// Check if the workshop folder is available & exists.
    /// Requires that workshop support be enabled and that the path is valid.  
    /// Note: this will track the signals required
    pub fn can_open_workshop_folder(&self) -> bool {
        if !self.workshop_support_enabled.get() {
            return false;
        }

        self.settings.with(|settings| {
            let path = &settings.workshop_path;
            !path.as_os_str().is_empty() && path.is_dir()
        })
    }

    /// Check if the game exe can be opened.  
    /// Requires that the path is valid.  
    /// Note: this will track the signals required
    pub fn can_open_game_exe(&self) -> bool {
        self.settings.with(|settings| {
            let path = &settings.game_executable_path;
            !path.as_os_str().is_empty() && path.is_file()
        })
    }

    /// Note: this will track the signals required
    pub fn extender_log_directory(&self) -> Option<PathBuf> {
        self.extender_settings
            .with(|ext| Some(ext.as_ref()?.log_directory.clone()))
    }

    /// Check if the log directory can be updated.
    pub fn can_open_log_directory(&self) -> bool {
        let Some(path) = self.extender_log_directory() else {
                return false;
            };
        !path.as_os_str().is_empty() && path.is_dir()
    }

    pub fn open_logs_folder(&self) {
        if let Some(path) = self.extender_log_directory() {
            if let Err(err) = open::that_detached(&path) {
                eprintln!("Failed to open logs folder: {err}");
            }
        }
    }

    /// Check if the mods folder is available & exists.  
    /// Note: this will track the signals required
    pub fn can_open_mods_folder(&self) -> bool {
        self.pathway.with(|pathway| {
            let path = &pathway.documents_mods_path;
            !path.as_os_str().is_empty() && path.is_dir()
        })
    }

    /// Open the mods folder in the user's file browser
    pub fn open_mods_folder(&self) {
        if let Err(err) = self
            .pathway
            .with_untracked(|pathway| open::that_detached(&pathway.documents_mods_path))
        {
            eprintln!("Failed to open mods folder: {err}");
        }
    }

    pub fn can_open_game_folder(&self) -> bool {
        self.settings.with_untracked(|settings| {
            let path = &settings.game_executable_path;
            !path.as_os_str().is_empty() && path.is_file()
        })
    }

    pub fn open_game_folder(&self) {
        self.settings.with_untracked(|settings| {
            if let Some(folder) = settings.game_executable_path.parent() {
                if folder.is_dir() {
                    if let Err(err) = open::that_detached(&folder) {
                        eprintln!("Failed to open game folder: {err}");
                    }
                }
            }
        })
    }

    pub fn open_workshop_folder(&self) {
        self.settings.with_untracked(|settings| {
            if !settings.workshop_path.as_os_str().is_empty() && settings.workshop_path.is_dir() {
                if let Err(err) = open::that_detached(&settings.workshop_path) {
                    eprintln!("Failed to open workshop folder: {err}");
                }
            }
        })
    }

    pub fn launch_game(&self) {
        // TODO: linux implemenation
        // TODO: macos implementation
        // TODO: could we just generalize this by talking to steam? Probably can't pass arguments that way?

        self.settings.with_untracked(|settings| {
            if !cfg!(windows) {
                // TODO: Can we tell steam to launch the game with specific args?
                // Or can we use proton-call and somehow get the proton version they have set?

                // TODO: show this as an alert, or as a tooltip and don't let linux/mac users click it in the first place
                // TODO: can mac users just launch the game with it since they have a native version?
                eprintln!("Launching game is not supported on this platform");
                return;
            }

            if !settings.game_executable_path.is_file() {
                if settings.game_executable_path.as_os_str().is_empty() {
                    eprintln!("No game executable path set");
                    // TODO: show an alert.
                } else {
                    eprintln!(
                        "Failed to find game executable at: {:?}",
                        settings.game_executable_path
                    );
                    // TODO: show an alert.
                }

                return;
            }

            let mut launch_params = settings.game_launch_params.clone();

            if settings.game_story_log_enabled && !launch_params.contains("storylog") {
                space_replace(&mut launch_params, "-storylog 1");
            }

            if settings.skip_launcher && !launch_params.contains("skip-launcher") {
                space_replace_front(&mut launch_params, "-skip-launcher");
            }

            let mut exe_path = settings.game_executable_path.clone();
            let exe_dir = exe_path.parent().unwrap();

            if settings.launch_dx11 {
                let next_exe = exe_dir.join("bg3_dx11.exe");
                if next_exe.is_file() {
                    exe_path = next_exe;
                }
            }

            eprintln!("Opening game exe at: {exe_path:?} with args {launch_params}");

            // We have to parse the arguments because Rust's command does not let you directly run
            // it.
            // We could manually pass it to system specific bash/cmd/whatever, but that is just
            // harder and has issues of escaping the parameters properly.
            //
            // We could store the parameters as a vector, but that would break settings file
            // compatibility with C# BG3ModManager. There's other methods but they have their own
            // issues. This is simple and more than fast enough.
            let args = shlex::split(&launch_params).unwrap_or_else(Vec::new);

            let mut command = std::process::Command::new(exe_path);
            command.args(args);

            match command.spawn() {
                Ok(_child) => {}
                Err(err) => {
                    eprintln!("Failed to launch game: {err}");
                }
            }

            todo!()
        })
    }

    /// Save the settings. This should be preferred over directly using [`Settings::save`] as it
    /// does various minor fixups.  
    /// Note: this does not track any signals.
    pub fn save_settings(&self) {
        let game_executable_path = self
            .settings
            .with_untracked(|settings| settings.game_executable_path.clone());

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
                    self.settings.update(|settings| {
                        settings.game_executable_path = exe;
                    })
                }
            }
        }

        // TODO: it runs export_extender_settings

        self.settings.with_untracked(|settings| {
            if self.dry {
                eprintln!("Would have saved settings");
                return;
            }

            if let Ok(_) = settings.save() {
                // TODO: It shows on the alert bar, 'saved settings to blahblah'
            }
        });
    }

    pub fn open_settings_folder(&self) {
        if let Err(err) = self
            .settings
            .with_untracked(|settings| open::that_detached(mod_mgr_lib::DIR_DATA))
        {
            eprintln!("Failed to open settings folder: {err}");
        }
    }

    pub fn export_extender_settings(&self) {
        if self.extender_settings.with_untracked(Option::is_none) {
            // TODO: C# version writes it regardless of whether it is initialized?
            eprintln!("Did not save script extender settings due to it not being initialized");
            return;
        }

        let game_executable_path = self
            .settings
            .with_untracked(|settings| settings.game_executable_path.clone());

        let dir_name = game_executable_path.parent().unwrap_or(&Path::new(""));
        let output_file = dir_name.join("ScriptExtenderSettings.json");

        // TODO: we aren't saving it quite like how the mod manager does it. They skip over fields that are their default values, if a certain setting is off.

        // let contents = serde_json::to_string_pretty(self.extender_settings.as_ref().unwrap());
        let contents = self
            .extender_settings
            .with_untracked(|ext| serde_json::to_string_pretty(ext.as_ref().unwrap()));

        match contents {
            Ok(contents) => {
                if let Err(err) = std::fs::write(&output_file, contents) {
                    eprintln!("Failed to write Script Extender settings to {output_file:?}: {err}");
                }

                // TODO: it shows a success alert
            }
            // TODO: it shows an alert
            Err(err) => {
                eprintln!("Error tyrbubg Script Extender settings into json: {err:?}")
            }
        }
    }

    // TODO: can reset extender settings observable

    pub fn reset_extender_settings_to_default(&self) {
        // TODO: it asks the user if they're sure. Really that should be done outside this function.

        self.settings.update(|settings| {
            settings.export_default_extender_settings = false;
        });
        self.extender_settings
            .set(Some(ScriptExtenderSettings::default()));
    }

    pub fn reset_keybindings(&self) {
        // TODO:
    }

    pub fn clear_workshop_cache(&self) {
        let path = Path::new("Data/workshopdata.json");
        if path.is_file() {
            // TODO:
        }
    }

    pub fn add_launch_param(&self, param: impl AsRef<str>) {
        self.settings.update(|settings| {
            space_replace(&mut settings.game_launch_params, param.as_ref());
        })
    }

    pub fn clear_launch_params(&self) {
        self.settings.update(|settings| {
            settings.game_launch_params = String::new();
        })
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
        settings: RwSignal<Settings>,
        current_game_data_path: &Path,
        larian_documents_folder: Option<PathBuf>,
    ) -> PathwayData {
        let (game_executable_path, game_data_path) = settings.with_untracked(|settings| {
            (
                settings.game_executable_path.clone(),
                settings.game_data_path.clone(),
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
                            settings.update(|settings| {
                                settings.game_executable_path = exe_path;
                                eprintln!("Exe path set to {:?}", settings.game_executable_path);
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
                    settings.update(|settings| {
                        settings.game_executable_path = exe_path;
                        eprintln!("Exe path set to {:?}", settings.game_executable_path);
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
