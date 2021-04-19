//! Types for working with confidence of STIX objects.

use std::{convert::TryFrom, fmt};

use serde::{de::Error, Deserialize, Deserializer, Serialize};
use thiserror::Error;

/// A confidence score in the range between 0 and 100 (inclusive).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Confidence(u8);

impl Confidence {
    pub const MIN: Confidence = Confidence(0);
    pub const MAX: Confidence = Confidence(100);

    /// Create a new instance by checking that the provided value is in the spec-defined
    /// bounds on the confidence score.
    pub fn new(value: u8) -> Result<Self, OutOfBoundsError> {
        if value > Self::MAX.0 {
            Err(OutOfBoundsError(value))
        } else {
            Ok(Self(value))
        }
    }
}

impl fmt::Display for Confidence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl TryFrom<u8> for Confidence {
    type Error = OutOfBoundsError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<'de> Deserialize<'de> for Confidence {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::try_from(u8::deserialize(deserializer)?).map_err(D::Error::custom)
    }
}

/// Error when a confidence value falls outside the permitted range in the STIX spec.
#[derive(Debug, Error)]
pub struct OutOfBoundsError(u8);

impl fmt::Display for OutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Confidence must be between {} and {}, was {}",
            Confidence::MIN,
            Confidence::MAX,
            self.0
        )
    }
}
