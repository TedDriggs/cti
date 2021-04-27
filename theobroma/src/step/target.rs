use serde::{de::Error as _, Deserialize, Serialize};
use stix::Id;

use crate::MaybeVariable;

#[derive(Debug, Clone)]
pub enum Target<T = crate::StandardTarget> {
    Inline(T),
    Ids(Vec<MaybeVariable<Id>>),
}

impl<T: Serialize> Serialize for Target<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct RawTarget<'a, T2> {
            #[serde(skip_serializing_if = "Option::is_none")]
            target: Option<&'a T2>,
            #[serde(skip_serializing_if = "Option::is_none")]
            target_ids: Option<&'a [MaybeVariable<Id>]>,
        }

        impl<'a, T2> From<&'a Target<T2>> for RawTarget<'a, T2> {
            fn from(v: &'a Target<T2>) -> Self {
                match v {
                    Target::Ids(ids) => Self {
                        target: None,
                        target_ids: Some(ids.as_slice()),
                    },
                    Target::Inline(inline) => Self {
                        target: Some(inline),
                        target_ids: None,
                    },
                }
            }
        }

        RawTarget::from(self).serialize(serializer)
    }
}

fn none<T>() -> Option<T> {
    None
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Target<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawTarget<T2> {
            #[serde(default = "none")]
            target: Option<T2>,
            #[serde(default)]
            target_ids: Option<Vec<MaybeVariable<Id>>>,
        }

        let raw = RawTarget::deserialize(deserializer)?;
        match (raw.target, raw.target_ids) {
            (Some(_), Some(_)) => Err(D::Error::custom(
                "`target` and `target_ids` are mutually exclusive",
            )),
            (Some(inline), None) => Ok(Self::Inline(inline)),
            (None, Some(ids)) => Ok(Self::Ids(ids)),
            (None, None) => Err(D::Error::custom(
                "either `target` or `target_ids` is required",
            )),
        }
    }
}
