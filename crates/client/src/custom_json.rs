#[allow(unused_imports)]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

pub(crate) trait ToJson {
    fn to_json(&self) -> Value;
}

impl Serialize for dyn ToJson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_json().serialize(serializer)
    }
}

#[allow(dead_code)]
pub(crate) trait FromJson: Sized {
    fn from_json(value: Value) -> Self;
}

// impl<'de> Deserialize<'de> for dyn FromJson where Self: Sized {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>
//     {
//         let value = Value::deserialize(deserializer)?;
//         Ok(Self::from_json(value))
//     }
// }
