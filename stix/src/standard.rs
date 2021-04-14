//! Types for build a collection of STIX 2.1 objects with no user-defined object types,
//! fields, or relationships.

use serde::Deserialize;

#[stix_derive::extension]
#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
#[non_exhaustive]
#[stix(core)]
pub enum Declaration {}
