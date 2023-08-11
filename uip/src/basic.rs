//! Basic UI elements

use serde::{Deserialize, Serialize};

use crate::{
    interaction::{InteractionTriggers, Trigger},
    larian::Storyboard,
    CowStr, DisplayText, Style, UriPath,
};

// TODO: make this a structure with a parse function
pub type Margin<'b> = CowStr<'b>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlTemplate<'b> {
    #[serde(rename = "StackPanel")]
    pub stack_panel: Option<StackPanel<'b>>,
    #[serde(rename = "ControlTemplate.Resources")]
    pub resources: Option<ControlTemplateResources<'b>>,
    #[serde(rename = "ControlTemplate.Triggers")]
    pub triggers: Option<ControlTemplateTriggers<'b>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlTemplateResources<'b> {
    #[serde(rename = "$value")]
    pub res: Vec<ControlTemplateResource<'b>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ControlTemplateResource<'b> {
    Storyboard(Storyboard<'b>),
    BitmapImage(BitmapImage<'b>),
    // TODO: it can probably have more
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlTemplateTriggers<'b> {
    // TODO: can this have general triggers or only data triggers?
    #[serde(rename = "$value")]
    pub triggers: Vec<Trigger<'b>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StackPanel<'b> {
    #[serde(rename = "@Name")]
    pub name: Option<CowStr<'b>>,
    #[serde(rename = "$value")]
    pub children: Vec<UIChildren<'b>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Grid<'b> {
    #[serde(rename = "@Name")]
    pub name: CowStr<'b>,
    // TODO: mark this as either a raw bool or a binding?
    #[serde(rename = "@IsEnabled")]
    pub is_enabled: CowStr<'b>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UIChildren<'b> {
    InteractionTriggers(InteractionTriggers<'b>),
    TextBlock(TextBlock<'b>),
    Listbox(ListBox<'b>),
    BitmapImage(BitmapImage<'b>),
    Image(Image<'b>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextBlock<'b> {
    #[serde(rename = "@TextBlockFormatter.SourceText")]
    source_text: DisplayText<'b>,
    // TODO: should we make a general macro/somehow-substructure for common fields like margin and style?
    #[serde(rename = "@Margin")]
    pub margin: Option<Margin<'b>>,
    #[serde(rename = "@Style")]
    pub style: Option<Style<'b>>,
    /// Default: [`TextAlignment::Left`]
    #[serde(rename = "@TextAlignment")]
    pub text_alignment: Option<TextAlignment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TextAlignment {
    Center,
    Left,
    Right,
    /// Equivalent to left?
    Start,
    Justify,
    DetectFromContent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListBox<'b> {
    #[serde(rename = "@Name")]
    pub name: CowStr<'b>,
    #[serde(rename = "@Template")]
    pub template: Option<CowStr<'b>>,
    #[serde(rename = "@Margin")]
    pub margin: Option<Margin<'b>>,
    #[serde(rename = "@Style")]
    pub style: Option<Style<'b>>,
    #[serde(rename = "@ItemTemplate")]
    pub item_template: Option<CowStr<'b>>,
    #[serde(rename = "@ItemsSource")]
    pub items_source: Option<CowStr<'b>>,
    #[serde(rename = "@SelectedItem")]
    pub selected_item: Option<CowStr<'b>>,
    #[serde(rename = "@ItemContainerStyle")]
    pub item_container_style: Option<Style<'b>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListBoxResources<'b> {
    #[serde(rename = "Style")]
    pub style_elem: Option<StyleElem<'b>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StyleElem<'b> {
    #[serde(rename = "@Key")]
    pub key: CowStr<'b>,
    #[serde(rename = "@BasedOn")]
    pub based_on: Option<Style<'b>>,
    #[serde(rename = "@TargetType")]
    pub target_type: Option<TargetType<'b>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TargetType<'b> {
    Rectangle,
    // TODO: other target types
    #[serde(rename = "$text")]
    Other(CowStr<'b>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BitmapImage<'b> {
    #[serde(rename = "@Key")]
    pub key: CowStr<'b>,
    #[serde(rename = "@UriSource")]
    pub uri_source: UriPath<'b>,
    // TODO: can we assert that there's no other fields or children?
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image<'b> {
    #[serde(rename = "@Style")]
    pub style: Option<Style<'b>>,
}
