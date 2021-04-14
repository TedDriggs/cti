use std::collections::BTreeSet;

use serde::Deserialize;
use stix::CommonProperties;

#[derive(Default, Deserialize)]
pub struct Data {
    #[serde(default, rename = "x_mitre_is_subtechnique")]
    pub is_subtechnique: Option<bool>,
    #[serde(default, rename = "x_mitre_data_sources")]
    pub data_sources: BTreeSet<String>,
    #[serde(default, rename = "x_mitre_detection")]
    pub detection: Option<String>,
    #[serde(default, rename = "x_mitre_permissions_required")]
    pub permissions_required: BTreeSet<String>,
    #[serde(default, rename = "x_mitre_platforms")]
    pub platforms: BTreeSet<String>,
}

#[derive(Deserialize, stix::TypedObject)]
pub struct AttackPattern {
    #[serde(flatten)]
    pub base: stix::AttackPattern,
    #[serde(flatten)]
    pub mitre: Data,
}

impl AttackPattern {
    pub fn name(&self) -> &str {
        &self.base.name
    }
}

impl AsRef<CommonProperties> for AttackPattern {
    fn as_ref(&self) -> &CommonProperties {
        self.base.as_ref()
    }
}

impl AsRef<stix::AttackPattern> for AttackPattern {
    fn as_ref(&self) -> &stix::AttackPattern {
        &self.base
    }
}
