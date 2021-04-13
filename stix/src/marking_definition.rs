use serde::Deserialize;

use crate::CommonProperties;

#[derive(Deserialize, stix_derive::TypedObject)]
#[typed_object(core)]
pub struct MarkingDefinition {
    #[serde(flatten)]
    common: CommonProperties,
    pub name: Option<String>,
}

impl AsRef<CommonProperties> for MarkingDefinition {
    fn as_ref(&self) -> &CommonProperties {
        &self.common
    }
}
