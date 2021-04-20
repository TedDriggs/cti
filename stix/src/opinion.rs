use std::collections::BTreeSet;

use serde::Deserialize;

use crate::{vocab, CommonProperties, Id};

#[derive(Deserialize, stix_derive::TypedObject)]
#[typed_object(core)]
pub struct Opinion {
    #[serde(flatten)]
    common: CommonProperties,
    #[serde(default)]
    pub explanation: Option<String>,
    #[serde(default)]
    pub authors: Vec<String>,
    pub opinion: vocab::Opinion,
    pub object_refs: BTreeSet<Id>,
}

impl AsRef<CommonProperties> for Opinion {
    fn as_ref(&self) -> &CommonProperties {
        &self.common
    }
}
