use serde::Deserialize;
use stix::CommonProperties;

#[derive(Deserialize, stix::TypedObject)]
#[typed_object(name = "x-mitre-tactic")]
pub struct Tactic {
    #[serde(flatten)]
    common: CommonProperties,
}

impl AsRef<CommonProperties> for Tactic {
    fn as_ref(&self) -> &CommonProperties {
        &self.common
    }
}
