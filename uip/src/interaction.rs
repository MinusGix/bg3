use serde::{Deserialize, Serialize};

use crate::{resources::Action, CowStr, PropertyBinding};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InteractionTriggers<'b> {
    #[serde(rename = "$value")]
    pub triggers: Vec<Trigger<'b>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Trigger<'b> {
    #[serde(rename = "EventTrigger")]
    Event(EventTrigger<'b>),
    #[serde(rename = "PropertyChangedTrigger")]
    PropertyChanged(PropertyChangedTrigger<'b>),
    #[serde(rename = "DataTrigger")]
    Data(DataTrigger<'b>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventTrigger<'b> {
    #[serde(rename = "@EventName")]
    pub event_name: CowStr<'b>,

    pub actions: Vec<Action<'b>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropertyChangedTrigger<'b> {
    #[serde(rename = "@Binding")]
    pub binding: PropertyBinding<'b>,

    pub actions: Vec<Action<'b>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataTrigger<'b> {
    #[serde(rename = "@Binding")]
    pub binding: PropertyBinding<'b>,

    #[serde(rename = "@Value")]
    pub value: CowStr<'b>,

    pub actions: Vec<Action<'b>>,
}

/// ```xml
/// <MultiDataTrigger>
///     <MultiDataTrigger.Conditions>
///         <Condition Binding="{Binding Blah}" Value="Toast"/>
//          <Condition Binding="{Binding BlahCount, Converter={StaticResource GreaterThanConverter}, ConverterParameter=0}" Value="True"/>
///     </MultiDataTrigger.Conditions>
///     <Setter TargetName="amazingBlahThing" Property="Visibility" Value="Visible"/>
/// </MultiDataTrigger>
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiDataTrigger<'b> {
    #[serde(rename = "MultiDataTrigger.Conditions")]
    pub conditions: MutliDataConditions<'b>,
    #[serde(rename = "$value")]
    pub triggers: Vec<Trigger<'b>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MutliDataConditions<'b> {
    #[serde(rename = "$value")]
    pub conds: Vec<MultiDataCondition<'b>>,
}

/// `<Condition>` inside `<MultiDataTrigger.Conditions>`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiDataCondition<'b> {
    #[serde(rename = "@Binding")]
    pub binding: PropertyBinding<'b>,

    #[serde(rename = "@Value")]
    pub value: CowStr<'b>,

    #[serde(rename = "@Converter")]
    pub converter: Option<CowStr<'b>>,
}

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct InteractionBehaviors<'b> {}
