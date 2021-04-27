use serde::{Deserialize, Serialize};
use stix::{location::Country, vocab::Region};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct CivicLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub building_details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<Region>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Country>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_area: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
}
