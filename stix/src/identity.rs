use std::fmt;

use serde::Deserialize;

use crate::{CommonProperties, TypedObject};

#[derive(Deserialize)]
pub struct Identity {
    #[serde(flatten)]
    common: CommonProperties,
    name: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    contact_information: Option<ContactInformation>,
}

impl Identity {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| s.as_str())
    }

    pub fn contact_information(&self) -> Option<&ContactInformation> {
        self.contact_information.as_ref()
    }
}

impl AsRef<CommonProperties> for Identity {
    fn as_ref(&self) -> &CommonProperties {
        &self.common
    }
}

impl TypedObject for Identity {
    const TYPE: &'static str = "identity";
}

/// Contact information for an [`Identity`].
#[derive(Debug, Deserialize, Clone)]
pub struct ContactInformation(String);

impl fmt::Display for ContactInformation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
