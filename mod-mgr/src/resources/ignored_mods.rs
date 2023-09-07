use mod_mgr_lib::mod_data::ModData;

/// GUIDS pf dependencies that should be ignored
pub const IGNORE_DEPENDENCIES: &[&str] = &[
    "e842840a-2449-588c-b0c4-22122cfce31b",
    "b176a0ac-d79f-ed9d-5a87-5c2c80874e10",
    "e0a4d990-7b9b-8fa9-d7c6-04017c6cf5b1",
    "ed539163-bb70-431b-96a7-f5b2eda5376b",
    "3d0c5ff8-c95d-c907-ff3e-34b204f1c630",
    "991c9c7a-fb80-40cb-8f0d-b92d4e80e9b1",
    "e5c9077e-1fca-4f24-b55d-464f512c98a8",
    "9dff4c3b-fda7-43de-a763-ce1383039999",
];

pub const IGNORE_BUILTIN_PATH: &[&str] = &["Game/GUI/Assets/Tooltips"];

// TODO: if these could be static then we wouldn't need to clone them a couple times.
/// Get the default ignored mods.  
/// This is an iterator rather than a global slice because we can't nicely call
/// `ModData::new` in a const-context.
pub fn ignored_mods_iter() -> impl Iterator<Item = ModData> {
    [
        ModData::new(
            "Gustav",
            "991c9c7a-fb80-40cb-8f0d-b92d4e80e9b1",
            "Gustav",
            36029301681017806,
            "Adventure",
            "Story",
            "Larian Studios",
            "The main story campaign dependency.",
            "Campaign",
        ),
        ModData::new(
            "GustavDev",
            "28ac9ce2-2aba-8cda-b3b5-6e922f71b6b8",
            "GustavDev",
            144115617576214574,
            "Adventure",
            "Story",
            "Larian Studios",
            "The main story campaign.",
            "Campaign",
        ),
        ModData::new(
            "Shared",
            "ed539163-bb70-431b-96a7-f5b2eda5376b",
            "Shared",
            36029297386049870,
            "Add-on",
            "Story",
            "Larian Studios",
            "",
            "",
        ),
        ModData::new(
            "SharedDev",
            "3d0c5ff8-c95d-c907-ff3e-34b204f1c630",
            "SharedDev",
            36028797022575353,
            "Adventure",
            "Story",
            "Larian Studios",
            "",
            "",
        ),
        ModData::new(
            "FW3",
            "e5c9077e-1fca-4f24-b55d-464f512c98a8",
            "FW3",
            268435456,
            "Add-on",
            "Story",
            "Larian Studios",
            "Shared project",
            "",
        ),
        ModData::new(
            "Engine",
            "9dff4c3b-fda7-43de-a763-ce1383039999",
            "Engine",
            36028797018963968,
            "Add-on",
            "Story",
            "Larian Studios",
            "",
            "",
        ),
        ModData::new(
            "Game",
            "Game",
            "Game",
            36028797018963968,
            "Add-on",
            "Story",
            "Larian Studios",
            "Not an actual mod. This is the Public/Game folder.",
            "",
        ),
        ModData::new(
            "DiceSet_01",
            "e842840a-2449-588c-b0c4-22122cfce31b",
            "DiceSet_01",
            36028797018963968,
            "Add-on",
            "Story",
            "Larian Studios",
            "",
            "",
        ),
        ModData::new(
            "DiceSet_02",
            "b176a0ac-d79f-ed9d-5a87-5c2c80874e10",
            "DiceSet_02",
            36028797018963968,
            "Add-on",
            "Story",
            "Larian Studios",
            "",
            "",
        ),
        ModData::new(
            "DiceSet_03",
            "e0a4d990-7b9b-8fa9-d7c6-04017c6cf5b1",
            "DiceSet_03",
            36028797018963968,
            "Add-on",
            "Story",
            "Larian Studios",
            "",
            "",
        ),
    ]
    .into_iter()
}
