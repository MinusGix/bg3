use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::{larian::MessageBoxData, Bool, Command, CowStr, DisplayText, PathRef, TextUuid};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceDictionary<'b> {
    // TODO: can there be multiple
    #[serde(rename = "ResourceDictionary.MergedDictionaries")]
    pub merged_dicts: MergedDictionaries<'b>,

    // TODO: there probably can be multiple of these
    #[serde(rename = "LSMessageBoxData")]
    pub message_box_data: Option<MessageBoxData<'b>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MergedDictionaries<'b> {
    #[serde(rename = "ResourceDictionary")]
    pub resource_dictionaries: Vec<SourceResourceDictionary<'b>>,
}

/// A `<ResourceDictionary Source="..." />` element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceResourceDictionary<'b> {
    #[serde(rename = "@Source")]
    source: PathRef<'b>,
    // TODO: can we assert that there's no other fields or children?
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action<'b> {
    #[serde(rename = "LSGameCommandData")]
    GameCommandData(GameCommandData<'b>),
    #[serde(rename = "InvokeCommandAction")]
    InvokeCommand(InvokeCommand<'b>),
    #[serde(rename = "ChangePropertyAction")]
    ChangeProperty(ChangeProperty<'b>),
    #[serde(rename = "Setter")]
    Setter(Setter<'b>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameCommandData<'b> {
    #[serde(rename = "@ActionName")]
    pub action_name: DisplayText<'b>,
    #[serde(rename = "@Command")]
    pub command: Command<'b>,
    #[serde(rename = "@BoundInput")]
    pub bound_input: Option<CowStr<'b>>,
    // TODO: can we assert that there's no other fields or children?
}

/// `<InvokeCommandAction />` element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvokeCommand<'b> {
    #[serde(rename = "@Command")]
    pub command: Command<'b>,
    #[serde(rename = "@CommandParameter")]
    pub command_parameter: Option<CowStr<'b>>,
}

/// `<ChangePropertyAction />` element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeProperty<'b> {
    #[serde(rename = "@TargetName")]
    pub target_name: CowStr<'b>,
    #[serde(rename = "@PropertyName")]
    pub property_name: CowStr<'b>,
    #[serde(rename = "@Value")]
    pub value: CowStr<'b>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Setter<'b> {
    #[serde(rename = "@TargetName")]
    pub target_name: CowStr<'b>,
    #[serde(rename = "@Property")]
    pub property: CowStr<'b>,
    /// A setter's value can be a property but it can also be sub-elements.
    #[serde(rename = "@Value")]
    pub value: Option<CowStr<'b>>,
    #[serde(rename = "Setter.Value")]
    pub value_elem: Option<SetterValue<'b>>,
}
/// This is often sub elements that it wants to swap in.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetterValue<'b> {
    #[serde(rename = "$text")]
    pub body: CowStr<'b>,
}
