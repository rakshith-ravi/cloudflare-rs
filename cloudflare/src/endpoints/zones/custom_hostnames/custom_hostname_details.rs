use crate::framework::endpoint::{EndpointSpec, Method};

use super::CustomHostname;
use crate::framework::response::ApiSuccess;

/// Get Custom Hostname Details
/// Retrieves details of an existing custom hostname for a zone
/// <https://developers.cloudflare.com/api/resources/custom_hostnames/methods/get/>
#[derive(Debug)]
pub struct GetCustomHostnameDetails<'a> {
    pub zone_identifier: &'a str,
    pub custom_hostname_id: &'a str,
}

impl EndpointSpec for GetCustomHostnameDetails<'_> {
    type JsonResponse = CustomHostname;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/custom_hostnames/{}",
            self.zone_identifier, self.custom_hostname_id
        )
    }
}
