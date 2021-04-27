use std::fmt;

use serde::{de::Error as _, Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct BoundedU8<const MIN: u8, const MAX: u8>(u8);

impl<const MIN: u8, const MAX: u8> BoundedU8<MIN, MAX> {
    pub fn new(value: u8) -> Result<Self, OutOfBoundsError> {
        if value < MIN || value > MAX {
            Err(OutOfBoundsError {
                min: MIN,
                max: MAX,
                value,
            })
        } else {
            Ok(Self(value))
        }
    }
}

impl<const MIN: u8, const MAX: u8> fmt::Display for BoundedU8<MIN, MAX> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<const MIN: u8, const MAX: u8> PartialEq<u8> for BoundedU8<MIN, MAX> {
    fn eq(&self, other: &u8) -> bool {
        self.0.eq(other)
    }
}

impl<const MIN: u8, const MAX: u8> PartialOrd<u8> for BoundedU8<MIN, MAX> {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<const MIN: u8, const MAX: u8> From<BoundedU8<MIN, MAX>> for u8 {
    fn from(v: BoundedU8<MIN, MAX>) -> Self {
        v.0
    }
}

impl<'de, const MIN: u8, const MAX: u8> Deserialize<'de> for BoundedU8<MIN, MAX> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::new(u8::deserialize(deserializer)?).map_err(D::Error::custom)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("Value {} is not between {} and {}", .value, .min, .max)]
pub struct OutOfBoundsError {
    min: u8,
    max: u8,
    value: u8,
}

#[cfg(test)]
mod tests {
    use super::BoundedU8;

    #[test]
    fn priority_range() {
        let _: BoundedU8<0, 100> = BoundedU8::new(75).unwrap();
    }
}
