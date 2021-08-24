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
pub struct ScalewayDomainV2beta1Record {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<f32>,
    #[serde(rename = "ttl", skip_serializing_if = "Option::is_none")]
    pub ttl: Option<f32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::ScalewayDomainV2beta1RecordType>,
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "geo_ip_config", skip_serializing_if = "Option::is_none")]
    pub geo_ip_config: Option<Box<crate::models::ScalewayDomainV2beta1RecordGeoIpConfig>>,
    #[serde(
        rename = "http_service_config",
        skip_serializing_if = "Option::is_none"
    )]
    pub http_service_config:
        Option<Box<crate::models::ScalewayDomainV2beta1RecordHttpServiceConfig>>,
    #[serde(rename = "weighted_config", skip_serializing_if = "Option::is_none")]
    pub weighted_config: Option<Box<crate::models::ScalewayDomainV2beta1RecordWeightedConfig>>,
    #[serde(rename = "view_config", skip_serializing_if = "Option::is_none")]
    pub view_config: Option<Box<crate::models::ScalewayDomainV2beta1RecordViewConfig>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl ScalewayDomainV2beta1Record {
    pub fn new() -> ScalewayDomainV2beta1Record {
        ScalewayDomainV2beta1Record {
            data: None,
            name: None,
            priority: None,
            ttl: None,
            _type: None,
            comment: None,
            geo_ip_config: None,
            http_service_config: None,
            weighted_config: None,
            view_config: None,
            id: None,
        }
    }
}
