use std::collections::BTreeSet;

use serde::Deserialize;

use crate::{CommonProperties, KillChainPhase, TypedObject};

#[derive(Deserialize)]
pub struct Infrastructure {
    #[serde(flatten)]
    common: CommonProperties,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub aliases: BTreeSet<String>,
    #[serde(default)]
    pub kill_chain_phases: Vec<KillChainPhase>,
}

impl AsRef<CommonProperties> for Infrastructure {
    fn as_ref(&self) -> &CommonProperties {
        &self.common
    }
}

impl TypedObject for Infrastructure {
    const TYPE: &'static str = "infrastructure";
}
