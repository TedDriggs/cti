use std::fmt;

use stix::Id;

/// Some data and its ID.
///
/// Unlike STIX objects, CACAO often uses the ID as a key in a dictionary with the object
/// body being the value. This can make it difficult to pass the key and value as a single
/// item.
///
/// `theobroma` upholds the invariant that a `WithId` struct's `id` and `data` always match.
/// Therefore, construction or mutation outside the crate is impossible.
#[derive(Debug, Clone, Copy)]
pub struct WithId<'a, T> {
    id: &'a Id,
    data: T,
}

impl<'a, T> WithId<'a, T> {
    pub(crate) fn new((id, data): (&'a Id, T)) -> Self {
        Self { id, data }
    }

    pub fn id(&self) -> &'a Id {
        self.id
    }

    pub fn data(&self) -> &T {
        &self.data
    }
}

impl<T: fmt::Display> fmt::Display for WithId<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.id, self.data)
    }
}

/// Behavior for a fallible function encountering an error.
///
/// Production systems which expect valid input typically do not want to do unnecessary work
/// finding every error in bad input before returning; returning the first-encountered error
/// is sufficient.
///
/// In contrast, editing tools want a complete list of problems so the user doesn't have to
/// run the tool over and over again to find each problem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnError {
    /// Fail as soon as any problem is encountered.
    FailFast,
    /// Keep going as long as possible, without causing any state changes.
    FailAtEnd,
}

impl Default for OnError {
    fn default() -> Self {
        Self::FailFast
    }
}
