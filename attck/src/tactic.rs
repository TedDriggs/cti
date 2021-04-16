use serde::Deserialize;
use stix::{CommonProperties, Object};

use crate::get_mitre_id;

#[derive(Deserialize, stix::TypedObject)]
#[typed_object(name = "x-mitre-tactic")]
pub struct Tactic {
    #[serde(flatten)]
    common: CommonProperties,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    /// The kebab-case name used in [`KillChainPhase`](::stix::KillChainPhase) to reference this tactic.
    #[serde(rename = "x_mitre_shortname")]
    pub shortname: String,
}

impl Tactic {
    /// Get the MITRE ATT&CK ID for this tactic, such as `TA0001`.
    pub fn mitre_id(&self) -> Option<&str> {
        self.external_references().iter().find_map(get_mitre_id)
    }
}

impl AsRef<CommonProperties> for Tactic {
    fn as_ref(&self) -> &CommonProperties {
        &self.common
    }
}
