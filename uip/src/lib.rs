pub mod basic;
pub mod binding;
pub mod interaction;
pub mod larian;
pub mod resources;

use std::{borrow::Cow, fs::File, io::BufRead, path::Path, sync::Arc};

use basic::Grid;
use larian::UIWidget;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};

/// Shorthand because I get tired of writing this.
pub type CowStr<'a> = Cow<'a, str>;
/// UUIDs are stored as strings, but since they're always the same length, we can store them as
/// arrays of bytes.  
/// Note that the uuids seem to have a prefix, like `h86f...`, where `h` is not a valid hex value.
/// This makes them 37 rather than 36 characters.  
///   
/// Note that there are fields in UI code called UUIDs that are normal strings, like
/// `UUIDTutorialsCheck`, but are not this type. This is for numeric UUIDs, which are typically
/// randomly generated, while those are chosen based on context and so that they can be easily
/// referenced by name.
pub struct Uuid(pub [u8; 37]);

// TODO: these could use some underlying type that has a generic for what they can be parsed into, like for Bools: `ValBinding<bool>` which says that it can either be a bool or it can be a binding to a bool.
/// A 'uuid' like `UUIDTutorialsCheck` that is used to reference a thing by name.  
/// These are typically (always?) raw text rather than bindings.
pub type TextUuid<'b> = CowStr<'b>;

/// A path to a file.
pub type PathRef<'b> = CowStr<'b>;
/// A more general path.  
/// Ex: `pack://application:,,,/GustavNoesisGUI;component/Assets/CC/thing.png`
pub type UriPath<'b> = CowStr<'b>;

/// Text that probably gets displayed to the user.  
/// This means it is likely not raw text but rather a binding to translate it from some source.
pub type DisplayText<'b> = CowStr<'b>;

/// A boolean value that is either `True` or `False` (TODO: verify that the case is always like
/// that) but it can also be a binding to a boolean value.
pub type Bool<'b> = CowStr<'b>;

/// Ex: `0:0:0.6`
pub type Dur<'b> = CowStr<'b>;

/// A command to do something. I don't yet understand their syntax very well.
pub type Command<'b> = CowStr<'b>;

pub type Style<'b> = CowStr<'b>;

/// General type that is almost certainly a binding
pub type Binding<'b> = CowStr<'b>;
/// General type that is almost certainly a binding to a property.
pub type PropertyBinding<'b> = CowStr<'b>;

// TODO: Binding parser.

/// A failure during parsing of a UI XAML file.
#[derive(Debug, Clone)]
pub enum UIError {
    Io(Arc<std::io::Error>),
    Xml(quick_xml::Error),
    XmlDe(quick_xml::de::DeError),
}
impl From<quick_xml::Error> for UIError {
    fn from(e: quick_xml::Error) -> Self {
        Self::Xml(e)
    }
}
impl From<std::io::Error> for UIError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(Arc::new(e))
    }
}
impl From<quick_xml::de::DeError> for UIError {
    fn from(e: quick_xml::de::DeError) -> Self {
        Self::XmlDe(e)
    }
}

// This would have issues with the lifetime.
// We could have a manager that loads the files and maintains them in memory? Since we'd later want to be linking stuff together anyway.
// And/or we could have a function to convert it to owned. There might be a crate to autogen that.
// pub fn parse_ui_xaml_from_file(path: impl AsRef<Path>) -> Result<(), UIError> {
//     let text = std::fs::read_to_string(path)?;
//     parse_ui_xaml_from_str(&text)
//     // let reader = quick_xml::Reader::from_file(path)?;

//     // parse_ui_xaml_from_reader(reader)
// }

// // TODO: should this require the filename? I"m uncertain how it gets the information.
pub fn parse_ui_xaml_from_str(text: &str) -> Result<UIWidget<'_>, UIError> {
    Ok(quick_xml::de::from_str(text)?)
}

// pub fn parse_ui_xaml_from_reader<R: BufRead>(mut reader: Reader<R>) -> Result<(), UIError> {
//     reader.
// }

#[cfg(test)]
mod tests {
    use super::*;

    /// The folder where you've extracted the PAKs. I've currently put them into folders by their names.
    fn bg3_folder() -> String {
        std::env::var("BG3_FOLDER").unwrap()
    }

    #[test]
    fn test_parse_ui_xaml_from_file() {
        let path = bg3_folder();
        let widget = parse_ui_xaml_from_str(r#"<ls:UIWidget x:Name="CharacterCreation"
        xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
        xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml"
        xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006"
        xmlns:ls="clr-namespace:ls;assembly=SharedGUI"
        xmlns:System="clr-namespace:System;assembly=mscorlib"
        xmlns:noesis="clr-namespace:NoesisGUIExtensions;assembly=Noesis.GUI.Extensions"
        xmlns:b="http://schemas.microsoft.com/xaml/behaviors"
        xmlns:d="http://schemas.microsoft.com/expression/blend/2008"
        mc:Ignorable="d"
        d:DesignHeight="2160" d:DesignWidth="3840"
        ls:UIWidget.ContextName="CharacterCreation"
        ls:TooltipExtender.Owner="{Binding DummyCharacter}"
        d:DataContext="{d:DesignInstance {x:Type ls:DCCharacterCreation}, IsDesignTimeCreatable=True}"></ls:UIWidget>"#).unwrap();

        assert_eq!(widget, UIWidget {
            name: CowStr::Borrowed("CharacterCreation"),
            design_height: 2160,
            design_width: 3840,
            context_name: CowStr::Borrowed("CharacterCreation"),
            tooltip_extender_owner: CowStr::Borrowed("{Binding DummyCharacter}"),
            data_context: CowStr::Borrowed("{d:DesignInstance {x:Type ls:DCCharacterCreation}, IsDesignTimeCreatable=True}"),
            focus_down: None,
            focus_left: None,
            focus_right: None,
            focus_up: None,
            focus_movement_mode: None,
            can_cache_surrounding_elements: None,
            resources: None,
            grid: None,
        });

        let file =
            std::fs::read_to_string(path + "/Game/Public/Game/GUI/Widgets/CharacterCreation.xaml")
                .unwrap();
        let widget = parse_ui_xaml_from_str(&file).unwrap();
        println!("{:#?}", widget);
        panic!();

        // parse_ui_xaml_from_file(path + "/Game/Public/Game/GUI/Widgets/CharacterCreation.xaml")
        //     .unwrap();
    }
}
