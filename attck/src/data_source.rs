use stix::CommonProperties;

#[derive(serde::Deserialize, stix::TypedObject)]
#[typed_object(name = "x-mitre-data-source")]
#[non_exhaustive]
pub struct DataSource {
    #[serde(flatten)]
    common: CommonProperties,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default, rename = "x_mitre_platforms")]
    pub platforms: Vec<String>,
    #[serde(default, rename = "x_mitre_contributors")]
    pub contributors: Vec<String>,
    #[serde(default, rename = "x_mitre_collection_layers")]
    pub collection_layers: Vec<String>,
}

impl AsRef<CommonProperties> for DataSource {
    fn as_ref(&self) -> &CommonProperties {
        &self.common
    }
}
