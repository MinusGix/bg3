use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::attr::TypeId;

pub type CowStr<'a> = Cow<'a, str>;

pub fn parse_lsx(input: &str) -> Result<Save<'_>, quick_xml::DeError> {
    quick_xml::de::from_str(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LSXVersion {
    V3,
    V4,
}

/// Root `<save>` element
/// This is called save, but is used for things that are not saves.
/// (Is it ever used for saves?)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Save<'b> {
    #[serde(rename = "version")]
    pub version: Version,
    #[serde(rename = "region")]
    pub region: Region<'b>,
}
impl<'b> Save<'b> {
    pub fn lsx_version(&self) -> LSXVersion {
        self.version.lsx_version()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
impl Version {
    pub fn lsx_version(&self) -> LSXVersion {
        if self.major >= 4 {
            LSXVersion::V4
        } else {
            LSXVersion::V3
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Region<'b> {
    #[serde(rename = "@id")]
    pub id: CowStr<'b>,
    #[serde(rename = "node")]
    pub node: Node<'b>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub struct Children<'b> {
    #[serde(rename = "$value")]
    pub elems: Option<Vec<Node<'b>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Attribute<'b> {
    #[serde(rename = "@id")]
    pub id: CowStr<'b>,
    // TODO: I think the logic in LSLib has so for v3 it can have a u32 typeid attribute field
    // or it can have a string attribute field. And v4 always has string attribute field.
    // So it should be fine that we serialize it as a str always, and don't remember what it was.
    // But it would be nice to have unaltered round trips
    #[serde(
        rename = "@type",
        deserialize_with = "TypeId::deser_str_or_int",
        serialize_with = "TypeId::ser_str"
    )]
    pub ty: TypeId,
    /// Exists only with `TranslatedString` and `TranslatedFSString`
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
    /// SeExists only with `TranslatedFSString`, typically 0
    #[serde(rename = "@arguments")]
    pub arguments: Option<u32>,
    #[serde(rename = "$value")]
    pub children: Option<Vec<AttributeChild<'b>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AttributeChild<'b> {
    Argument(Argument<'b>),
    Float2(Float2),
    Float3(Float3),
    Float4(Float4),
    Mat4(Mat4),
    #[serde(rename = "$text")]
    Other(Cow<'b, str>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Argument<'b> {
    #[serde(rename = "@key")]
    pub key: CowStr<'b>,
    #[serde(rename = "@value")]
    pub value: CowStr<'b>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Float2 {
    #[serde(rename = "@x")]
    pub x: f32,
    #[serde(rename = "@y")]
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Float3 {
    #[serde(rename = "@x")]
    pub x: f32,
    #[serde(rename = "@y")]
    pub y: f32,
    #[serde(rename = "@z")]
    pub z: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Float4 {
    #[serde(rename = "@x")]
    pub x: f32,
    #[serde(rename = "@y")]
    pub y: f32,
    #[serde(rename = "@z")]
    pub z: f32,
    #[serde(rename = "@w")]
    pub w: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mat4 {
    #[serde(rename = "float4")]
    pub elems: [Float4; 4],
}
