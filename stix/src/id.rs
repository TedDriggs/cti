use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
use std::{borrow::Cow, fmt, str::FromStr};
use thiserror::Error;
use uuid::Uuid;

use crate::TypedObject;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id {
    namespace: Cow<'static, str>,
    id: Uuid,
}

impl Id {
    /// Create a new ID for the specified object type.
    pub fn new<T: TypedObject>(uuid: Uuid) -> Self {
        Self {
            namespace: Cow::Borrowed(T::TYPE),
            id: uuid,
        }
    }

    pub fn object_type(&self) -> &str {
        &self.namespace
    }

    pub fn uuid(&self) -> &Uuid {
        &self.id
    }
}

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}--{}", self.namespace, self.id.to_hyphenated())
    }
}

impl FromStr for Id {
    type Err = IdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, "--");
        let raw_ns = parts.next().ok_or(IdParseError::TooFewParts)?;

        if raw_ns.chars().any(|c| c.is_uppercase()) {
            return Err(IdParseError::ObjectType(raw_ns.to_string()));
        }

        let namespace = Cow::Owned(raw_ns.to_string());
        let id = parts.next().ok_or(IdParseError::TooFewParts)?.parse()?;

        Ok(Id { namespace, id })
    }
}

/// Create an ID from a static string. This is equivalent to calling `"".parse().unwrap()`.
impl From<&'static str> for Id {
    fn from(id: &'static str) -> Self {
        id.parse().unwrap()
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(D::Error::custom)
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum IdParseError {
    #[error("Not enough parts. An ID is a namespace and UUID joined by '--'")]
    TooFewParts,
    #[error("Invalid object-type string `{}`", .0)]
    ObjectType(String),
    #[error("Unable to parse UUID")]
    Uuid(#[from] uuid::Error),
}
