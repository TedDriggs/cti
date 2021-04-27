use serde::{Deserialize, Serialize};

use crate::MaybeVariableRef;

mod http_api;
mod sector;
mod simple;
mod ssh;

pub use http_api::TargetHttpApi;
pub use sector::TargetSector;
pub use simple::{TargetGroup, TargetIndividual, TargetOrganization};
pub use ssh::TargetSsh;

/// Common properties on the Target object.
pub trait Target {
    fn target_type(&self) -> &TargetType;
    fn name(&self) -> MaybeVariableRef<&str>;
    fn description(&self) -> Option<&str>;
}

stix::vocabulary!(
    TargetType = [
        individual,
        group,
        organization,
        location,
        sector,
        http_api,
        ssh,
        security_infrastructure_category,
        net_address,
        kali,
        attacker,
        attack_agent,
        attack_group,
    ]
);

/// The target types defined in the CACAO spec.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum StandardTarget {
    HttpApi(TargetHttpApi),
    Group(TargetGroup),
    Individual(TargetIndividual),
    Organization(TargetOrganization),
    Sector(TargetSector),
    Ssh(TargetSsh),
}

impl Target for StandardTarget {
    fn target_type(&self) -> &TargetType {
        match self {
            Self::HttpApi(v) => v.target_type(),
            Self::Group(v) => v.target_type(),
            Self::Individual(v) => v.target_type(),
            Self::Organization(v) => v.target_type(),
            Self::Sector(v) => v.target_type(),
            Self::Ssh(v) => v.target_type(),
        }
    }

    fn name(&self) -> MaybeVariableRef<&str> {
        match self {
            Self::HttpApi(v) => v.name(),
            Self::Group(v) => v.name(),
            Self::Individual(v) => v.name(),
            Self::Organization(v) => v.name(),
            Self::Sector(v) => v.name(),
            Self::Ssh(v) => v.name(),
        }
    }

    fn description(&self) -> Option<&str> {
        match self {
            Self::HttpApi(v) => v.description(),
            Self::Group(v) => v.description(),
            Self::Individual(v) => v.description(),
            Self::Organization(v) => v.description(),
            Self::Sector(v) => v.description(),
            Self::Ssh(v) => v.description(),
        }
    }
}
