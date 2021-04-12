use std::borrow::Cow;

pub fn pluralize(s: &str) -> Cow<'static, str> {
    match s {
        "course_of_action" => Cow::Borrowed("courses_of_action"),
        "identity" => Cow::Borrowed("identities"),
        _ => Cow::Owned(format!("{}s", s)),
    }
}
