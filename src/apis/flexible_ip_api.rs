/*
 * Bare metal API
 *
 * # Introduction  Bare metal as a service allows ordering a dedicated server on-demand like a cloud instance. Dedicated servers could be used for large workloads, big data, those requiring more security, ….  This is the `v1` documentation, the `v1alpha1` version is available [here](/en/products/baremetal/api/v1alpha1).  ## Technical Limitations  - Bare metal is only available in `fr-par-2` zone  - Installation is done by preseed (± 10min) (preseed: complete install from a virtual media)  - The list of OS is limited, you can install your own using the following tutorial: https://www.scaleway.com/en/docs/bare-metal-server-installation-kvm-over-ip/  ## Features  - Install (Server is installed by preseed (preseed: complete install from a virtual media), you must define at least one ssh key to install your server)  - Start/Stop/Reboot  - Rescue Reboot, a rescue image is an operating system image designed to help you diagnose and fix an OS experiencing failures. When your server boot on rescue, you can mount your disks and start diagnosing/fixing your image.  - BMC access: Baseboard Management Controller (BMC) allows you to remotely access the low-level parameters of your dedicated server. For instance, your KVM-IP management console could be accessed with it.  - Billed by minute (The billing start when the server is delivered and stop when the server is deleted)  - IPv6, all servers are available with an IPv6 /128  - ReverseIP, You can configure your reverse IP (IPv4 and IPv6), you must register the server IP in your DNS records before calling the endpoint  - Basic monitoring with ping status  - IP failovers are not available in api v1, use the api v1alpha1  ## FAQ  ### How can I get my ssh key id ?  You can find your `$SCW_SECRET_KEY` and your `SCW_DEFAULT_ORGANIZATION_ID` at the following page: https://console.scaleway.com/project/credentials
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method `attach_flexible_ip`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttachFlexibleIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `create_flexible_ip`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFlexibleIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_flexible_ip`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteFlexibleIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_mac_addr`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteMacAddrError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `detach_flexible_ip`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DetachFlexibleIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `duplicate_mac_addr`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DuplicateMacAddrError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `generate_mac_addr`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenerateMacAddrError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_flexible_ip`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFlexibleIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_flexible_ips`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFlexibleIpsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `update_flexible_ip`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateFlexibleIpError {
    UnknownValue(serde_json::Value),
}

pub async fn attach_flexible_ip(
    configuration: &configuration::Configuration,
    zone: &str,
    inline_object55: crate::models::InlineObject55,
) -> Result<
    crate::models::ScalewayFlexibleIpV1alpha1AttachFlexibleIpsResponse,
    Error<AttachFlexibleIpError>,
> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/flexible-ip/v1alpha1/zones/{zone}/fips/attach",
        configuration.base_path,
        zone = crate::apis::urlencode(zone)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Auth-Token", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&inline_object55);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AttachFlexibleIpError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn create_flexible_ip(
    configuration: &configuration::Configuration,
    zone: &str,
    inline_object50: crate::models::InlineObject50,
) -> Result<crate::models::ScalewayFlexibleIpV1alpha1FlexibleIp, Error<CreateFlexibleIpError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/flexible-ip/v1alpha1/zones/{zone}/fips",
        configuration.base_path,
        zone = crate::apis::urlencode(zone)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Auth-Token", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&inline_object50);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateFlexibleIpError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_flexible_ip(
    configuration: &configuration::Configuration,
    zone: &str,
    fip_id: &str,
) -> Result<(), Error<DeleteFlexibleIpError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}",
        configuration.base_path,
        zone = crate::apis::urlencode(zone),
        fip_id = crate::apis::urlencode(fip_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Auth-Token", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteFlexibleIpError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_mac_addr(
    configuration: &configuration::Configuration,
    zone: &str,
    fip_id: &str,
) -> Result<(), Error<DeleteMacAddrError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}/mac",
        configuration.base_path,
        zone = crate::apis::urlencode(zone),
        fip_id = crate::apis::urlencode(fip_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Auth-Token", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteMacAddrError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn detach_flexible_ip(
    configuration: &configuration::Configuration,
    zone: &str,
    inline_object56: crate::models::InlineObject56,
) -> Result<
    crate::models::ScalewayFlexibleIpV1alpha1DetachFlexibleIpsResponse,
    Error<DetachFlexibleIpError>,
> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/flexible-ip/v1alpha1/zones/{zone}/fips/detach",
        configuration.base_path,
        zone = crate::apis::urlencode(zone)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Auth-Token", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&inline_object56);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DetachFlexibleIpError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Duplicate a Virtual MAC from a given Flexible IP onto another attached on the same server.
pub async fn duplicate_mac_addr(
    configuration: &configuration::Configuration,
    zone: &str,
    fip_id: &str,
    inline_object53: crate::models::InlineObject53,
) -> Result<crate::models::ScalewayFlexibleIpV1alpha1FlexibleIp, Error<DuplicateMacAddrError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}/mac/duplicate",
        configuration.base_path,
        zone = crate::apis::urlencode(zone),
        fip_id = crate::apis::urlencode(fip_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Auth-Token", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&inline_object53);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DuplicateMacAddrError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn generate_mac_addr(
    configuration: &configuration::Configuration,
    zone: &str,
    fip_id: &str,
    inline_object52: crate::models::InlineObject52,
) -> Result<crate::models::ScalewayFlexibleIpV1alpha1FlexibleIp, Error<GenerateMacAddrError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}/mac",
        configuration.base_path,
        zone = crate::apis::urlencode(zone),
        fip_id = crate::apis::urlencode(fip_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Auth-Token", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&inline_object52);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GenerateMacAddrError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_flexible_ip(
    configuration: &configuration::Configuration,
    zone: &str,
    fip_id: &str,
) -> Result<crate::models::ScalewayFlexibleIpV1alpha1FlexibleIp, Error<GetFlexibleIpError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}",
        configuration.base_path,
        zone = crate::apis::urlencode(zone),
        fip_id = crate::apis::urlencode(fip_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Auth-Token", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetFlexibleIpError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list_flexible_ips(
    configuration: &configuration::Configuration,
    zone: &str,
    order_by: Option<&str>,
    page: Option<f32>,
    page_size: Option<f32>,
    tags: Option<Vec<String>>,
    status: Option<Vec<crate::models::ScalewayFlexibleIpV1alpha1FlexibleIpStatus>>,
    server_ids: Option<Vec<String>>,
    organization_id: Option<&str>,
    project_id: Option<&str>,
) -> Result<
    crate::models::ScalewayFlexibleIpV1alpha1ListFlexibleIpsResponse,
    Error<ListFlexibleIpsError>,
> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/flexible-ip/v1alpha1/zones/{zone}/fips",
        configuration.base_path,
        zone = crate::apis::urlencode(zone)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = order_by {
        local_var_req_builder =
            local_var_req_builder.query(&[("order_by", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder =
            local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("page_size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = tags {
        local_var_req_builder = local_var_req_builder.query(&[(
            "tags",
            &local_var_str
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(",")
                ,
        )]);
    }
    if let Some(ref local_var_str) = status {
        local_var_req_builder = local_var_req_builder.query(&[(
            "status",
            &local_var_str
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(",")
                ,
        )]);
    }
    if let Some(ref local_var_str) = server_ids {
        local_var_req_builder = local_var_req_builder.query(&[(
            "server_ids",
            &local_var_str
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(",")
                ,
        )]);
    }
    if let Some(ref local_var_str) = organization_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("organization_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = project_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("project_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Auth-Token", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListFlexibleIpsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_flexible_ip(
    configuration: &configuration::Configuration,
    zone: &str,
    fip_id: &str,
    inline_object51: crate::models::InlineObject51,
) -> Result<crate::models::ScalewayFlexibleIpV1alpha1FlexibleIp, Error<UpdateFlexibleIpError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}",
        configuration.base_path,
        zone = crate::apis::urlencode(zone),
        fip_id = crate::apis::urlencode(fip_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Auth-Token", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&inline_object51);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateFlexibleIpError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
