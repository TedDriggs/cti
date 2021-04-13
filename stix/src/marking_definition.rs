use serde::Deserialize;

use crate::CommonProperties;

#[derive(Deserialize)]
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
