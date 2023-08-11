use lsj::{parse_lsj, LSJ};

fn main() {
    // let path = "/run/media/minus/Kairos/Games/Modding/BG3/Out/Gustav/Mods/Gustav/Story/Dialogs/Tutorial/TUT_Start_Laezel.lsj";
    let path = "/run/media/minus/Kairos/Games/Modding/BG3/Out/Gustav/Mods/Gustav/Story/Dialogs/Tutorial/TUT_Lab_DevourerFollower.lsj";
    let data = std::fs::read_to_string(path).unwrap();

    let lsj = parse_lsj(&data).unwrap();
    println!("{:#?}", lsj);

    print_essentials(&lsj);
}

fn print_essentials(lsj: &LSJ) {
    let regions = &lsj.save.regions;

    let synopsis = regions.editor_data.synopsis.as_ref();

    println!("Synopsis: {}", synopsis);

    for node_e in regions.dialog.nodes.iter() {
        for node in node_e.node.iter() {
            let editor_data = node.editor_data.first().unwrap();
            let editor_data = &editor_data.data;
            // TODO: we could have an 'is::<T>` and `as::<T>` functions?
            for d in editor_data.iter() {
                let key = d.key.as_ref();
                if key == "CinematicNodeContext"
                    || key == "NodeContext"
                    || key == "InternalNodeContext"
                {
                    let val = d.val.as_ref();
                    if !val.is_empty() {
                        println!("{}: {}", key, val);
                    }
                }
            }
        }
    }
}
