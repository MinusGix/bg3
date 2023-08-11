use serde::{de::DeserializeOwned, ser::SerializeSeq, Deserialize, Serialize};

/// A type that is `[ {} ]` if it is empty, and otherwise has a `Vec<Val<T>>`  
/// Note that this errors if the field is an empty array, since we've never seen that.
#[derive(Debug, Clone, PartialEq)]
pub enum VecOrEmpty<T> {
    Vec(Vec<T>),
    Empty,
}
impl<T> VecOrEmpty<T> {
    pub fn as_slice(&self) -> &[T] {
        match self {
            VecOrEmpty::Vec(v) => v.as_slice(),
            VecOrEmpty::Empty => &[],
        }
    }
}
impl<'de, T> Deserialize<'de> for VecOrEmpty<T>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
        Self: Sized,
    {
        // We can't just deserialize directly as a Vec<Val<T>> because `{}` is not necessarily a valid `T` inst
        let data: serde_json::Value = serde_json::Value::deserialize(deserializer)?;
        let arr = match &data {
            serde_json::Value::Array(arr) => arr,
            _ => return Err(serde::de::Error::custom("Expected array for ValVecOrEmpty")),
        };

        if arr.len() == 1 {
            let elem = arr.into_iter().next().unwrap();
            if elem == &serde_json::Value::Object(serde_json::Map::new()) {
                return Ok(VecOrEmpty::Empty);
            }
        }

        let v: Vec<T> = serde_json::from_value(data).map_err(serde::de::Error::custom)?;
        Ok(VecOrEmpty::Vec(v))
    }
}
impl<T> Serialize for VecOrEmpty<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        Self: Sized,
    {
        match self {
            VecOrEmpty::Vec(v) => v.serialize(serializer),
            // We have to serialize as [ {} ]
            VecOrEmpty::Empty => {
                let mut seq = serializer.serialize_seq(Some(1))?;
                seq.serialize_element(&serde_json::Value::Object(serde_json::Map::new()))?;
                seq.end()
            }
        }
    }
}

/// A type that panics if it is deserialized/serialized.  
/// For when we need to stub a type that we don't know but want to be alerted when there's more
/// info about it
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PanicDeser;
impl<'de> Deserialize<'de> for PanicDeser {
    fn deserialize<D>(_: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
        Self: Sized,
    {
        panic!("Attempted to deserialize PanicDeser")
    }
}
impl Serialize for PanicDeser {
    fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        Self: Sized,
    {
        panic!("Attempted to serialize PanicDeser")
    }
}
