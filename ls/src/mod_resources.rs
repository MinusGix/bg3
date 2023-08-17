use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use once_cell::unsync::Lazy;
use regex::Regex;

use crate::{
    pak::{
        common::{FileInfo, FileInfoLike, FilesystemFileInfo},
        read_package, Package, PackageError,
    },
    story::compiler::compilation_context::TargetGame,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ModInfo {
    pub name: String,
    pub meta: Option<FileInfo>,
    pub scripts: IndexMap<String, FileInfo>,
    pub stats: IndexMap<String, FileInfo>,
    pub globals: IndexMap<String, FileInfo>,
    pub level_objects: IndexMap<String, FileInfo>,
    pub orphan_query_ignore_list: Option<FileInfo>,
    pub story_header_file: Option<FileInfo>,
    pub type_coercion_whitelist_file: Option<FileInfo>,
}
impl ModInfo {
    pub fn new(name: String) -> ModInfo {
        Self {
            name,
            meta: None,
            scripts: IndexMap::new(),
            stats: IndexMap::new(),
            globals: IndexMap::new(),
            level_objects: IndexMap::new(),
            orphan_query_ignore_list: None,
            story_header_file: None,
            type_coercion_whitelist_file: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModResources {
    pub mods: IndexMap<String, ModInfo>,
    pub loaded_packages: Vec<Package>,
}

// TODO: Check whether our regex will match the C# versions exactly
// TODO: Switch to stds lazycell once it is standardized
const META_RE: Lazy<Regex> = Lazy::new(|| {
    let v = "(?i)^Mods/([^/]+)/meta\\.lsx$";
    Regex::new(v).unwrap_or_else(|_| panic!("Failed to compile regex: {}", v))
});
const SCRIPT_RE: Lazy<Regex> = Lazy::new(|| {
    let v = "(?i)^Mods/([^/]+)/Story/RawFiles/Goals/(.*\\.txt)$";
    Regex::new(v).unwrap_or_else(|_| panic!("Failed to compile regex: {}", v))
});
const STAT_RE: Lazy<Regex> = Lazy::new(|| {
    let v = "(?i)^Public/([^/]+)/Stats/Generated/Data/(.*\\.txt)$";
    Regex::new(v).unwrap_or_else(|_| panic!("Failed to compile regex: {}", v))
});
const ORPHAN_QUERY_IGNORES_RE: Lazy<Regex> = Lazy::new(|| {
    let v = "(?i)^Mods/([^/]+)/Story/story_orphanqueries_ignore_local\\.txt$";
    Regex::new(v).unwrap_or_else(|_| panic!("Failed to compile regex: {}", v))
});
const STORY_DEFINITIONS_RE: Lazy<Regex> = Lazy::new(|| {
    let v = "(?i)^Mods/([^/]+)/Story/RawFiles/story_header\\.div$";
    Regex::new(v).unwrap_or_else(|_| panic!("Failed to compile regex: {}", v))
});
const TYPE_COERCION_WHITELIST_RE: Lazy<Regex> = Lazy::new(|| {
    let v = "(?i)^Mods/([^/]+)/Story/RawFiles/TypeCoercionWhitelist\\.txt$";
    Regex::new(v).unwrap_or_else(|_| panic!("Failed to compile regex: {}", v))
});
const GLOBALS_RE: Lazy<Regex> = Lazy::new(|| {
    let v = "(?i)^Mods/([^/]+)/Globals/.*/.*/.*\\.lsf$";
    Regex::new(v).unwrap_or_else(|_| panic!("Failed to compile regex: {}", v))
});
const LEVEL_OBJECTS_RE: Lazy<Regex> = Lazy::new(|| {
    let v = "(?i)^Mods/([^/]+)/Levels/.*/(Characters|Items|Triggers)/.*\\.lsf$";
    Regex::new(v).unwrap_or_else(|_| panic!("Failed to compile regex: {}", v))
});
// Pattern for excluding subsequent parts of a multi-part archive
const ARCHIVE_PART_RE: Lazy<Regex> = Lazy::new(|| {
    let v = "(?i)^(.*)_[0-9]+\\.pak$";
    Regex::new(v).unwrap_or_else(|_| panic!("Failed to compile regex: {}", v))
});

#[derive(Debug)]
pub enum ModPathVisitError {
    Io(std::io::Error),
    EnumerateFilesPrefix,
}
impl From<std::io::Error> for ModPathVisitError {
    fn from(e: std::io::Error) -> Self {
        ModPathVisitError::Io(e)
    }
}

#[derive(Debug)]
pub enum ModPathDiscoverError {
    Visit(ModPathVisitError),
    Package(PackageError),
}

pub struct ModPathVisitor {
    resources: ModResources,
    pub collect_story_goals: bool,
    pub collect_stats: bool,
    pub collect_globals: bool,
    pub collect_levels: bool,
    pub load_packages: bool,
    pub target_game: TargetGame,
}
impl ModPathVisitor {
    pub fn new(resources: ModResources) -> ModPathVisitor {
        ModPathVisitor {
            resources,
            collect_story_goals: false,
            collect_stats: false,
            collect_globals: false,
            collect_levels: false,
            load_packages: true,
            target_game: TargetGame::DOS2,
        }
    }

    // TODO: I have an intuition that we could make this into an iterator of some sort to avoid allocating the vector
    /// Enumerate the files  
    fn enumerate_files(
        &self,
        paths: &mut Vec<String>,
        root_path: &Path,
        current_path: &Path,
        ext: &str,
    ) -> Result<(), ModPathVisitError> {
        for file_path in exts_in_dir(current_path, ext)? {
            let file_path = file_path?;

            // TODO: is this correct? The original does this via length and doesn't check it at all
            let relative_path = file_path
                .strip_prefix(root_path)
                .map_err(|_| ModPathVisitError::EnumerateFilesPrefix)?
                .to_str()
                .unwrap();

            // TODO: original removes prefix '/' or '\\', does that matter?

            paths.push(relative_path.to_owned());
        }

        let dirs = std::fs::read_dir(current_path)?.filter(|d| {
            if let Ok(d) = &d {
                if let Ok(file_type) = d.file_type() {
                    file_type.is_dir()
                } else {
                    false
                }
            } else {
                false
            }
        });
        for dir in dirs {
            let dir = dir?;
            let dir_path = dir.path();
            self.enumerate_files(paths, root_path, &dir_path, ext)?;
        }

        Ok(())
    }

    fn get_mod(&mut self, mod_name: &str) -> Option<&ModInfo> {
        self.resources.mods.get(mod_name)
    }

    /// Get a mod mutably, initializing it if it doesn't exist.
    fn get_mod_init_mut(&mut self, mod_name: &str) -> &mut ModInfo {
        if !self.resources.mods.contains_key(mod_name) {
            let mod_d = ModInfo::new(mod_name.to_string());
            self.resources.mods.insert(mod_name.to_string(), mod_d);
        }

        self.resources.mods.get_mut(mod_name).unwrap()
    }

    fn add_metadata_to_mod(&mut self, mod_name: &str, file: FileInfo) {
        self.get_mod_init_mut(mod_name).meta = Some(file);
    }

    fn add_stat_to_mod(&mut self, mod_name: &str, path: String, file: FileInfo) {
        self.get_mod_init_mut(mod_name).stats.insert(path, file);
    }

    fn add_script_to_mod(&mut self, mod_name: &str, script_name: String, file: FileInfo) {
        self.get_mod_init_mut(mod_name)
            .scripts
            .insert(script_name, file);
    }

    fn add_globals_to_mod(&mut self, mod_name: &str, path: String, file: FileInfo) {
        self.get_mod_init_mut(mod_name).globals.insert(path, file);
    }

    fn add_level_objects_to_mod(&mut self, mod_name: &str, path: String, file: FileInfo) {
        self.get_mod_init_mut(mod_name)
            .level_objects
            .insert(path, file);
    }

    fn discover_packaged_file<F: FileInfoLike + Clone>(&mut self, file: &F) {
        if file.is_deletion() {
            return;
        }

        let name = file.name();

        if name.ends_with("meta.lsx") {
            let mod_name = META_RE.captures(name).and_then(|m| m.get(1));
            if let Some(mod_name) = mod_name {
                let mod_name = mod_name.as_str();
                self.add_metadata_to_mod(mod_name, file.clone().into());
            }
        }

        if self.collect_story_goals {
            if name.ends_with(".txt") && name.contains("/Story/RawFiles/Goals") {
                if let Some(match_) = SCRIPT_RE.captures(name) {
                    // TODO: Is it fine to panic?
                    let mod_name = match_.get(1).unwrap().as_str();
                    let script_name = match_.get(2).unwrap().as_str();

                    self.add_script_to_mod(mod_name, script_name.to_string(), file.clone().into());
                }
            }

            if name.ends_with("/Story/story_orphanqueries_ignore_local.txt") {
                let mod_name = ORPHAN_QUERY_IGNORES_RE
                    .captures(name)
                    .and_then(|m| m.get(1));
                if let Some(mod_name) = mod_name {
                    let mod_name = mod_name.as_str();

                    self.get_mod_init_mut(mod_name).orphan_query_ignore_list =
                        Some(file.clone().into());
                }
            }

            if name.ends_with("/Story/RawFiles/story_header.div") {
                let mod_name = STORY_DEFINITIONS_RE.captures(name).and_then(|m| m.get(1));
                if let Some(mod_name) = mod_name {
                    let mod_name = mod_name.as_str();

                    self.get_mod_init_mut(mod_name).story_header_file = Some(file.clone().into());
                }
            }

            if name.ends_with("/Story/RawFiles/TypeCoercionWhitelist.txt") {
                let mod_name = TYPE_COERCION_WHITELIST_RE
                    .captures(name)
                    .and_then(|m| m.get(1));
                if let Some(mod_name) = mod_name {
                    let mod_name = mod_name.as_str();

                    self.get_mod_init_mut(mod_name).type_coercion_whitelist_file =
                        Some(file.clone().into());
                }
            }
        }

        if self.collect_stats {
            if name.ends_with(".txt") && name.contains("/Stats/Generated/Data") {
                if let Some(match_) = STAT_RE.captures(name) {
                    let mod_name = match_.get(1).unwrap().as_str();
                    let stat_name = match_.get(2).unwrap().as_str();

                    self.add_stat_to_mod(mod_name, stat_name.to_string(), file.clone().into());
                }
            }
        }

        if self.collect_globals {
            if name.ends_with(".lsf") && name.contains("/Globals/") {
                if let Some(match_) = GLOBALS_RE.captures(name) {
                    let mod_name = match_.get(1).unwrap().as_str();
                    let path = match_.get(0).unwrap().as_str();

                    self.add_globals_to_mod(mod_name, path.to_string(), file.clone().into());
                }
            }
        }

        if self.collect_levels {
            if name.ends_with(".lsf") && name.contains("/Levels/") {
                if let Some(match_) = LEVEL_OBJECTS_RE.captures(name) {
                    let mod_name = match_.get(1).unwrap().as_str();
                    let path = match_.get(0).unwrap().as_str();

                    self.add_level_objects_to_mod(mod_name, path.to_string(), file.clone().into());
                }
            }
        }
    }

    pub fn discover_package(&mut self, package_path: &Path) -> Result<(), PackageError> {
        let file = std::fs::File::open(&package_path)?;
        let file = std::io::BufReader::new(file);

        let package = read_package(file, package_path, false)?;

        for file in &package.files {
            self.discover_packaged_file(file);
        }

        self.resources.loaded_packages.push(package);

        Ok(())
    }

    pub fn discover_builtin_packages(&mut self, game_data_path: &Path) -> Result<(), PackageError> {
        // List of packages we won't ever load
        // These packages don't contain any mod resources, but do have a large file table
        // which would be a waste of time to load.
        const PASSAGE_BLACKLIST: &[&str] = &[
            "Assets.pak",
            "Effects.pak",
            "Engine.pak",
            "EngineShaders.pak",
            "Game.pak",
            "GamePlatform.pak",
            "Gustav_Textures.pak",
            "Icons.pak",
            "LowTex.pak",
            "Materials.pak",
            "Minimaps.pak",
            "Models.pak",
            "SharedSoundBanks.pak",
            "SharedSounds.pak",
            "Textures.pak",
            "VirtualTextures.pak",
        ];

        // Collect priority value from headers
        let mut package_priorities: Vec<(PathBuf, u8)> = Vec::new();

        for path in exts_in_dir(game_data_path, "pak")? {
            let path = path?;

            let base_name = path.file_name().and_then(|b| b.to_str());
            if let Some(base_name) = base_name {
                if PASSAGE_BLACKLIST.contains(&base_name) {
                    continue;
                }

                // Don't load 2nd, 4rd, etc, parts of a multi-part archive
                if !ARCHIVE_PART_RE.is_match(base_name) {
                    continue;
                }

                let file = std::fs::File::open(&path)?;
                let file = std::io::BufReader::new(file);

                let package = read_package(file, &path, true)?;

                package_priorities.push((path, package.metadata.priority))
            }
        }

        package_priorities.sort_by_key(|(_, priority)| *priority);

        // Load non-patch packages first
        for (path, _) in package_priorities {
            self.discover_package(&path)?;
        }

        Ok(())
    }

    pub fn discover_user_packages(&mut self, game_data_path: &Path) -> Result<(), PackageError> {
        for path in exts_in_dir(game_data_path, "pak")? {
            let path = path?;

            // TODO: don't panic
            let path_str = path.to_str().expect("Non-UTF8 path found");

            // Don't load 2nd, 4rd, etc, parts of a multi-part archive
            if !ARCHIVE_PART_RE.is_match(path_str) {
                continue;
            }

            self.discover_package(&path)?;
        }

        Ok(())
    }

    fn discover_mod_goals(
        &mut self,
        mod_name: &str,
        mod_path: &Path,
    ) -> Result<(), ModPathVisitError> {
        let goal_path = mod_path.join("Story/RawFiles/Goals");

        if !goal_path.is_dir() {
            return Ok(());
        }

        let mut goal_files = Vec::new();

        self.enumerate_files(&mut goal_files, &goal_path, &goal_path, "txt")?;

        for goal_file in goal_files {
            let path = goal_path.join(&goal_file);
            let file_info = FilesystemFileInfo::new(path, goal_file.clone());

            self.add_script_to_mod(mod_name, goal_file, file_info.into());
        }

        Ok(())
    }

    fn discover_mod_stats(
        &mut self,
        mod_name: &str,
        mod_public_path: &Path,
    ) -> Result<(), ModPathVisitError> {
        let stats_path = mod_public_path.join("Stats/Generated/Data");
        if !stats_path.is_dir() {
            return Ok(());
        }

        let mut stat_files = Vec::new();
        self.enumerate_files(&mut stat_files, &stats_path, &stats_path, "txt")?;

        for stat_file in stat_files {
            let file_info = FilesystemFileInfo::new(stats_path.join(&stat_file), stat_file.clone());

            self.add_stat_to_mod(mod_name, stat_file, file_info.into());
        }

        Ok(())
    }

    fn discover_mod_globals(
        &mut self,
        mod_name: &str,
        mod_path: &Path,
    ) -> Result<(), ModPathVisitError> {
        let globals_path = mod_path.join("Globals");
        if !globals_path.is_dir() {
            return Ok(());
        }

        let mut global_files = Vec::new();
        self.enumerate_files(&mut global_files, &globals_path, &globals_path, "lsf")?;

        for global_file in global_files {
            let file_info =
                FilesystemFileInfo::new(globals_path.join(&global_file), global_file.clone());

            self.add_globals_to_mod(mod_name, global_file, file_info.into());
        }

        Ok(())
    }

    fn discover_mod_level_objects(
        &mut self,
        mod_name: &str,
        mod_path: &Path,
    ) -> Result<(), ModPathVisitError> {
        let levels_path = mod_path.join("Levels");
        if !levels_path.is_dir() {
            return Ok(());
        }

        let mut level_files = Vec::new();
        self.enumerate_files(&mut level_files, &levels_path, &levels_path, "lsf")?;

        for level_file in level_files {
            let file_info =
                FilesystemFileInfo::new(levels_path.join(&level_file), level_file.clone());

            self.add_level_objects_to_mod(mod_name, level_file, file_info.into());
        }

        Ok(())
    }

    pub fn discover_mod_directory(
        &mut self,
        mod_name: &str,
        mod_path: &Path,
        public_path: &Path,
    ) -> Result<(), ModPathVisitError> {
        self.get_mod_init_mut(mod_name);

        if self.collect_story_goals {
            self.discover_mod_goals(mod_name, mod_path)?;

            let header_path = mod_path.join("/Story/RawFiles/story_header.div");
            if header_path.is_file() {
                let header_path_text = header_path.to_string_lossy().to_string();
                let file_info = FilesystemFileInfo::new(header_path, header_path_text);
                self.get_mod_init_mut(mod_name).story_header_file = Some(file_info.into());
            }

            let orphan_query_ignores_path =
                mod_path.join("/Story/story_orphanqueries_ignore_local.txt");
            if orphan_query_ignores_path.is_file() {
                let orphan_query_ignores_path_text =
                    orphan_query_ignores_path.to_string_lossy().to_string();
                let file_info = FilesystemFileInfo::new(
                    orphan_query_ignores_path,
                    orphan_query_ignores_path_text,
                );
                self.get_mod_init_mut(mod_name).orphan_query_ignore_list = Some(file_info.into());
            }

            let type_coercion_whitelist_path =
                mod_path.join("/Story/RawFiles/TypeCoercionWhitelist.txt");
            if type_coercion_whitelist_path.is_file() {
                let type_coercion_whitelist_path_text =
                    type_coercion_whitelist_path.to_string_lossy().to_string();
                let file_info = FilesystemFileInfo::new(
                    type_coercion_whitelist_path,
                    type_coercion_whitelist_path_text,
                );
                self.get_mod_init_mut(mod_name).type_coercion_whitelist_file =
                    Some(file_info.into());
            }
        }

        if self.collect_stats {
            self.discover_mod_stats(mod_name, public_path)?;
        }

        if self.collect_globals {
            self.discover_mod_globals(mod_name, mod_path)?;
        }

        if self.collect_levels {
            self.discover_mod_level_objects(mod_name, mod_path)?;
        }

        Ok(())
    }

    pub fn discover_mods(&mut self, game_data_path: &Path) -> Result<(), ModPathVisitError> {
        let mods_path = game_data_path.join("Mods");
        let public_path = game_data_path.join("Public");

        if !mods_path.is_dir() {
            return Ok(());
        }

        let mod_paths = std::fs::read_dir(&mods_path)?;
        for entry in mod_paths {
            let entry = entry?;

            let mod_path = entry.path();

            let meta_path = mod_path.join("meta.lsx");
            if !meta_path.is_file() {
                continue;
            }

            let mod_name = mod_path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap_or_else(|| panic!("Found non-UTF8 path for mod: {mod_path:?}"));
            let mod_filename = mod_path.file_name().unwrap();
            let mod_public_path = public_path.join(mod_filename);

            self.discover_mod_directory(mod_name, &mod_path, &mod_public_path)?;
        }

        Ok(())
    }

    pub fn discover(&mut self, game_data_path: &Path) -> Result<(), ModPathDiscoverError> {
        if self.load_packages {
            self.discover_builtin_packages(game_data_path)
                .map_err(ModPathDiscoverError::Package)?;
        }

        self.discover_mods(game_data_path)
            .map_err(ModPathDiscoverError::Visit)
    }
}

/// Get the list of paths with a specific extension in a directory  
/// Extension does not include '.'
fn exts_in_dir<'a>(
    dir: &Path,
    ext: &'a str,
) -> std::io::Result<impl Iterator<Item = std::io::Result<PathBuf>> + 'a> {
    Ok(std::fs::read_dir(dir)?
        .map(|f| f.map(|f| f.path()))
        .filter(move |p| {
            p.as_ref()
                .map(move |p| p.extension() == Some(ext.as_ref()))
                .unwrap_or(true)
        }))
}
