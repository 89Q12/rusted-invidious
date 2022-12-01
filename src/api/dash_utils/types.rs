use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub declaration: Declaration,
    pub elements: Vec<Element>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Declaration {
    pub attributes: Attributes,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub version: String,
    pub encoding: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub attributes: Attributes2,
    pub elements: Vec<Element2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes2 {
    pub xmlns: String,
    pub profiles: String,
    pub min_buffer_time: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub media_presentation_duration: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element2 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub elements: Vec<AdaptionSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdaptionSet {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub attributes: AdaptionSetAttributes,
    pub elements: Vec<Element4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdaptionSetAttributes {
    pub id: usize,
    pub mime_type: String,
    #[serde(rename = "startWithSAP")]
    pub start_with_sap: String,
    pub subsegment_alignment: String,
    pub scan_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element4 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub attributes: Attributes4,
    pub elements: Vec<Element5>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes4 {
    pub id: String,
    pub codecs: String,
    pub bandwidth: String,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub max_playout_rate: Option<String>,
    pub frame_rate: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element5 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub elements: Option<Vec<Element6>>,
    pub attributes: Option<Attributes8>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element6 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<Attributes5>,
    pub elements: Option<Vec<Element7>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes5 {
    pub range: Option<i64>,
    pub id: Option<i64>,
    pub codecs: Option<String>,
    pub bandwidth: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element7 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub attributes: Option<Attributes6>,
    #[serde(default)]
    pub elements: Vec<Element8>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes6 {
    pub index_range: Option<i64>,
    pub scheme_id_uri: Option<String>,
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element8 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: Option<String>,
    pub attributes: Option<Attributes7>,
    pub text: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes7 {
    pub range: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes8 {
    pub index_range: Option<i64>,
    pub scheme_id_uri: Option<String>,
    pub value: Option<String>,
}
