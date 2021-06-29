/*
 * Instance API
 *
 * # Introduction  ## Endpoints  Scaleway instance API can be reach on  - `https://api.scaleway.com/instance/v1/zones/fr-par-1` - `https://api.scaleway.com/instance/v1/zones/fr-par-2` - `https://api.scaleway.com/instance/v1/zones/nl-ams-1` - `https://api.scaleway.com/instance/v1/zones/pl-waw-1`  Older endpoints are still reachable but should not be used for new projects  - `https://cp-par1.scaleway.com` - `https://cp-ams1.scaleway.com`  <Example>  The following code is an example request to retrieve detailed information about a volume:  ``` % curl -H 'X-Auth-Token: xxxxxxxx-xxxx-xxxxx-xxxx-xxxxxxxxxxxxx' -H 'Content-Type: application/json' https://api.scaleway.com/instance/v1/zones/fr-par-1/volumes/f929fe39-63f8-4be8-a80e-1e9c8ae22a76 -i  HTTP/1.1 200 OK Server: nginx Date: Thu, 22 May 2014 07:55:00 GMT Content-Type: application/json Content-Length: 1345 Connection: keep-alive Strict-Transport-Security: max-age=86400  {   \"volumes\": [     {       \"export_uri\": null,       \"id\": \"f929fe39-63f8-4be8-a80e-1e9c8ae22a76\",       \"name\": \"volume-0-1\",       \"organization\": \"000a115d-2852-4b0a-9ce8-47f1134ba95a\",       \"server\": null,       \"size\": 10000000000,       \"volume_type\": \"l_ssd\"     },     {       \"export_uri\": null,       \"id\": \"0facb6b5-b117-441a-81c1-f28b1d723779\",       \"name\": \"volume-0-2\",       \"organization\": \"000a115d-2852-4b0a-9ce8-47f1134ba95a\",       \"server\": null,       \"size\": 20000000000,       \"volume_type\": \"l_ssd\"     }   ] } ```  </Example>  ## Pagination  Most of listing requests receive a paginated response.  **Paginated request**  Requests against paginated endpoints accept two `query` arguments:  - `page`, a positive integer to choose the page to return. - `per_page`, an positive integer lower or equal to 100 to select the number of   items to return. The default value is `50`.  Paginated endpoints usually also accept filters to search and sort results. These filters are documented along each endpoint documentation.  **Paginated response**  ```bash % curl -H 'X-Auth-Token: <token>' 'https://api.scaleway.com/instance/v1/zones/fr-par-1/images/?page=2&per_page=10' -i HTTP/1.0 200 OK [...] X-Total-Count: 209 [...] ```  The `X-Total-Count` header contains the total number of items for the resource. 
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// K8sV1RegionsRegionClustersOpenIdConnectConfig : This feature is in ALPHA state, it may be deleted or modified. This configuration enables to set the [OpenID Connect configuration](https://kubernetes.io/docs/reference/access-authn-authz/authentication/#openid-connect-tokens) of the Kubernetes API server.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct K8sV1RegionsRegionClustersOpenIdConnectConfig {
    /// URL of the provider which allows the API server to discover public signing keys. Only URLs which use the `https://` scheme are accepted. This is typically the provider's discovery URL without a path, for example \"https://accounts.google.com\" or \"https://login.salesforce.com\". This URL should point to the level below .well-known/openid-configuration. 
    #[serde(rename = "issuer_url", skip_serializing_if = "Option::is_none")]
    pub issuer_url: Option<String>,
    /// A client id that all tokens must be issued for
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// JWT claim to use as the user name. By default `sub`, which is expected to be a unique identifier of the end user. Admins can choose other claims, such as `email` or `name`, depending on their provider. However, claims other than `email` will be prefixed with the issuer URL to prevent naming clashes with other plugins. 
    #[serde(rename = "username_claim", skip_serializing_if = "Option::is_none")]
    pub username_claim: Option<String>,
    /// Prefix prepended to username claims to prevent clashes with existing names (such as `system:` users). For example, the value `oidc:` will create usernames like `oidc:jane.doe`. If this flag isn't provided and `username_claim` is a value other than `email` the prefix defaults to `( Issuer URL )#` where `( Issuer URL )` is the value of `issuer_url`. The value `-` can be used to disable all prefixing. 
    #[serde(rename = "username_prefix", skip_serializing_if = "Option::is_none")]
    pub username_prefix: Option<String>,
    /// JWT claim to use as the user's group
    #[serde(rename = "groups_claim", skip_serializing_if = "Option::is_none")]
    pub groups_claim: Option<Vec<String>>,
    /// Prefix prepended to group claims to prevent clashes with existing names (such as `system:` groups). For example, the value `oidc:` will create group names like `oidc:engineering` and `oidc:infra`. 
    #[serde(rename = "groups_prefix", skip_serializing_if = "Option::is_none")]
    pub groups_prefix: Option<String>,
    /// Multiple key=value pairs that describes a required claim in the ID Token. If set, the claims are verified to be present in the ID Token with a matching value. 
    #[serde(rename = "required_claim", skip_serializing_if = "Option::is_none")]
    pub required_claim: Option<Vec<String>>,
}

impl K8sV1RegionsRegionClustersOpenIdConnectConfig {
    /// This feature is in ALPHA state, it may be deleted or modified. This configuration enables to set the [OpenID Connect configuration](https://kubernetes.io/docs/reference/access-authn-authz/authentication/#openid-connect-tokens) of the Kubernetes API server.
    pub fn new() -> K8sV1RegionsRegionClustersOpenIdConnectConfig {
        K8sV1RegionsRegionClustersOpenIdConnectConfig {
            issuer_url: None,
            client_id: None,
            username_claim: None,
            username_prefix: None,
            groups_claim: None,
            groups_prefix: None,
            required_claim: None,
        }
    }
}


