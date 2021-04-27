use serde::{Deserialize, Serialize};

use crate::{CivicLocation, Contact, MaybeVariable, MaybeVariableRef};

use super::{Target, TargetType};

macro_rules! simple_target {
    ($name:ident, $ty_const:expr) => {
        #[derive(Debug, Clone, Deserialize, Serialize)]
        pub struct $name {
            name: MaybeVariable<String>,
            #[serde(default)]
            description: Option<String>,
            #[serde(default)]
            contact: Option<Contact>,
            #[serde(default)]
            location: Option<CivicLocation>,
        }

        impl Target for $name {
            fn target_type(&self) -> &TargetType {
                $ty_const
            }

            fn name(&self) -> MaybeVariableRef<&str> {
                self.name.as_str()
            }

            fn description(&self) -> Option<&str> {
                self.description.as_ref().map(|s| s.as_str())
            }
        }
    };
}

simple_target!(TargetIndividual, TargetType::INDIVIDUAL);
simple_target!(TargetGroup, TargetType::GROUP);
simple_target!(TargetOrganization, TargetType::ORGANIZATION);
