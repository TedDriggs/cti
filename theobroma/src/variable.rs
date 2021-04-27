use std::fmt;

use serde::{de::Error as _, Deserialize, Serialize};

mod def;
mod maybe;

pub use def::{CommonDef, VariableType};
pub use maybe::MaybeVariable;

/// A variable name in a CACAO playbook, e.g. `$$quarantine_target$$`.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Variable(String);

impl Variable {
    pub fn new(s: impl Into<String>) -> Result<Self, ParseError> {
        let s: String = s.into();
        if !s.starts_with(|c: char| c.is_alphabetic() || c == '_') {
            return Err(ParseError {
                kind: ErrorKind::InvalidStartCharacter,
                input: s,
            });
        }

        if s.len() > 250 {
            return Err(ParseError {
                kind: ErrorKind::TooLong,
                input: s,
            });
        }

        Ok(Self(s))
    }
}

impl Serialize for Variable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

impl From<&'static str> for Variable {
    fn from(v: &'static str) -> Self {
        Self::new(v).unwrap()
    }
}

impl<'de> Deserialize<'de> for Variable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if !s.starts_with("$$") {
            return Err(D::Error::custom(ParseError {
                kind: ErrorKind::MissingPrefix,
                input: s,
            }));
        }

        if !s.ends_with("$$") {
            return Err(D::Error::custom(ParseError {
                kind: ErrorKind::MissingSuffix,
                input: s,
            }));
        }

        Self::new(&s["$$".len()..(s.len() - "$$".len())]).map_err(D::Error::custom)
    }
}

impl fmt::Debug for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#""$${}$$""#, self.0)
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "$${}$$", self.0)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Unable to parse `{}`: {}", .input, .kind)]
pub struct ParseError {
    kind: ErrorKind,
    input: String,
}

#[derive(Debug, thiserror::Error)]
enum ErrorKind {
    #[error("Missing $$ prefix")]
    MissingPrefix,
    #[error("Missing $$ suffix")]
    MissingSuffix,
    #[error("First character after `$$` must be a letter or underscore")]
    InvalidStartCharacter,
    #[error("Name must be ≤250 ASCII characters")]
    TooLong,
}
