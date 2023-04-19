use stix::CommonProperties;

#[derive(serde::Deserialize, stix::TypedObject)]
#[typed_object(name = "x-mitre-data-component")]
#[non_exhaustive]
pub struct DataComponent {
    #[serde(flatten)]
    common: CommonProperties,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
}

impl AsRef<CommonProperties> for DataComponent {
    fn as_ref(&self) -> &CommonProperties {
        &self.common
    }
}
