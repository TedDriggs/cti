use serde::Deserialize;

use crate::{Declaration, Id, TypedObject};

#[derive(Deserialize)]
pub struct Bundle {
    pub id: Id,
    pub spec_version: String,
    pub objects: Vec<Declaration>,
}

impl TypedObject for Bundle {
    const TYPE: &'static str = "bundle";
}
