use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, str::FromStr};
use thiserror::Error;
use uuid::Uuid;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id {
    namespace: String,
    id: Uuid,
}

impl Id {
    pub fn namespace(&self) -> &str {
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
        let ns = parts.next().ok_or(IdParseError::TooFewParts)?;
        let id = parts.next().ok_or(IdParseError::TooFewParts)?.parse()?;

        Ok(Id {
            namespace: ns.to_string(),
            id: id,
        })
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
pub enum IdParseError {
    #[error("Not enough parts. An ID is a namespace and UUID joined by '--'")]
    TooFewParts,
    #[error("Unable to parse UUID")]
    Uuid(#[from] uuid::Error),
}
