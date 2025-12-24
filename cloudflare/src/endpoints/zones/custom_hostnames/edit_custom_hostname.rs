use std::collections::BTreeMap;

use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};

use super::{CustomHostname, CustomHostnameSsl};
use crate::framework::response::ApiSuccess;

/// Edit a Custom Hostname
/// Updates an existing custom hostname for a zone
/// <https://developers.cloudflare.com/api/resources/custom_hostnames/methods/edit/>
#[derive(Debug)]
pub struct EditCustomHostname<'a> {
    pub zone_identifier: &'a str,
    pub custom_hostname_id: &'a str,
    pub params: EditCustomHostnameParams,
}

impl EndpointSpec for EditCustomHostname<'_> {
    type JsonResponse = CustomHostname;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PATCH
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/custom_hostnames/{}",
            self.zone_identifier, self.custom_hostname_id
        )
    }
    #[inline]
    fn body(&self) -> Option<RequestBody<'_>> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct EditCustomHostnameParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_metadata: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_origin_server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_origin_sni: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl: Option<CustomHostnameSsl>,
}
