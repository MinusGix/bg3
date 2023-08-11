use loca::parse_loca;
use lsj::parse_lsj;

fn main() {
    let english_loca_path = "/run/media/minus/Kairos/Games/Modding/BG3/Out/Localization/English/Localization/English/english.loca";

    let dialogue_path = "/run/media/minus/Kairos/Games/Modding/BG3/Out/Gustav/Mods/Gustav/Story/Dialogs/Tutorial/TUT_Lab_DevourerFollower.lsj";

    let english_loca =
        std::fs::read(english_loca_path).expect("Failed to read english localization file");
    let english_loca =
        parse_loca(&english_loca).expect("Failed to parse english localization files");

    let dialogue = std::fs::read_to_string(dialogue_path).expect("Failed to read dialogue file");
    let dialogue = parse_lsj(&dialogue).expect("Failed to parse dialogue file");

    {
        let d = &dialogue;
        let regions = &d.save.regions;

        let synopsis = regions.editor_data.synopsis.as_ref();

        println!("Synopsis: {}", synopsis);

        let di = &regions.dialog;

        let default_speaker_index = di.default_speaker_index; // ??
        let speaker_list = &di.speaker_list;

        if default_speaker_index.0 != -1 {
            println!(
                "Default speaker index was not -1, maybe check the speaker list for who it is?"
            );
        }

        for speaker in speaker_list.iter() {
            for speaker in speaker.speaker.iter() {
                let list = speaker.list.as_ref();
                // speakers aren't in loca
                // if let Some(val) = english_loca.get_str(list) {
                //     println!("Speaker: {}", val);
                // } else {
                //     println!("Speaker: ??? {}", list);
                // }
            }
        }

        for nodes_v in di.nodes.iter() {
            for root_nodes in nodes_v.root_nodes.iter() {
                // The initial(?) node uuid
                let root_node = root_nodes.root_nodes.as_ref();
                println!("Root Node => ({root_node})");

                let node = nodes_v
                    .node
                    .iter()
                    .find(|x| x.uuid.as_ref() == root_node)
                    .expect("Failed to get root node");

                println!("\n  Node         ({})", node.uuid.as_ref());

                // Get meta info from editor data
                for v in node.editor_data.iter() {
                    for v in v.data.iter() {
                        let key = v.key.as_ref();
                        if key == "CinematicNodeContext"
                            || key == "NodeContext"
                            || key == "InternalNodeContext"
                        {
                            if !v.val.as_ref().is_empty() {
                                println!("    {}: {:?}", key, v.val.as_ref());
                            }
                        }
                    }
                }

                // TODO: constructor? That might just affect how the scene looks.

                let tagged_text = node
                    .tagged_texts
                    .as_ref()
                    .into_iter()
                    .map(|x| x.iter())
                    .flatten()
                    // TODO: should we take care to look at these as separate vecs?
                    .map(|t| t.tagged_text.iter())
                    .flatten();
                for tagged in tagged_text {
                    // TODO: has tag rule?
                    // TODO: rule group parsing probably decides how things appear?

                    let tag_texts = tagged.tag_texts.iter().map(|x| x.tag_text.iter()).flatten();
                    // TODO: line id
                    // TODO: rule group might be how it decides how things appear?
                    for tag_text in tag_texts {
                        let text_ref = tag_text.tag_text.as_ref();
                        let text = english_loca
                            .get_str(text_ref)
                            .expect("Failed to get tag text");

                        println!("    - {text_ref}:  {text:?}");
                    }
                }
            }
        }
    }
}
