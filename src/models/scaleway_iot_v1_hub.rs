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
pub struct ScalewayIotV1Hub {
    /// Hub ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Hub name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Current status of the Hub
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Hub feature set
    #[serde(rename = "product_plan", skip_serializing_if = "Option::is_none")]
    pub product_plan: Option<ProductPlan>,
    /// Whether the hub has been enabled
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Number of registered devices
    #[serde(rename = "device_count", skip_serializing_if = "Option::is_none")]
    pub device_count: Option<f32>,
    /// Number of currently connected devices
    #[serde(
        rename = "connected_device_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub connected_device_count: Option<f32>,
    /// Devices should be connected to this host, port may be 1883 (MQTT), 8883 (MQTT over TLS), 80 (MQTT over Websocket) or 443 (MQTT over Websocket over TLS).
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// Disable Hub events
    #[serde(rename = "disable_events", skip_serializing_if = "Option::is_none")]
    pub disable_events: Option<bool>,
    /// Hub events topic prefix
    #[serde(
        rename = "events_topic_prefix",
        skip_serializing_if = "Option::is_none"
    )]
    pub events_topic_prefix: Option<String>,
    /// Region of the Hub
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Hub creation date
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Hub last modification date
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Project owning the resource
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// Organization owning the resource
    #[serde(rename = "organization_id", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// When an unknown device connects to your hub using a valid certificate chain, it will be automatically provisioned inside your hub. The hub uses the common name of the device certifcate to find out if a device with the same name already exists. This setting can only be enabled on a hub with a custom certificate authority.
    #[serde(
        rename = "enable_device_auto_provisioning",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_device_auto_provisioning: Option<bool>,
    /// After creating a hub, this flag is set to False as the hub certificates are managed by Scaleway. Once a custom certificate authority is installed, this flag will be set to true.
    #[serde(rename = "has_custom_ca", skip_serializing_if = "Option::is_none")]
    pub has_custom_ca: Option<bool>,
    #[serde(
        rename = "twins_graphite_config",
        skip_serializing_if = "Option::is_none"
    )]
    pub twins_graphite_config:
        Option<Box<crate::models::IotV1RegionsRegionHubsTwinsGraphiteConfig>>,
}

impl ScalewayIotV1Hub {
    pub fn new() -> ScalewayIotV1Hub {
        ScalewayIotV1Hub {
            id: None,
            name: None,
            status: None,
            product_plan: None,
            enabled: None,
            device_count: None,
            connected_device_count: None,
            endpoint: None,
            disable_events: None,
            events_topic_prefix: None,
            region: None,
            created_at: None,
            updated_at: None,
            project_id: None,
            organization_id: None,
            enable_device_auto_provisioning: None,
            has_custom_ca: None,
            twins_graphite_config: None,
        }
    }
}

/// Current status of the Hub
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "enabling")]
    Enabling,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "disabling")]
    Disabling,
    #[serde(rename = "disabled")]
    Disabled,
}
/// Hub feature set
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProductPlan {
    #[serde(rename = "plan_unknown")]
    Unknown,
    #[serde(rename = "plan_shared")]
    Shared,
    #[serde(rename = "plan_dedicated")]
    Dedicated,
    #[serde(rename = "plan_ha")]
    Ha,
}
