use serde::Deserialize;

use crate::{Declaration, Id};

#[derive(Deserialize)]
pub struct Bundle {
    pub id: Id,
    pub spec_version: String,
    pub objects: Vec<Declaration>,
}
