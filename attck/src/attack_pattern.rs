use std::collections::BTreeSet;

use serde::Deserialize;
use stix::{CommonProperties, Object};

use crate::get_mitre_id;

/// MITRE custom properties to extend the `attack-pattern` STIX domain object.
#[stix::custom_properties(namespace = "mitre")]
#[derive(Default, Deserialize)]
pub struct MitreAttackPattern {
    #[serde(default)]
    pub is_subtechnique: Option<bool>,
    #[serde(default)]
    pub data_sources: BTreeSet<String>,
    #[serde(default)]
    pub detection: Option<String>,
    #[serde(default)]
    pub effective_permissions: BTreeSet<String>,
    #[serde(default)]
    pub permissions_required: BTreeSet<String>,
    #[serde(default)]
    pub platforms: BTreeSet<String>,
    #[serde(default)]
    pub system_requirements: BTreeSet<String>,
}

#[derive(Deserialize, stix::TypedObject)]
pub struct AttackPattern {
    #[serde(flatten)]
    pub base: stix::AttackPattern,
    #[serde(flatten)]
    pub mitre: MitreAttackPattern,
}

impl AttackPattern {
    pub fn name(&self) -> &str {
        &self.base.name
    }

    /// Get the MITRE ID for this attack pattern, such as `T1156` or `T1546.004`.
    pub fn mitre_id(&self) -> Option<&str> {
        self.external_references().iter().find_map(get_mitre_id)
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
