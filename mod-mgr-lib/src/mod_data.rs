use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub struct ModData {
    pub file_path: PathBuf,
    // TODO: uuids are constant size
    pub uuid: String,
    pub name: String,
    pub folder: String,
    pub description: String,
    pub author: String,
    pub md5: String,

    pub version: ModVersion,
    pub header_version: ModVersion,
    pub publish_version: ModVersion,

    pub last_modified: Option<u64>,

    pub display_file_for_name: bool,
    pub is_hidden: bool,

    /// If this mod is in `ignored_mods`, or the author is Larian.  
    /// Larian mods are hidden from the load order.  
    pub is_larian_mod: bool,
    /// Mods with a header version from the non-DE version are considered "classic" and can't be
    /// loaded in the DE version.
    pub is_classic_mod: bool,
    /// Whether the mod was loaded from the user's mod directory.  
    pub is_user_mod: bool,
    /// Whether the mod has a base game mod directory. This data is always loaded regardless if the
    /// mod is enabled or not.
    pub is_force_loaded: bool,
    /// Whether the mod has files of its own (i.e. it overrides Gustav, but it has Public/ModFolder/
    /// Assets files etc)
    pub is_force_loaded_merged_mod: bool,

    pub builtin_override_mods_text: String,

    pub help_text: String,

    pub tags: Vec<String>,

    pub visibility: Visibility,
}

impl ModData {
    // TODO(minor): don't unwrap
    pub fn filename(&self) -> String {
        self.file_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn display_name(&self) -> String {
        if self.display_file_for_name {
            self.filename()
        } else {
            self.name.clone()
        }
    }

    pub fn get_help_text() -> String {
        // TODO
        String::new()
    }

    pub fn add_tag(&mut self, tag: String) {
        if tag.trim().is_empty() || self.tags.contains(&tag) {
            return;
        }

        self.tags.push(tag);
        self.sort_tags();
    }

    pub fn add_tags(&mut self, tags: impl Iterator<Item = String>) {
        for tag in tags {
            if tag.trim().is_empty() || self.tags.contains(&tag) {
                continue;
            }

            self.tags.push(tag);
        }

        self.sort_tags();
    }

    fn sort_tags(&mut self) {
        self.tags
            .sort_by(|x, y| x.to_lowercase().cmp(&y.to_lowercase()));
    }

    pub fn description_visibility(&self) -> Visibility {
        if self.description.trim().is_empty() {
            Visibility::Collapsed
        } else {
            Visibility::Visible
        }
    }

    pub fn author_visibility(&self) -> Visibility {
        if self.author.trim().is_empty() {
            Visibility::Collapsed
        } else {
            Visibility::Visible
        }
    }

    pub fn folder_has_uuid(&self) -> bool {
        self.folder.contains(&self.uuid)
    }

    pub fn pak_name(&self) -> String {
        let mut filename = if self.folder_has_uuid() {
            self.filename()
        } else {
            format!("{}_{}", self.folder, self.uuid)
            // TODO: the C# code uses change extension for this branch, but is that really needed? Can't you just append the .pak?
        };

        change_extension(&mut filename, "pak");

        filename
    }

    pub fn pak_equals(&self, filename: &str) -> bool {
        // TODO: C# version lets you set string comparison. Default is ordinal.

        let mut output_package = if self.folder_has_uuid() {
            self.folder.clone()
        } else {
            format!("{}_{}", self.folder, self.uuid)
        };
        change_extension(&mut output_package, "pak");

        output_package == filename
    }

    pub fn is_newer_than(&self, time: u64) -> bool {
        self.last_modified.unwrap_or(0) > time
    }

    pub fn is_newer_than_mod(&self, other: &ModData) -> bool {
        self.is_newer_than(other.last_modified.unwrap_or(0))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModVersion {
    pub major: u8,
    pub minor: u8,
    pub revision: u16,
    pub build: u32,
}
impl From<u64> for ModVersion {
    fn from(v: u64) -> Self {
        let major = (v >> 55) as u8;
        let minor = ((v >> 47) & 0xff) as u8;
        let revision = ((v >> 32) & 0xffff) as u16;
        let build = (v & 0x7FFFFFFF) as u32;

        Self {
            major,
            minor,
            revision,
            build,
        }
    }
}
impl From<ModVersion> for u64 {
    fn from(v: ModVersion) -> Self {
        let mut res = 0u64;
        res |= (v.major as u64) << 55;
        res |= (v.minor as u64) << 47;
        res |= (v.revision as u64) << 32;
        res |= v.build as u64;
        res
    }
}
impl std::fmt::Display for ModVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.major, self.minor, self.revision, self.build
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Visible,
    Collapsed,
}

/// Change the extension of a string
fn change_extension(p: &mut String, ext: &str) -> bool {
    // We could parse it ourselves, but whatever.
    let path = Path::new(p);
    let new_path = path.with_extension(ext);
    if new_path == path {
        return false;
    }

    *p = new_path.to_string_lossy().into_owned();
    true
}

#[derive(Debug, Clone, PartialEq)]
pub struct DivinityModDependencyData {
    pub uuid: String,
    pub name: String,
    pub folder: String,
    pub md5: String,
    pub version: ModVersion,
}
impl DivinityModDependencyData {
    pub fn from_mod_data(data: &ModData) -> DivinityModDependencyData {
        DivinityModDependencyData {
            uuid: data.uuid.clone(),
            name: data.name.clone(),
            folder: data.folder.clone(),
            md5: data.md5.clone(),
            version: data.version,
        }
    }
}
