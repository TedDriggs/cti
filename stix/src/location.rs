use serde::Deserialize;

use crate::{CommonProperties, TypedObject};

#[derive(Deserialize)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
    #[serde(default)]
    pub precision: Option<f64>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Region(String);

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Country(String);

#[derive(Deserialize)]
pub struct Location {
    #[serde(flatten)]
    base: CommonProperties,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub region: Option<Region>,
    #[serde(default)]
    pub country: Option<Country>,
    #[serde(default)]
    pub administrative_area: Option<String>,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default)]
    pub street_address: Option<String>,
    #[serde(default)]
    pub postal_code: Option<String>,
    #[serde(default, flatten)]
    coordinates: Option<Coordinates>,
}

impl Location {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| s.as_str())
    }

    pub fn coordinates(&self) -> Option<&Coordinates> {
        self.coordinates.as_ref()
    }
}

impl TypedObject for Location {
    const TYPE: &'static str = "location";
}

impl AsRef<CommonProperties> for Location {
    fn as_ref(&self) -> &CommonProperties {
        &self.base
    }
}
