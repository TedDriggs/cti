use serde::{Deserialize, Serialize};

use super::Variable;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MaybeVariable<T> {
    Variable(Variable),
    Value(T),
}

impl<T> MaybeVariable<T> {
    pub fn as_value(&self) -> Option<&T> {
        if let Self::Value(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_variable(&self) -> Option<&Variable> {
        if let Self::Variable(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl<T> From<Variable> for MaybeVariable<T> {
    fn from(v: Variable) -> Self {
        Self::Variable(v)
    }
}

#[cfg(test)]
mod test {
    use stix::Id;

    use super::{MaybeVariable, Variable};

    #[test]
    fn deserialize_maybe_id_is_id() {
        let val: MaybeVariable<Id> =
            serde_json::from_str(r#""step--ba23c1b3-fdd2-4264-bc5b-c056c6862ba2""#).unwrap();
        assert_eq!(
            val.as_value(),
            Some(&Id::from("step--ba23c1b3-fdd2-4264-bc5b-c056c6862ba2"))
        );
    }

    #[test]
    fn deserialize_maybe_id_is_var() {
        let val: MaybeVariable<Id> = serde_json::from_str(r#""$$LOCAL_TARGET$$""#).unwrap();
        assert_eq!(val.as_variable(), Some(&Variable::from("LOCAL_TARGET")));
    }

    /// Test that a string which looks like a variable will be interpreted as one,
    /// even if it would be legal to interpret as a value.
    #[test]
    fn deserialize_prefers_variable() {
        let val: MaybeVariable<String> = serde_json::from_str(r#""$$LOCAL_TARGET$$""#).unwrap();
        assert_eq!(val.as_variable(), Some(&Variable::from("LOCAL_TARGET")));
    }
}
