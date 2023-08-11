pub mod util;
pub mod val;

use serde::{Deserialize, Serialize};
use util::{PanicDeser, VecOrEmpty};
use val::{FixedString, Guid, LSString, StringKind, Val, ValType};

// TODO: wtf is a peanut

pub fn parse_lsj(text: &str) -> Result<LSJ, serde_json::Error> {
    let j = serde_json::from_str(text)?;
    Ok(j)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LSJ {
    pub save: Save,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Save {
    pub header: Header,
    pub regions: Regions,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Header {
    /// Ex: "4.0.8.609"
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Regions {
    pub dialog: Dialog,
    pub editor_data: DialogueEditorData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Dialog {
    #[serde(rename = "AllowDeadSpeakers")]
    pub allow_dead_speakers: Val<bool>,
    /// Presumably whoever gets talked to? -1 might be unset? unsure. It has a 0 and 1 key
    #[serde(rename = "DefaultAddressedSpeakers")]
    pub default_addressed_speakers: Vec<ObjectList>,
    /// Presumably -1 for unset/default/auto
    #[serde(rename = "DefaultSpeakerIndex")]
    pub default_speaker_index: Val<i32>,
    #[serde(rename = "IsAllowingJoinCombat")]
    pub is_allowing_join_combat: Val<bool>,
    /// Default: ?
    #[serde(rename = "IsBehaviour")]
    pub is_behaviour: Option<Val<bool>>,
    /// Default: ?
    #[serde(rename = "IsPrivateDialog")]
    pub is_private_dialog: Option<Val<bool>>,
    #[serde(rename = "IsSubbedDialog")]
    pub is_subbed_dialog: Val<bool>,
    #[serde(rename = "IsWorld")]
    pub is_world: Val<bool>,
    /// Maybe timeline is 'history of dialogue' actions?  
    /// Seems to be a UUID
    #[serde(rename = "TimelineId")]
    pub timeline_id: Val<FixedString>,
    // TODO: special type for things like this?
    /// Presumably the ID of this specific dialogue
    #[serde(rename = "UUID")]
    pub uuid: Val<FixedString>,
    pub automated: Val<bool>,
    // TODO: enum for common categories?
    /// Ex: "Generic NPC Dialog"
    pub category: Val<LSString>,
    pub issfxdialog: Val<bool>,
    pub nodes: Vec<Nodes>,
    #[serde(rename = "speakerlist")]
    pub speaker_list: Vec<Speakers>,
}

// TODO: There's this really wacky pattern in lsj files like:
// ```json
// "RootNodes" : [
//   {
//     "RootNodes" : {"type" : "FixedString", "value" : "805168c8-b27d-42e2-8f00-49321e82d837"}
//   }
// ],
// ```
// Where the value only ever has a single thing and then has a child with the same field name??

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Nodes {
    #[serde(rename = "RootNodes")]
    pub root_nodes: Vec<RootNodesContainer>,
    pub node: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RootNodesContainer {
    /// Seems to be a UUID
    #[serde(rename = "RootNodes")]
    pub root_nodes: Val<FixedString>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Node {
    /// Default: ?
    #[serde(rename = "AllowNodeGrouping")]
    pub allow_node_grouping: Option<Val<bool>>,
    #[serde(rename = "GameData")]
    pub game_data: Option<Vec<GameData>>,
    /// Default: ?
    #[serde(rename = "Greeting")]
    pub greeting: Option<Val<bool>>,
    #[serde(rename = "GroupID")]
    pub group_id: Val<FixedString>,
    #[serde(rename = "GroupIndex")]
    pub group_index: Val<i32>,
    #[serde(rename = "Root")]
    pub root: Val<bool>,
    #[serde(rename = "ShowOnce")]
    pub show_once: Val<bool>,
    #[serde(rename = "TaggedTexts")]
    pub tagged_texts: Option<Vec<TaggedTextCont>>,
    #[serde(rename = "Tags")]
    pub tags: VecOrEmpty<TagCont>,
    #[serde(rename = "UUID")]
    pub uuid: Val<FixedString>,
    #[serde(rename = "ValidatedFlags")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub validated_flags: Vec<ValidatedFlag>,
    #[serde(rename = "addressedspeaker")]
    pub addressed_speaker: Option<Val<i32>>,
    #[serde(rename = "checkflags")]
    pub check_flags: VecOrEmpty<FlagCollection>,
    // Child nodes?
    pub children: VecOrEmpty<NodeChildCont>,
    pub constructor: Val<FixedString>,
    #[serde(rename = "editorData")]
    pub editor_data: Vec<DataList>,
    #[serde(rename = "endnode")]
    pub end_node: Val<bool>,
    #[serde(rename = "exclusive")]
    pub exclusive: Val<bool>,
    #[serde(rename = "gameplaynode")]
    pub gameplay_node: Val<bool>,
    /// Maybe the target for camera? Unsure
    #[serde(rename = "jumptarget")]
    pub jump_target: Option<Val<FixedString>>,
    /// Enum? Character index?
    #[serde(rename = "jumptargetpoint")]
    pub jump_target_point: Option<Val<u8>>,
    #[serde(rename = "optional")]
    pub optional: Val<bool>,
    pub setflags: VecOrEmpty<FlagCollection>,
    #[serde(rename = "speaker")]
    pub speaker: Option<Val<i32>>,
    // Default: ?
    #[serde(rename = "stub")]
    pub stub: Option<Val<bool>>,
    #[serde(rename = "suppresssubtitle")]
    pub suppress_subtitle: Val<bool>,
    #[serde(rename = "transitionmode")]
    pub transition_mode: Val<u8>,
    #[serde(rename = "waittime")]
    pub wait_time: Val<f32>,
}

// This json is evil.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TagCont {
    #[serde(rename = "Tag")]
    pub tag: Vec<Tag>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tag {
    #[serde(rename = "Tag")]
    pub tag: Val<Guid>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValidatedFlag {
    #[serde(rename = "ValidatedHasValue")]
    pub validated_has_value: Val<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FlagCollection {
    #[serde(rename = "flaggroup")]
    pub flag_group: Vec<FlagGroup>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FlagGroup {
    #[serde(rename = "flag")]
    pub flag: Vec<Flag>,
    #[serde(rename = "type")]
    pub typ: Val<FixedString>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Flag {
    #[serde(rename = "UUID")]
    pub uuid: Val<FixedString>,
    // TODO: what is the difference between paramval and value?
    #[serde(rename = "paramval")]
    pub param_val: Option<Val<i32>>,
    pub value: Val<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameData {
    #[serde(rename = "AiPersonalities")]
    pub ai_personalities: VecOrEmpty<PanicDeser>,
    /// -1 for unset/default/auto?
    #[serde(rename = "CameraTarget")]
    pub camera_target: Val<i32>,
    #[serde(rename = "CustomMovie")]
    pub custom_movie: Val<LSString>,
    #[serde(rename = "ExtraWaitTime")]
    pub extra_wait_time: Val<i32>,
    #[serde(rename = "MusicInstrumentSounds")]
    pub music_instrument_sounds: VecOrEmpty<PanicDeser>,
    #[serde(rename = "OriginSound")]
    pub origin_sound: VecOrEmpty<PanicDeser>,
    #[serde(rename = "SoundEvent")]
    pub sound_event: Val<LSString>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodeChildCont {
    pub child: Vec<NodeChild>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodeChild {
    #[serde(rename = "UUID")]
    pub uuid: Val<FixedString>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaggedTextCont {
    #[serde(rename = "TaggedText")]
    pub tagged_text: Vec<TaggedText>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaggedText {
    #[serde(rename = "HasTagRule")]
    pub has_tag_rule: Val<bool>,
    #[serde(rename = "RuleGroup")]
    pub rule_group: Vec<RuleGroup>,
    #[serde(rename = "TagTexts")]
    pub tag_texts: Vec<TagTextsCont>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuleGroup {
    #[serde(rename = "Rules")]
    pub rules: VecOrEmpty<PanicDeser>,
    #[serde(rename = "TagCombineOp")]
    pub tag_combine_op: Val<u8>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TagTextsCont {
    #[serde(rename = "TagText")]
    pub tag_text: Vec<TagText>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TagText {
    #[serde(rename = "LineId")]
    pub line_id: Val<Guid>,
    // TODO: Can this be any string or just translated string?
    #[serde(rename = "TagText")]
    pub tag_text: Val<StringKind>,
    pub stub: Val<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Speakers {
    pub speaker: Vec<Speaker>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Speaker {
    #[serde(rename = "IsPeanutSpeaker")]
    pub is_peanut_speaker: Val<bool>,
    #[serde(rename = "SpeakerMappingId")]
    pub speaker_mapping_id: Val<Guid>,
    // TODO: this was an empty string on the file I was looking at, but given 'index' I wouldn't be surprised if that was some weird default and it is actually an i32 most of the time or something...
    #[serde(rename = "SpeakerTagsIndex")]
    pub speaker_tags_index: Val<LSString>,
    // TODO: seems to be a stringy index, should we have a type for that?
    pub index: Val<FixedString>,
    /// Seems to be a UUID
    pub list: Val<LSString>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DialogueEditorData {
    #[serde(rename = "HowToTrigger")]
    pub how_to_trigger: Val<StringKind>,
    pub default_attitudes: Vec<EditorDataAttitudes>,
    pub default_emotions: Vec<EditorDataEmotions>,
    pub is_important_for_stagings: VecOrEmpty<PanicDeser>,
    // TODO: Seems to be stringy bools, should we have a type for that?
    pub is_peanuts: Vec<DataList>,
    pub need_layout: Val<bool>,
    pub next_node_id: Val<u32>,
    pub speaker_slot_description: VecOrEmpty<PanicDeser>,
    pub synopsis: Val<LSString>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObjectList<K: ValType = i32, V: ValType = i32> {
    #[serde(rename = "Object")]
    pub object: Vec<ObjectListEntry<K, V>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObjectListEntry<K: ValType, V: ValType> {
    #[serde(rename = "MapKey")]
    pub map_key: Val<K>,
    #[serde(rename = "MapValue")]
    pub map_value: Val<V>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DataList<K: ValType = FixedString, V: ValType = LSString> {
    pub data: Vec<DataListEntry<K, V>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DataListEntry<K: ValType, V: ValType> {
    pub key: Val<K>,
    pub val: Val<V>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EditorDataAttitudes {
    pub data: Vec<EditorDataAttitude>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EditorDataAttitude {
    /// Q: Seems to be an index?
    pub key: Val<FixedString>,
    /// "Neutral" | ...
    pub val: Val<LSString>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EditorDataEmotions {
    pub data: Vec<EditorDataEmotion>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EditorDataEmotion {
    /// Q: Seems to be an index?
    pub key: Val<FixedString>,
    /// "Neutral" | ...
    pub val: Val<LSString>,
}
