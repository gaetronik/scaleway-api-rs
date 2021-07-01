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
pub struct ScalewayK8sV1CreateClusterRequestPoolConfig {
    /// The name of the pool
    #[serde(rename = "name")]
    pub name: String,
    /// The node type is the type of Scaleway Instance wanted for the pool
    #[serde(rename = "node_type")]
    pub node_type: String,
    /// The placement group ID in which all the nodes of the pool will be created
    #[serde(rename = "placement_group_id", skip_serializing_if = "Option::is_none")]
    pub placement_group_id: Option<String>,
    /// The enablement of the autoscaling feature for the pool
    #[serde(rename = "autoscaling", skip_serializing_if = "Option::is_none")]
    pub autoscaling: Option<bool>,
    /// The size (number of nodes) of the pool
    #[serde(rename = "size")]
    pub size: f32,
    /// The minimun size of the pool. Note that this fields will be used only when autoscaling is enabled.
    #[serde(rename = "min_size", skip_serializing_if = "Option::is_none")]
    pub min_size: Option<f32>,
    /// The maximum size of the pool. Note that this fields will be used only when autoscaling is enabled.
    #[serde(rename = "max_size", skip_serializing_if = "Option::is_none")]
    pub max_size: Option<f32>,
    /// The customization of the container runtime is available for each pool. Note that `docker` is the only supporter runtime at the moment. Others are to be considered experimental.
    #[serde(rename = "container_runtime", skip_serializing_if = "Option::is_none")]
    pub container_runtime: Option<ContainerRuntime>,
    /// The enablement of the autohealing feature for the pool
    #[serde(rename = "autohealing", skip_serializing_if = "Option::is_none")]
    pub autohealing: Option<bool>,
    /// The tags associated with the pool
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The Kubelet arguments to be used by this pool. Note that this feature is to be considered as experimental
    #[serde(rename = "kubelet_args", skip_serializing_if = "Option::is_none")]
    pub kubelet_args: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "upgrade_policy", skip_serializing_if = "Option::is_none")]
    pub upgrade_policy:
        Option<Box<crate::models::ScalewayK8sV1CreateClusterRequestPoolConfigUpgradePolicy>>,
    /// The Zone in which the Pool's node will be spawn in
    #[serde(rename = "zone", skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

impl ScalewayK8sV1CreateClusterRequestPoolConfig {
    pub fn new(
        name: String,
        node_type: String,
        size: f32,
    ) -> ScalewayK8sV1CreateClusterRequestPoolConfig {
        ScalewayK8sV1CreateClusterRequestPoolConfig {
            name,
            node_type,
            placement_group_id: None,
            autoscaling: None,
            size,
            min_size: None,
            max_size: None,
            container_runtime: None,
            autohealing: None,
            tags: None,
            kubelet_args: None,
            upgrade_policy: None,
            zone: None,
        }
    }
}

/// The customization of the container runtime is available for each pool. Note that `docker` is the only supporter runtime at the moment. Others are to be considered experimental.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContainerRuntime {
    #[serde(rename = "unknown_runtime")]
    UnknownRuntime,
    #[serde(rename = "docker")]
    Docker,
    #[serde(rename = "containerd")]
    Containerd,
    #[serde(rename = "crio")]
    Crio,
}