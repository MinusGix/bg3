use binrw::{io::TakeSeekExt, until_eof, BinRead};
use lsf::{lsx::parse_lsx, parse_lsf};

fn main() {
    // test_reading_lsf();
    test_reading_lsx();
}

fn test_reading_lsf() {
    // TODO: _merged.lsf had an `has_sibling_data` of 2?
    // Probably has_sibling_data is several flags, since it is a u32.
    // so 0b1 would be has sibling data
    // and 0b10 would be something else
    // let data = std::fs::read("_merged.lsf").unwrap();
    // let data = std::fs::read("Resources.lsf").unwrap();
    // let data = std::fs::read("/run/media/minus/Kairos/Games/Modding/BG3/Out/Gustav/Public/Gustav/Tags/ffd08582-7396-4cac-bcd4-8f9cd0fd8ef3.lsf").unwrap();
    // let data = std::fs::read("/run/media/minus/Kairos/Games/Modding/BG3/Out/Gustav/Public/Gustav/Content/Assets/Dialogs/Act1/[PAK]_Chapel/_merged.lsf").unwrap();
    let data = std::fs::read("/run/media/minus/Kairos/Games/Modding/BG3/Out/Gustav/Mods/Gustav/Localization/Act0_Books.lsf").unwrap();
    let lsf = parse_lsf(&data).unwrap();
    println!("{:#?}", lsf);

    println!("\nNodes:");
    for node in lsf.nodes.nodes.nodes.iter() {
        println!("{:#?}", node);
        let name_index = node.name_index();
        let name_offset = node.name_offset();

        let name = lsf
            .name(name_index, name_offset)
            .expect("Failed to get nodes name");
        let name = name.as_str().expect("Node's name was not valid utf8");
        println!("Name: {}", name);
        println!();
    }

    println!("\n=== Attributes ===");
    for attr in lsf.attributes.attrs.attrs.iter() {
        let name_index = attr.name_index();
        let name_offset = attr.name_offset();

        let name = lsf
            .name(name_index, name_offset)
            .expect("Failed to get attributes name");

        let name = name.as_str().expect("Attribute's name was not valid utf8");
        println!("\tName: {}", name);
    }
}

fn test_reading_lsx() {
    // let path = "/run/media/minus/Kairos/Games/Modding/BG3/Out/Shared/Public/Shared/Gods/Gods.lsx";
    // let path =
    // "/run/media/minus/Kairos/Games/Modding/BG3/Out/Gustav/Public/Gustav/Voices/Voices.lsx";
    let path =
        "/run/media/minus/Kairos/Games/Modding/BG3/Out/Gustav/Public/Gustav/Timeline/Generated/";
    let files = std::fs::read_dir(path).unwrap();
    for entry in files {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            continue;
        }
        let path = entry.path();

        if path.extension() != Some("lsx".as_ref()) {
            continue;
        }

        println!("Path: {:?}", path);
        let data = std::fs::read_to_string(path).unwrap();
        let lsx = parse_lsx(&data).unwrap();
    }
    // println!("{:#?}", lsx);
}

// TODO: lsf -> lsx
// TODO: lsx -> lsf
// TODO: more post processed version of lsx/lsf for getting information in a nicer manner
