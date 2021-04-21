use std::borrow::Cow;

pub fn pluralize(s: &str) -> Cow<'static, str> {
    match s {
        "course_of_action" => Cow::Borrowed("courses_of_action"),
        "infrastructure" => Cow::Borrowed("infrastructure"),
        "malware" => Cow::Borrowed("malware"),
        "matrix" => Cow::Borrowed("matrices"),
        _ if s.ends_with("analysis") => Cow::Owned(
            s.chars()
                .take(s.len() - "analysis".len())
                .chain("analyses".chars())
                .collect(),
        ),
        _ if s.ends_with('y') => {
            Cow::Owned(s.chars().take(s.len() - 1).chain("ies".chars()).collect())
        }
        _ => Cow::Owned(format!("{}s", s)),
    }
}

#[cfg(test)]
mod tests {
    use super::pluralize;

    #[test]
    fn simple() {
        assert_eq!(pluralize("attack_pattern"), "attack_patterns");
    }

    #[test]
    fn ending_in_y() {
        assert_eq!(pluralize("vulnerability"), "vulnerabilities");
        assert_eq!(pluralize("identity"), "identities");
    }
}
