//! This module is for elements from Larian Studios' UI

use serde::{Deserialize, Serialize};

use crate::{
    basic::Grid,
    resources::{Action, ResourceDictionary},
    Bool, CowStr, DisplayText, Dur, TextUuid,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIWidget<'b> {
    // I think we need to grab the other attributes even if we consider them not relevant so that we can serialize them back into the file.
    #[serde(rename = "@Name")]
    pub name: CowStr<'b>,
    #[serde(rename = "@DesignHeight")]
    pub design_height: u32,
    #[serde(rename = "@DesignWidth")]
    pub design_width: u32,
    // TODO: is there any times where this is not equivalent to `name`?
    #[serde(rename = "@UIWidget.ContextName")]
    pub context_name: CowStr<'b>,
    #[serde(rename = "@TooltipExtender.Owner")]
    pub tooltip_extender_owner: CowStr<'b>,
    #[serde(rename = "@DataContext")]
    pub data_context: CowStr<'b>,

    // TODO: are these ever anything other than "UIDown", "UILeft", ...?
    #[serde(rename = "@FocusDown")]
    pub focus_down: Option<CowStr<'b>>,
    #[serde(rename = "@FocusLeft")]
    pub focus_left: Option<CowStr<'b>>,
    #[serde(rename = "@FocusRight")]
    pub focus_right: Option<CowStr<'b>>,
    #[serde(rename = "@FocusUp")]
    pub focus_up: Option<CowStr<'b>>,

    #[serde(rename = "@MoveFocus.FocusMovementMode")]
    pub focus_movement_mode: Option<FocusMovementMode>,
    #[serde(rename = "@CanCacheFocusSurroundingElements")]
    pub can_cache_surrounding_elements: Option<Bool<'b>>,

    // TODO: are any of these optional?
    // TODO: can we make the names a part of the structure rather than forcing it to be defined here?
    #[serde(rename = "UIWidget.Resources")]
    pub resources: Option<UIWidgetResources<'b>>,
    // #[serde(rename = "UIWidget.Interaction.Triggers")]
    // pub interaction_triggers: InteractionTriggers<'b>,
    // #[serde(rename = "UIWidget.Interaction.Behaviors")]
    // pub interaction_behaviors: InteractionBehaviors<'b>,
    #[serde(rename = "Grid")]
    pub grid: Option<Grid<'b>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIWidgetResources<'b> {
    #[serde(rename = "ResourceDictionary")]
    pub dict: ResourceDictionary<'b>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FocusMovementMode {
    Closest,
    // TODO: presumably there are others
}

/// `<LSMessageBoxData>` element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageBoxData<'b> {
    #[serde(rename = "@Key")]
    pub key: CowStr<'b>,
    // TODO: mark this as either raw text or a binding? Though it would probably always be a binding so that it can do the translated stuff.
    #[serde(rename = "@Text")]
    pub text: DisplayText<'b>,
    #[serde(rename = "@Title")]
    pub title: DisplayText<'b>,
    #[serde(rename = "@UUID")]
    pub uuid: TextUuid<'b>,
    #[serde(rename = "@CheckBox")]
    pub check_box: Bool<'b>,
    // TODO: is this saying that it is by default true or?
    #[serde(rename = "@CheckBoxValue")]
    pub check_box_value: Bool<'b>,
    #[serde(rename = "@CheckBoxLabel")]
    pub check_box_label: DisplayText<'b>,

    #[serde(rename = "LSMessageBoxData.Actions")]
    pub actions: Vec<MessageBoxDataActions<'b>>,
    // TODO: can we assert that there's no other fields or children?
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageBoxDataActions<'b> {
    #[serde(rename = "$value")] // ??
    pub actions: Vec<Action<'b>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Storyboard<'b> {
    #[serde(rename = "@Key")]
    pub key: CowStr<'b>,
    #[serde(rename = "$value")]
    pub children: Vec<StoryboardChild<'b>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StoryboardChild<'b> {
    DoubleAnimation(DoubleAnimation<'b>),
    ObjectAnimationUsingKeyFrames(ObjectAnimationUsingKeyFrames<'b>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DoubleAnimation<'b> {
    #[serde(rename = "@Storyboard.TargetName")]
    pub target_name: CowStr<'b>,
    #[serde(rename = "@Storyboard.TargetProperty")]
    pub target_property: CowStr<'b>,
    #[serde(rename = "@To")]
    pub to: f64,
    /// ex: `0:0:0.6`
    #[serde(rename = "@Duration")]
    pub duration: Dur<'b>,

    #[serde(rename = "@BeginTime")]
    pub begin_time: Option<Dur<'b>>,

    #[serde(rename = "@DecelerationRatio")]
    pub deceleration_ratio: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectAnimationUsingKeyFrames<'b> {
    #[serde(rename = "@Storyboard.TargetName")]
    pub target_name: CowStr<'b>,
    #[serde(rename = "@Storyboard.TargetProperty")]
    pub target_property: CowStr<'b>,

    #[serde(rename = "$value")]
    pub key_frames: Vec<ObjectKeyFrame<'b>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ObjectKeyFrame<'b> {
    DiscreteObjectKeyFrame(DiscreteObjectKeyFrame<'b>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscreteObjectKeyFrame<'b> {
    #[serde(rename = "@KeyTime")]
    pub key_time: Dur<'b>,
    #[serde(rename = "@Value")]
    pub value: CowStr<'b>,
}
