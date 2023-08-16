use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

// TODO: This should maybe be extracted to a utility crate
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TypeId {
    None = 0,
    Uint8 = 1,
    Int16 = 2,
    Uint16 = 3,
    Int32 = 4,
    Uint32 = 5,
    Float = 6,
    Double = 7,
    Ivec2 = 8,
    Ivec3 = 9,
    Ivec4 = 10,
    Fvec2 = 11,
    Fvec3 = 12,
    Fvec4 = 13,
    Mat2x2 = 14,
    Mat3x3 = 15,
    Mat3x4 = 16,
    Mat4x3 = 17,
    Mat4x4 = 18,
    Bool = 19,
    String = 20,
    Path = 21,
    FixedString = 22,
    LSString = 23,
    Uint64 = 24,
    ScratchBuffer = 25,
    OldInt64 = 26,
    Int8 = 27,
    TranslatedString = 28,
    WString = 29,
    LSWString = 30,
    Guid = 31,
    Int64 = 32,
    TranslatedFSString = 33,
}
impl TypeId {
    pub const fn to_id(&self) -> u8 {
        *self as u8
    }

    pub const fn from_id(id: u8) -> Option<TypeId> {
        Some(match id {
            0 => TypeId::None,
            1 => TypeId::Uint8,
            2 => TypeId::Int16,
            3 => TypeId::Uint16,
            4 => TypeId::Int32,
            5 => TypeId::Uint32,
            6 => TypeId::Float,
            7 => TypeId::Double,
            8 => TypeId::Ivec2,
            9 => TypeId::Ivec3,
            10 => TypeId::Ivec4,
            11 => TypeId::Fvec2,
            12 => TypeId::Fvec3,
            13 => TypeId::Fvec4,
            14 => TypeId::Mat2x2,
            15 => TypeId::Mat3x3,
            16 => TypeId::Mat3x4,
            17 => TypeId::Mat4x3,
            18 => TypeId::Mat4x4,
            19 => TypeId::Bool,
            20 => TypeId::String,
            21 => TypeId::Path,
            22 => TypeId::FixedString,
            23 => TypeId::LSString,
            24 => TypeId::Uint64,
            25 => TypeId::ScratchBuffer,
            26 => TypeId::OldInt64,
            27 => TypeId::Int8,
            28 => TypeId::TranslatedString,
            29 => TypeId::WString,
            30 => TypeId::LSWString,
            31 => TypeId::Guid,
            32 => TypeId::Int64,
            33 => TypeId::TranslatedFSString,
            _ => return None,
        })
    }

    /// Parse this from the string representation used in LSX files.
    pub fn from_str(v: &str) -> Option<TypeId> {
        Some(match v {
            "None" => TypeId::None,
            "uint8" => TypeId::Uint8,
            "int16" => TypeId::Int16,
            "uint16" => TypeId::Uint16,
            "int32" => TypeId::Int32,
            "uint32" => TypeId::Uint32,
            "float" => TypeId::Float,
            "double" => TypeId::Double,
            "ivec2" => TypeId::Ivec2,
            "ivec3" => TypeId::Ivec3,
            "ivec4" => TypeId::Ivec4,
            "fvec2" => TypeId::Fvec2,
            "fvec3" => TypeId::Fvec3,
            "fvec4" => TypeId::Fvec4,
            "mat2x2" => TypeId::Mat2x2,
            "mat3x3" => TypeId::Mat3x3,
            "mat3x4" => TypeId::Mat3x4,
            "mat4x3" => TypeId::Mat4x3,
            "mat4x4" => TypeId::Mat4x4,
            "bool" => TypeId::Bool,
            "string" => TypeId::String,
            "path" => TypeId::Path,
            "FixedString" => TypeId::FixedString,
            "LSString" => TypeId::LSString,
            "uint64" => TypeId::Uint64,
            "ScratchBuffer" => TypeId::ScratchBuffer,
            "old_int64" => TypeId::OldInt64,
            "int8" => TypeId::Int8,
            "TranslatedString" => TypeId::TranslatedString,
            "WString" => TypeId::WString,
            "LSWString" => TypeId::LSWString,
            "guid" => TypeId::Guid,
            "int64" => TypeId::Int64,
            "TranslatedFSString" => TypeId::TranslatedFSString,
            _ => return None,
        })
    }

    pub const fn to_str(&self) -> &'static str {
        use TypeId::*;
        match self {
            None => "None",
            Uint8 => "uint8",
            Int16 => "int16",
            Uint16 => "uint16",
            Int32 => "int32",
            Uint32 => "uint32",
            Float => "float",
            Double => "double",
            Ivec2 => "ivec2",
            Ivec3 => "ivec3",
            Ivec4 => "ivec4",
            Fvec2 => "fvec2",
            Fvec3 => "fvec3",
            Fvec4 => "fvec4",
            Mat2x2 => "mat2x2",
            Mat3x3 => "mat3x3",
            Mat3x4 => "mat3x4",
            Mat4x3 => "mat4x3",
            Mat4x4 => "mat4x4",
            Bool => "bool",
            String => "string",
            Path => "path",
            FixedString => "FixedString",
            LSString => "LSString",
            Uint64 => "uint64",
            ScratchBuffer => "ScratchBuffer",
            OldInt64 => "old_int64",
            Int8 => "int8",
            TranslatedString => "TranslatedString",
            WString => "WString",
            LSWString => "LSWString",
            Guid => "guid",
            Int64 => "int64",
            TranslatedFSString => "TranslatedFSString",
        }
    }

    /// Serialize as a string, for use with `serialize_with`
    #[doc(hidden)]
    pub fn ser_str<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.to_str())
    }

    /// Deserialize from a string, for use with `deserialize_with`
    #[doc(hidden)]
    pub fn deser_str<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).ok_or_else(|| D::Error::custom(format!("invalid type: {}", s)))
    }

    /// Deserialize from a string, but check if it is an integer id first and then fallback to str
    /// conversion
    #[doc(hidden)]
    pub fn deser_str_or_int<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        if let Ok(id) = s.parse::<u8>() {
            Self::from_id(id).ok_or_else(|| D::Error::custom(format!("invalid type id: {}", id)))
        } else {
            Self::from_str(&s).ok_or_else(|| D::Error::custom(format!("invalid type: {}", s)))
        }
    }
}
