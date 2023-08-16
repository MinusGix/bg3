//! https://github.com/LaughingLeader/BG3ModManager/blob/master/DivinityModManagerCore/Models/DivinityModData.cs  
//! Though this is split up into multiple files in a different manner.

use std::{borrow::Cow, path::PathBuf};

use mod_mgr_lib::{
    mod_data::{DivinityModDependencyData, ModData, ModVersion},
    MAIN_CAMPAIGN_UUID,
};

// TODO: this has various selection/active, workshop, etc fields on it.
// TODO: I think various pieces of these could be put into core.
#[derive(Debug, Clone)]
pub struct UIModData {
    pub data: ModData,
    pub idx: usize,
    pub mod_type: String,
    pub modes: Vec<String>,
    pub targets: String,
    pub last_updated: u64,
    pub extender_status: DivinityExtenderModStatus,
    pub current_extender_version: Option<u32>,
    pub extender_data: ScriptExtenderConfig,
    pub dependencies: im::Vector<DivinityModDependencyData>,
    pub has_script_extender_settings: bool,
    pub is_editor_mod: bool,
    pub is_active: bool,
}
impl UIModData {
    pub fn output_pak_name(&self) -> PathBuf {
        todo!()
    }

    pub fn tooltip(&self, current_version: Option<u32>) -> String {
        let tooltip = self.extender_status.tooltip(&self.extender_data);
        if let Some(ver) = current_version {
            format!("{tooltip}(Currently installed version is v{ver})")
        } else {
            format!("{tooltip}(No installed extender version found)")
        }
    }

    pub fn display_name(&self) -> String {
        if self.data.display_file_for_name {
            if self.is_editor_mod {
                format!("{} [Editor Project]", self.data.folder)
            } else {
                self.data.filename()
            }
        } else {
            // TODO: C# version checks whether dev mode is enabled
            if self.data.uuid == MAIN_CAMPAIGN_UUID {
                return "Main".to_string();
            } else if self.data.is_classic_mod {
                format!("{} [Classic]", self.data.name)
            } else {
                self.data.name.clone()
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DivinityExtenderModStatus {
    None,
    Supports,
    Required,
    RequiredOld,
    RequiredMissing,
    RequiredDisabled,
}
impl DivinityExtenderModStatus {
    pub fn tooltip(self, data: &ScriptExtenderConfig) -> Cow<'static, str> {
        use DivinityExtenderModStatus::*;
        match self {
            None => Cow::Borrowed(""),
            Supports => {
                if data.required_extension_version.is_some() {
                    Cow::Owned(format!(
                        "Supports Script Extender v{} or higher\n",
                        data.required_extension_version.unwrap()
                    ))
                } else {
                    Cow::Borrowed("Supports the Script Extender\n")
                }
            }
            Required | RequiredMissing | RequiredDisabled | RequiredOld => {
                let prefix = if self == RequiredMissing {
                    "[MISSING] "
                } else if self == RequiredDisabled {
                    "[EXTENSIONS DISABLED] "
                } else if self == RequiredOld {
                    "[OLD] "
                } else {
                    unreachable!()
                };

                let req = if let Some(version) = data.required_extension_version {
                    Cow::Owned(format!("Requires Script Extender v{version} or higher"))
                } else {
                    Cow::Borrowed("Requires the Script Extender")
                };

                let extra = if self == RequiredDisabled {
                    " (Enable Extensions in the Script Extender config)"
                } else if self == RequiredOld {
                    " (Update by running the game)"
                } else {
                    ""
                };

                Cow::Owned(format!("{}{}{}\n", prefix, req, extra))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptExtenderConfig {
    pub required_extension_version: Option<u32>,
    pub feature_flags: Vec<String>,
}
impl ScriptExtenderConfig {
    pub fn has_any_settings(&self) -> bool {
        self.required_extension_version.is_some() || !self.feature_flags.is_empty()
    }
}

pub fn load_workshop_mods() {
    todo!()
}

pub fn check_for_mod_updates() {
    todo!()
}
