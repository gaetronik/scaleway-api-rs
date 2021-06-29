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
pub struct ScalewayRdbV1Instance {
    /// Creation date (Format ISO 8601)
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<Box<crate::models::ScalewayRdbV1InstanceVolume>>,
    /// Region the instance is in
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// UUID of the instance
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of the instance
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Organization ID the instance belongs to
    #[serde(rename = "organization_id", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// Project ID the instance belongs to
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// Status of the instance
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Database engine of the database (PostgreSQL, MySQL, ...)
    #[serde(rename = "engine", skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Box<crate::models::ScalewayRdbV1InstanceEndpoint>>,
    /// List of tags applied to the instance
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Advanced settings of the instance
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<Vec<crate::models::ScalewayRdbV1InstanceSetting>>,
    #[serde(rename = "backup_schedule", skip_serializing_if = "Option::is_none")]
    pub backup_schedule: Option<Box<crate::models::ScalewayRdbV1InstanceBackupSchedule>>,
    /// Whether or not High-Availability is enabled
    #[serde(rename = "is_ha_cluster", skip_serializing_if = "Option::is_none")]
    pub is_ha_cluster: Option<bool>,
    /// Read replicas of the instance
    #[serde(rename = "read_replicas", skip_serializing_if = "Option::is_none")]
    pub read_replicas: Option<Vec<crate::models::ScalewayRdbV1Endpoint>>,
    /// Node type of the instance
    #[serde(rename = "node_type", skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// List of engine settings to be set at database initialisation
    #[serde(rename = "init_settings", skip_serializing_if = "Option::is_none")]
    pub init_settings: Option<Vec<crate::models::ScalewayRdbV1InstanceSetting>>,
}

impl ScalewayRdbV1Instance {
    pub fn new() -> ScalewayRdbV1Instance {
        ScalewayRdbV1Instance {
            created_at: None,
            volume: None,
            region: None,
            id: None,
            name: None,
            organization_id: None,
            project_id: None,
            status: None,
            engine: None,
            endpoint: None,
            tags: None,
            settings: None,
            backup_schedule: None,
            is_ha_cluster: None,
            read_replicas: None,
            node_type: None,
            init_settings: None,
        }
    }
}

/// Status of the instance
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "provisioning")]
    Provisioning,
    #[serde(rename = "configuring")]
    Configuring,
    #[serde(rename = "deleting")]
    Deleting,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "autohealing")]
    Autohealing,
    #[serde(rename = "locked")]
    Locked,
    #[serde(rename = "initializing")]
    Initializing,
    #[serde(rename = "disk_full")]
    DiskFull,
    #[serde(rename = "backuping")]
    Backuping,
    #[serde(rename = "snapshotting")]
    Snapshotting,
}

