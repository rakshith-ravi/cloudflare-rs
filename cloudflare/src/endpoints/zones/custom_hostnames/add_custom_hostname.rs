use std::collections::BTreeMap;

use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};

use super::{CustomHostname, CustomHostnameSsl};
use crate::framework::response::ApiSuccess;
use serde::Serialize;

/// Create a Custom Hostname
/// Creates a custom hostname for a zone
/// <https://developers.cloudflare.com/api/resources/custom_hostnames/methods/create/>
#[derive(Debug)]
pub struct AddCustomHostname<'a> {
    pub zone_identifier: &'a str,
    pub params: AddCustomHostnameParams,
}

impl EndpointSpec for AddCustomHostname<'_> {
    type JsonResponse = CustomHostname;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        format!("zones/{}/custom_hostnames", self.zone_identifier)
    }
    #[inline]
    fn body(&self) -> Option<RequestBody<'_>> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct AddCustomHostnameParams {
    pub hostname: String,
    pub ssl: Option<CustomHostnameSsl>,
    pub custom_metadata: Option<BTreeMap<String, String>>,
}
