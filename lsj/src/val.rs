use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// In LSJ files it is comment to see data of the form:
/// ```json
/// { "type": "FixedString", "value": "0" }
/// ```
/// And many of them take on specific types always.
/// So this type lets you write `Val<FixedString>` and it will hold a `FixedString` instance after
/// parsing the type field away (and checking it, of course)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Val<T>(pub T);
impl<T: AsRef<str>> AsRef<str> for Val<T> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
impl<'de, T: ValType + Deserialize<'de>> Deserialize<'de> for Val<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deser(deserializer).map(Val)
    }
}
impl<T: ValType + Serialize> Serialize for Val<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.ser(serializer)
    }
}

pub trait ValType: Sized {
    fn valid_typ(typ: &str) -> bool;

    fn typ(&self) -> &'static str;

    // Decide how to serialize the type yourself
    fn ser<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer;

    // Decide how to deserialize the type yourself
    fn deser<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
        Self: Deserialize<'de>; // AAAAH
}
#[derive(Serialize)]
struct InnerSer<'a, U> {
    #[serde(rename = "type")]
    type_: &'static str,
    value: &'a U,
}
#[derive(Debug, Deserialize)]
struct InnerDeser<'a, U> {
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
    value: U,
}
pub trait StaticValType {
    const TYPE: &'static str = "bool";
}
impl<'d, V: StaticValType + Serialize + std::fmt::Debug> ValType for V {
    fn valid_typ(typ: &str) -> bool {
        typ == Self::TYPE
    }

    fn typ(&self) -> &'static str {
        Self::TYPE
    }

    fn ser<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        InnerSer {
            type_: Self::TYPE,
            // this assumes references serialize the same, probably true
            value: &self,
        }
        .serialize(serializer)
    }

    fn deser<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
        V: Deserialize<'de>,
    {
        // TODO: this wastes work if the type is wrong
        // And could fail in a confusing manner if the type was wrong since we also parse the value first
        let inner: InnerDeser<V> = InnerDeser::deserialize(deserializer)?;
        if !V::valid_typ(&inner.type_) {
            return Err(serde::de::Error::custom(format!(
                "Expected type for struct {} but got {}",
                std::any::type_name::<V>(),
                inner.type_
            )));
        }
        Ok(inner.value)
    }
}

impl StaticValType for bool {
    const TYPE: &'static str = "bool";
}

impl StaticValType for u8 {
    const TYPE: &'static str = "uint8";
}

impl StaticValType for i16 {
    const TYPE: &'static str = "int16";
}

impl StaticValType for u16 {
    const TYPE: &'static str = "uint16";
}

impl StaticValType for i32 {
    const TYPE: &'static str = "int32";
}

impl StaticValType for u32 {
    const TYPE: &'static str = "uint32";
}

impl StaticValType for i64 {
    const TYPE: &'static str = "int64";
}

impl StaticValType for u64 {
    const TYPE: &'static str = "uint64";
}

impl StaticValType for f32 {
    const TYPE: &'static str = "float";
}

impl StaticValType for f64 {
    const TYPE: &'static str = "double";
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Ivec2(pub [i32; 2]);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LSString(pub String);
impl StaticValType for LSString {
    const TYPE: &'static str = "LSString";
}
impl AsRef<str> for LSString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FixedString(pub String);
impl StaticValType for FixedString {
    const TYPE: &'static str = "FixedString";
}
impl AsRef<str> for FixedString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TranslatedString {
    pub handle: String,
    pub version: u32,
}
impl ValType for TranslatedString {
    fn valid_typ(typ: &str) -> bool {
        typ == "TranslatedString"
    }

    fn typ(&self) -> &'static str {
        "TranslatedString"
    }

    fn ser<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct InnerSer<'a> {
            #[serde(rename = "type")]
            type_: &'static str,
            handle: &'a str,
            version: u32,
        }

        InnerSer {
            type_: "TranslatedString",
            handle: &self.handle,
            version: self.version,
        }
        .serialize(serializer)
    }

    fn deser<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
        Self: Deserialize<'de>,
    {
        #[derive(Deserialize)]
        struct InnerDeser {
            #[serde(rename = "type")]
            type_: String,
            handle: String,
            version: u32,
        }

        let inner: InnerDeser = InnerDeser::deserialize(deserializer)?;
        if inner.type_ != "TranslatedString" {
            return Err(serde::de::Error::custom(format!(
                "Expected type for struct TranslatedString but got {}",
                inner.type_
            )));
        }
        Ok(Self {
            handle: inner.handle,
            version: inner.version,
        })
    }
}

// TODO: parse this as a byte array to avoid allocating
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Guid(pub String);
impl StaticValType for Guid {
    const TYPE: &'static str = "guid";
}

macro_rules! decl_enum {
    ($name:ident => $($f:ident : $t:ty),* $(,)?) => {
        impl ValType for $name {
            fn valid_typ(typ: &str) -> bool {
                // We have to use $t::is_valid_typ here because we can't guarantee it is static

                $(
                    if <$t>::valid_typ(typ) {
                        return true;
                    }
                )*

                false
            }

            fn typ(&self) -> &'static str {
                match self {
                    $(
                        $name::$f(x) => x.typ(),
                    )*
                }
            }

            fn ser<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    $(
                        $name::$f(v) => v.ser(serializer),
                    )*
                }
            }

            fn deser<'de, D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let inner: serde_json::Value = serde_json::Value::deserialize(deserializer)?;

                let typ = inner
                    .get("type")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| serde::de::Error::custom("Expected type field"))?;

                $(
                    if <$t>::valid_typ(typ) {
                        return Ok(<$t>::deser(&inner).map_err(serde::de::Error::custom).map($name::$f)?);
                    }
                )*

                // The type wasn't valid
                Err(serde::de::Error::custom(format!(
                    "Expected allowed type for enum {} but got {}",
                    stringify!($name),
                    typ
                )))
            }
        }
    };
}

/// Types that are like strings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StringKind {
    Fixed(FixedString),
    LS(LSString),
    Translated(TranslatedString),
}
decl_enum!(StringKind => Fixed: FixedString, LS: LSString, Translated: TranslatedString,);
impl AsRef<str> for StringKind {
    fn as_ref(&self) -> &str {
        match self {
            StringKind::Fixed(s) => s.0.as_ref(),
            StringKind::LS(s) => s.0.as_ref(),
            // Sortof!
            StringKind::Translated(s) => s.handle.as_ref(),
        }
    }
}

// TODO: We could have 'Constant Strings' which are required to be specific values, like emotions and the like

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_val() {
        let val: Val<LSString> = serde_json::from_value(json!({
            "type": "LSString",
            "value": "hello"
        }))
        .unwrap();
        assert_eq!(val.0, LSString("hello".to_string()));
    }
}
