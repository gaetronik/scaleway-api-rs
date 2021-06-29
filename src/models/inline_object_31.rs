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
pub struct InlineObject31 {
    /// Specify the port used to health check
    #[serde(rename = "port")]
    pub port: f32,
    /// Time between two consecutive health checks (in milliseconds)
    #[serde(rename = "check_delay")]
    pub check_delay: f32,
    /// Additional check timeout, after the connection has been already established (in milliseconds)
    #[serde(rename = "check_timeout")]
    pub check_timeout: f32,
    /// Number of consecutive unsuccessful health checks, after wich the server will be considered dead
    #[serde(rename = "check_max_retries")]
    pub check_max_retries: f32,
    #[serde(rename = "mysql_config", skip_serializing_if = "Option::is_none")]
    pub mysql_config: Option<Box<crate::models::LbV1RegionsRegionBackendsBackendIdHealthcheckMysqlConfig>>,
    /// The response is analyzed to find an LDAPv3 response message
    #[serde(rename = "ldap_config", skip_serializing_if = "Option::is_none")]
    pub ldap_config: Option<serde_json::Value>,
    /// The response is analyzed to find the +PONG response message
    #[serde(rename = "redis_config", skip_serializing_if = "Option::is_none")]
    pub redis_config: Option<serde_json::Value>,
    #[serde(rename = "pgsql_config", skip_serializing_if = "Option::is_none")]
    pub pgsql_config: Option<Box<crate::models::LbV1RegionsRegionBackendsBackendIdHealthcheckPgsqlConfig>>,
    #[serde(rename = "tcp_config", skip_serializing_if = "Option::is_none")]
    pub tcp_config: Option<serde_json::Value>,
    #[serde(rename = "http_config", skip_serializing_if = "Option::is_none")]
    pub http_config: Option<Box<crate::models::LbV1RegionsRegionBackendsBackendIdHealthcheckHttpConfig>>,
    #[serde(rename = "https_config", skip_serializing_if = "Option::is_none")]
    pub https_config: Option<Box<crate::models::LbV1RegionsRegionBackendsBackendIdHealthcheckHttpConfig>>,
    /// It defines whether the healthcheck should be done considering the proxy protocol
    #[serde(rename = "check_send_proxy", skip_serializing_if = "Option::is_none")]
    pub check_send_proxy: Option<bool>,
}

impl InlineObject31 {
    pub fn new(port: f32, check_delay: f32, check_timeout: f32, check_max_retries: f32) -> InlineObject31 {
        InlineObject31 {
            port,
            check_delay,
            check_timeout,
            check_max_retries,
            mysql_config: None,
            ldap_config: None,
            redis_config: None,
            pgsql_config: None,
            tcp_config: None,
            http_config: None,
            https_config: None,
            check_send_proxy: None,
        }
    }
}


