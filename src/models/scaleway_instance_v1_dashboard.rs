/*
 * Instance API
 *
 * # Introduction  ## Endpoints  Scaleway instance API can be reach on  - `https://api.scaleway.com/instance/v1/zones/fr-par-1` - `https://api.scaleway.com/instance/v1/zones/fr-par-2` - `https://api.scaleway.com/instance/v1/zones/nl-ams-1` - `https://api.scaleway.com/instance/v1/zones/pl-waw-1`  Older endpoints are still reachable but should not be used for new projects  - `https://cp-par1.scaleway.com` - `https://cp-ams1.scaleway.com`  <Example>  The following code is an example request to retrieve detailed information about a volume:  ``` % curl -H 'X-Auth-Token: xxxxxxxx-xxxx-xxxxx-xxxx-xxxxxxxxxxxxx' -H 'Content-Type: application/json' https://api.scaleway.com/instance/v1/zones/fr-par-1/volumes/f929fe39-63f8-4be8-a80e-1e9c8ae22a76 -i  HTTP/1.1 200 OK Server: nginx Date: Thu, 22 May 2014 07:55:00 GMT Content-Type: application/json Content-Length: 1345 Connection: keep-alive Strict-Transport-Security: max-age=86400  {   \"volumes\": [     {       \"export_uri\": null,       \"id\": \"f929fe39-63f8-4be8-a80e-1e9c8ae22a76\",       \"name\": \"volume-0-1\",       \"organization\": \"000a115d-2852-4b0a-9ce8-47f1134ba95a\",       \"server\": null,       \"size\": 10000000000,       \"volume_type\": \"l_ssd\"     },     {       \"export_uri\": null,       \"id\": \"0facb6b5-b117-441a-81c1-f28b1d723779\",       \"name\": \"volume-0-2\",       \"organization\": \"000a115d-2852-4b0a-9ce8-47f1134ba95a\",       \"server\": null,       \"size\": 20000000000,       \"volume_type\": \"l_ssd\"     }   ] } ```  </Example>  ## Pagination  Most of listing requests receive a paginated response.  **Paginated request**  Requests against paginated endpoints accept two `query` arguments:  - `page`, a positive integer to choose the page to return. - `per_page`, an positive integer lower or equal to 100 to select the number of   items to return. The default value is `50`.  Paginated endpoints usually also accept filters to search and sort results. These filters are documented along each endpoint documentation.  **Paginated response**  ```bash % curl -H 'X-Auth-Token: <token>' 'https://api.scaleway.com/instance/v1/zones/fr-par-1/images/?page=2&per_page=10' -i HTTP/1.0 200 OK [...] X-Total-Count: 209 [...] ```  The `X-Total-Count` header contains the total number of items for the resource. 
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalewayInstanceV1Dashboard {
    #[serde(rename = "volumes_count", skip_serializing_if = "Option::is_none")]
    pub volumes_count: Option<f32>,
    #[serde(rename = "running_servers_count", skip_serializing_if = "Option::is_none")]
    pub running_servers_count: Option<f32>,
    #[serde(rename = "servers_by_types", skip_serializing_if = "Option::is_none")]
    pub servers_by_types: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "images_count", skip_serializing_if = "Option::is_none")]
    pub images_count: Option<f32>,
    #[serde(rename = "snapshots_count", skip_serializing_if = "Option::is_none")]
    pub snapshots_count: Option<f32>,
    #[serde(rename = "servers_count", skip_serializing_if = "Option::is_none")]
    pub servers_count: Option<f32>,
    #[serde(rename = "ips_count", skip_serializing_if = "Option::is_none")]
    pub ips_count: Option<f32>,
    #[serde(rename = "security_groups_count", skip_serializing_if = "Option::is_none")]
    pub security_groups_count: Option<f32>,
    #[serde(rename = "ips_unused", skip_serializing_if = "Option::is_none")]
    pub ips_unused: Option<f32>,
}

impl ScalewayInstanceV1Dashboard {
    pub fn new() -> ScalewayInstanceV1Dashboard {
        ScalewayInstanceV1Dashboard {
            volumes_count: None,
            running_servers_count: None,
            servers_by_types: None,
            images_count: None,
            snapshots_count: None,
            servers_count: None,
            ips_count: None,
            security_groups_count: None,
            ips_unused: None,
        }
    }
}


