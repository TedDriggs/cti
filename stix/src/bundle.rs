use serde::Deserialize;

use crate::{Id, TypedObject};

#[derive(Deserialize)]
pub struct Bundle<T> {
    pub id: Id,
    pub spec_version: String,
    pub objects: Vec<T>,
}

impl<T> TypedObject for Bundle<T> {
    const TYPE: &'static str = "bundle";
}
