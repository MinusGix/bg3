pub mod mod_data;
pub mod settings;
pub mod util;

pub const BG3_STEAM_ID: u32 = 1086940;

pub const DIR_DATA: &str = "Data/";

pub const URL_REPO: &str = "todo";
pub const ORIGINAL_URL_REPO: &str = "https://github.com/LaughingLeader/BG3ModManager";

pub fn xml_mod_order_module(uuid: &str) -> String {
    format!(
        r#"<node id=""Module""><attribute id=""UUID"" value=""{}"" type=""FixedString""/></node>"#,
        uuid
    )
}

pub fn xml_module_short_desc(
    folder: &str,
    md5: &str,
    name: &str,
    uuid: &str,
    version64: i64,
) -> String {
    format!(
        r#"<node id=""ModuleShortDesc""><attribute id=""Folder"" value=""{}"" type=""LSString""/><attribute id=""MD5"" value=""{}"" type=""LSString""/><attribute id=""Name"" value=""{}"" type=""LSString""/><attribute id=""UUID"" value=""{}"" type=""FixedString"" /><attribute id=""Version64"" value=""{}"" type=""int64""/></node>"#,
        folder, md5, name, uuid, version64
    )
}

pub fn xml_mod_settings_template(mod_order: &str, mods: &str) -> String {
    format!(
        r#"<?xml version=""1.0"" encoding=""UTF-8""?><save><version major=""4"" minor=""0"" revision=""9"" build=""331""/><region id=""ModuleSettings""><node id=""root""><children><node id=""ModOrder""><children>{}</children></node><node id=""Mods""><children>{}</children></node></children></node></region></save>"#,
        mod_order, mods
    )
}

pub const MAIN_CAMPAIGN_UUID: &str = "28ac9ce2-2aba-8cda-b3b5-6e922f71b6b8";
pub const GAMEMASTER_UUID: &str = "NotYetAvailableInBG3";

pub const EXTENDER_REPO_URL: &str = "Norbyte/bg3se";
pub const EXTENDER_LATEST_URL: &str = "https://github.com/Norbyte/bg3se/releases/latest";
pub const EXTENDER_APPDATA_URL: &str = "BG3ScriptExtender/OsiExtenderEoCApp";
pub const EXTENDER_MOD_CONFIG: &str = "Config.json";
pub const EXTENDER_UPDATER_FILE: &str = "DWrite.dll";
