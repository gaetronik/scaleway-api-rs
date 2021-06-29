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
pub struct ScalewayInstanceV1Bootscript {
    /// The bootscript arguments
    #[serde(rename = "bootcmdargs", skip_serializing_if = "Option::is_none")]
    pub bootcmdargs: Option<String>,
    /// Dispmay if the bootscript is the default bootscript if no other boot option is configured
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    /// Provide information regarding a Device Tree Binary (dtb) for use with C1 servers
    #[serde(rename = "dtb", skip_serializing_if = "Option::is_none")]
    pub dtb: Option<String>,
    /// The bootscript ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The initrd (initial ramdisk) configuration
    #[serde(rename = "initrd", skip_serializing_if = "Option::is_none")]
    pub initrd: Option<String>,
    /// The server kernel version
    #[serde(rename = "kernel", skip_serializing_if = "Option::is_none")]
    pub kernel: Option<String>,
    /// The bootscript organization ID
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// The bootscript project ID
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// Provide information if the bootscript is public
    #[serde(rename = "public", skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    /// The bootscript title
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The bootscript arch
    #[serde(rename = "arch", skip_serializing_if = "Option::is_none")]
    pub arch: Option<Arch>,
    /// The zone in which is the bootscript
    #[serde(rename = "zone", skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

impl ScalewayInstanceV1Bootscript {
    pub fn new() -> ScalewayInstanceV1Bootscript {
        ScalewayInstanceV1Bootscript {
            bootcmdargs: None,
            default: None,
            dtb: None,
            id: None,
            initrd: None,
            kernel: None,
            organization: None,
            project: None,
            public: None,
            title: None,
            arch: None,
            zone: None,
        }
    }
}

/// The bootscript arch
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Arch {
    #[serde(rename = "x86_64")]
    X8664,
    #[serde(rename = "arm")]
    Arm,
}

