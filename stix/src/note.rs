use std::collections::BTreeSet;

use serde::Deserialize;

use crate::{CommonProperties, Id};

#[derive(Deserialize, stix_derive::TypedObject)]
#[typed_object(core)]
pub struct Note {
    #[serde(flatten)]
    common: CommonProperties,
    #[serde(default, rename = "abstract")]
    pub note_abstract: Option<String>,
    pub content: String,
    #[serde(default)]
    pub authors: Vec<String>,
    pub object_refs: BTreeSet<Id>,
}

impl AsRef<CommonProperties> for Note {
    fn as_ref(&self) -> &CommonProperties {
        &self.common
    }
}
