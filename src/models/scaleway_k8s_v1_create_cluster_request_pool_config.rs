/*
 * Bare metal API
 *
 * # Introduction  Bare metal as a service allows ordering a dedicated server on-demand like a cloud instance. Dedicated servers could be used for large workloads, big data, those requiring more security, ….  This is the `v1` documentation, the `v1alpha1` version is available [here](/en/products/baremetal/api/v1alpha1).  ## Technical Limitations  - Bare metal is only available in `fr-par-2` zone  - Installation is done by preseed (± 10min) (preseed: complete install from a virtual media)  - The list of OS is limited, you can install your own using the following tutorial: https://www.scaleway.com/en/docs/bare-metal-server-installation-kvm-over-ip/  ## Features  - Install (Server is installed by preseed (preseed: complete install from a virtual media), you must define at least one ssh key to install your server)  - Start/Stop/Reboot  - Rescue Reboot, a rescue image is an operating system image designed to help you diagnose and fix an OS experiencing failures. When your server boot on rescue, you can mount your disks and start diagnosing/fixing your image.  - BMC access: Baseboard Management Controller (BMC) allows you to remotely access the low-level parameters of your dedicated server. For instance, your KVM-IP management console could be accessed with it.  - Billed by minute (The billing start when the server is delivered and stop when the server is deleted)  - IPv6, all servers are available with an IPv6 /128  - ReverseIP, You can configure your reverse IP (IPv4 and IPv6), you must register the server IP in your DNS records before calling the endpoint  - Basic monitoring with ping status  - IP failovers are not available in api v1, use the api v1alpha1  ## FAQ  ### How can I get my ssh key id ?  You can find your `$SCW_SECRET_KEY` and your `SCW_DEFAULT_ORGANIZATION_ID` at the following page: https://console.scaleway.com/project/credentials
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
