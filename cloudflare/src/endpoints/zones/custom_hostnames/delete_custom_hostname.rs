use crate::framework::endpoint::{EndpointSpec, Method};

use super::CustomHostnameOnlyId;
use crate::framework::response::ApiSuccess;

/// Delete a Custom Hostname
/// Deletes a custom hostname from a zone
/// <https://developers.cloudflare.com/api/resources/custom_hostnames/methods/delete/>
#[derive(Debug)]
pub struct DeleteCustomHostname<'a> {
    pub zone_identifier: &'a str,
    pub custom_hostname_id: &'a str,
}

impl EndpointSpec for DeleteCustomHostname<'_> {
    type JsonResponse = CustomHostnameOnlyId;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/custom_hostnames/{}",
            self.zone_identifier, self.custom_hostname_id
        )
    }
}
