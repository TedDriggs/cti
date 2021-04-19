use std::borrow::Cow;

use thiserror::Error;

use crate::{Id, TypedObject};

/// Error when an ID tries to reference an object with a different type name.
///
/// The beginning of a STIX object ID is the object's type name, e.g. `tool--...`.
/// That ID must refer to an object where `type == "tool"` per the STIX specification,
/// so it is always incorrect to request malware using that tool ID.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("ID `{}` cannot identify an object of type `{}`", .id, .object_type)]
pub struct IdTypeMismatchError<'id> {
    id: Cow<'id, Id>,
    object_type: &'static str,
}

impl<'id> IdTypeMismatchError<'id> {
    pub fn new<D: TypedObject>(id: &'id Id) -> Self {
        Self {
            id: Cow::Borrowed(id),
            object_type: D::TYPE,
        }
    }

    /// Get the ID which caused the error
    pub fn id(&self) -> &Id {
        &self.id
    }

    /// Create a new error which owns its ID, allowing it to outlive the input ID.
    pub fn to_owned(&self) -> IdTypeMismatchError<'static> {
        IdTypeMismatchError {
            id: Cow::Owned(self.id().clone()),
            object_type: self.object_type,
        }
    }
}

impl IdTypeMismatchError<'static> {
    pub fn new_owned<D: TypedObject>(id: Id) -> Self {
        Self {
            id: Cow::Owned(id),
            object_type: D::TYPE,
        }
    }
}
