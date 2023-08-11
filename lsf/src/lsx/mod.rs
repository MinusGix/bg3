use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::attr::TypeId;

pub type CowStr<'a> = Cow<'a, str>;

pub fn parse_lsx(input: &str) -> Result<Save<'_>, quick_xml::DeError> {
    quick_xml::de::from_str(input)
}

/// Root `<save>` element  
/// This is called save, but is used for things that are not saves.  
/// (Is it ever used for saves?)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Save<'b> {
    #[serde(rename = "version")]
    pub version: Version,
    #[serde(rename = "region")]
    pub region: Region<'b>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Version {
    #[serde(rename = "@major")]
    pub major: u32,
    #[serde(rename = "@minor")]
    pub minor: u32,
    #[serde(rename = "@revision")]
    pub revision: u32,
    #[serde(rename = "@build")]
    pub build: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Region<'b> {
    #[serde(rename = "@id")]
    pub id: CowStr<'b>,
    #[serde(rename = "node")]
    pub node: Node<'b>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node<'b> {
    #[serde(rename = "@id")]
    pub id: CowStr<'b>,
    #[serde(default, rename = "attribute")]
    pub attrs: Vec<Attribute<'b>>,
    /// It has a literal child element named children
    #[serde(rename = "children")]
    pub children: Option<Children<'b>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Children<'b> {
    #[serde(rename = "$value")]
    pub elems: Vec<Node<'b>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attribute<'b> {
    #[serde(rename = "@id")]
    pub id: CowStr<'b>,
    #[serde(
        rename = "@type",
        deserialize_with = "TypeId::deser_str",
        serialize_with = "TypeId::ser_str"
    )]
    pub ty: TypeId,
    /// Seems to exist with: `TranslatedString`
    #[serde(rename = "@handle")]
    pub handle: Option<CowStr<'b>>,
    /// Seems to exist with: `TranslatedString`
    // TODO: are versions integers or floats?
    #[serde(rename = "@version")]
    pub version: Option<CowStr<'b>>,
    /// Seems to exist with: `FixedString`, `guid`
    /// Does not seem to exist with: `TranslatedString`
    #[serde(rename = "@value")]
    pub value: Option<CowStr<'b>>,
}
