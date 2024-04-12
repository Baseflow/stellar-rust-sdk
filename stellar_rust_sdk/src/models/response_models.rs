use crate::Getters;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize, Clone, Getters)]
pub struct ResponseLinks {
    #[serde(rename = "self")]
    pub self_link: Link,
    pub next: Option<Link>,
    pub prev: Option<Link>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, Getters)]
pub struct Link {
    pub href: Option<String>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, Getters)]
pub struct Embedded<T> {
    pub records: Vec<T>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, Getters)]
pub struct TemplateLink {
    pub href: Option<String>,
    pub templated: Option<bool>,
}

/// Represents the authorization and control flags for an asset in the all assets response.
///
/// Details the various boolean flags that are
/// set for an asset, indicating specific permissions or restrictions. These flags define how the
/// asset is controlled and can be used within the Stellar network.
///
#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
pub struct Flags {
    /// A `bool` indicating whether authorization is required for an account to hold
    ///   or transact with the asset. If `true`, the issuer must approve account holders.
    auth_required: bool,
    /// A `bool` indicating whether the issuer has the ability to revoke the asset.
    ///   If `true`, the issuer can freeze the asset in user accounts.
    auth_revocable: bool,
    /// A `bool` indicating whether the asset's authorization flags can be changed
    ///   after issuance. If `true`, the issuer cannot change the `auth_required` and `auth_revocable` flags.
    auth_immutable: bool,
    /// A `bool` indicating whether the asset supports the clawback operation.
    ///   If `true`, the issuer can claw back the asset from user accounts.
    auth_clawback_enabled: bool,
}
