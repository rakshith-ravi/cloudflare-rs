use std::collections::BTreeMap;

use crate::framework::response::ApiResult;

/// The route to add a custom hostname to a zone
mod add_custom_hostname;
/// The route to get custom hostname details
mod custom_hostname_details;
/// The route to delete a custom hostname
mod delete_custom_hostname;
/// The route to edit a custom hostname
mod edit_custom_hostname;

pub use self::{
    add_custom_hostname::*, custom_hostname_details::*, delete_custom_hostname::*,
    edit_custom_hostname::*,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CustomHostnameOnlyId {
    pub id: String,
}

impl ApiResult for CustomHostnameOnlyId {}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CustomHostname {
    pub id: String,
    pub hostname: String,
    pub ssl: Option<CustomHostnameSsl>,
    pub custom_metadata: Option<BTreeMap<String, String>>,
    pub ownership_verification: Option<OwnershipVerification>,
    pub ownership_verification_http: Option<OwnershipVerificationHttp>,
    pub status: String,
}

impl ApiResult for CustomHostname {}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CustomHostnameSsl {
    pub bundle_method: Option<CustomHostnameSslBundleMethod>,
    pub certificate_authority: Option<CustomHostnameSslCertificateAuthority>,
    #[serde(rename = "type")]
    pub type_: Option<CustomHostnameSslType>,
    pub method: Option<CustomHostnameSslMethod>,
    pub settings: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_records: Option<Vec<CustomHostnameSslValidationRecord>>,
    pub wildcard: Option<bool>,
    pub status: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CustomHostnameSslBundleMethod {
    Ubiquitous,
    Optimal,
    Force,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CustomHostnameSslCertificateAuthority {
    Digicert,
    Google,
    #[serde(rename = "lets_encrypt")]
    LetsEncrypt,
    #[serde(rename = "ssl_com")]
    SslCom,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CustomHostnameSslType {
    DV,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CustomHostnameSslMethod {
    Http,
    Txt,
    Email,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct OwnershipVerification {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub value: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct OwnershipVerificationHttp {
    pub http_body: String,
    pub http_url: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CustomHostnameSslValidationRecord {
    pub emails: Option<Vec<String>>,
    pub http_body: Option<String>,
    pub http_url: Option<String>,
    pub txt_name: Option<String>,
    pub txt_value: Option<String>,
}
