use serde::{Deserialize, Serialize};

stix::vocabulary!(CommandType = [manual, http_api, ssh, bash, openc2_json, attack_cmd]);

mod optional_base64 {
    use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<String>, D::Error> {
        // Read the string JSON field
        Option::<&str>::deserialize(deserializer)?
            // Attempt to do base64 decoding
            .map(base64::decode)
            .transpose()
            .map_err(D::Error::custom)?
            // Make sure the decoded data is a valid UTF-8 string
            .map(String::from_utf8)
            .transpose()
            .map_err(D::Error::custom)
    }

    pub fn serialize<S: Serializer>(
        data: &Option<String>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        data.as_ref().map(base64::encode).serialize(serializer)
    }
}

/// A command executed by a workflow [`Step`](crate::Step).
///
/// The contents of this struct are hidden pending more work on providing type-safe access to the internals.
///
/// The standard exposes the command and its base64 representation as strings, and leaves parsing to the caller.
/// This struct may end up exposing that intermediate form, but the crate should support being more precise
/// with the expected types.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Command {
    #[serde(rename = "type")]
    ty: CommandType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    command: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "optional_base64"
    )]
    command_b64: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    version: Option<String>,
}
