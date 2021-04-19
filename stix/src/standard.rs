//! Types for build a collection of STIX 2.1 objects with no user-defined object types,
//! fields, or relationships.

use serde::Deserialize;

#[stix_derive::declaration]
#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
#[non_exhaustive]
#[stix(core)]
pub enum Declaration {}

#[cfg(test)]
mod tests {
    use super::Collection;
    use crate::{Id, Malware};

    #[test]
    #[should_panic]
    fn get_with_bad_id() {
        // Mimikatz
        let id = Id::from("tool--afc079f3-c0ea-4096-b75d-3f05338b7f60");

        let collection = Collection::builder().build();

        // This should panic because the ID says we're expecting a `tool` object
        collection.get::<Malware>(&id);
    }
}
