use serde::Deserialize;
use stix::{CommonProperties, Id, TypedObject};

#[derive(Deserialize, TypedObject)]
#[typed_object(name = "x-mitre-matrix")]
pub struct Matrix {
    #[serde(flatten)]
    common: CommonProperties,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub tactic_refs: Vec<Id>,
}

impl AsRef<CommonProperties> for Matrix {
    fn as_ref(&self) -> &CommonProperties {
        &self.common
    }
}
