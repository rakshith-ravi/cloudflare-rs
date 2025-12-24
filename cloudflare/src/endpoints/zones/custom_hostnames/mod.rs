use std::collections::BTreeMap;

use crate::framework::response::ApiResult;

/// The route to add a custom hostname to a zone
pub mod add_custom_hostname;
/// The route to get custom hostname details
pub mod custom_hostname_details;
/// The route to delete a custom hostname
pub mod delete_custom_hostname;
/// The route to edit a custom hostname
pub mod edit_custom_hostname;

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
    pub wildcard: Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum CustomHostnameSslBundleMethod {
    Ubiquitous,
    Optimal,
    Force,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum CustomHostnameSslCertificateAuthority {
    Digicert,
    Google,
    LetsEncrypt,
    SslCom,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum CustomHostnameSslType {
    DV,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
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
