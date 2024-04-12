use crate::models::prelude::*;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

/// Represents the response to a request for listing all assets from the Stellar Horizon API.
///
/// This struct encapsulates the complete response returned by the Horizon server when querying
/// for all assets. It includes navigational links for pagination and a collection of detailed
/// asset records, providing a comprehensive view of the assets available on the Stellar network.
///
#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
pub struct AllAssetsResponse {
    /// A `Links` struct containing navigational links. These links are used for
    ///   pagination purposes, allowing access to the current, next, and previous pages of the asset list.
    #[serde(rename = "_links")]
    links: ResponseLinks,
    ///
    /// An `Embedded` struct that contains the actual list of asset records. Each
    ///   record in this list provides detailed information about an individual asset, including
    ///   its type, issuer, and various statistics related to its distribution and usage.
    #[serde(rename = "_embedded")]
    embedded: Embedded<Asset>,
}

/// Represents a single navigational or related link in the all assets response from the Stellar Horizon API.
///
/// This struct is used to define individual links within the asset response, providing URLs to related
/// resources or additional information. It is a component of the [`Links`] struct within the [`AllAssetsResponse`].
///
#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
pub struct AssetTomlLink {
    /// The URL of the linked resource. This field is used to provide direct access to relevant resources
    ///     or additional data related to an asset.
    href: Option<String>,
    /// A link to a TOML file associated with the asset. The TOML file typically contains metadata about
    ///     the asset, such as details about the issuer.
    toml: Option<Link>,
}

/// Represents a single asset record in the all assets response from the Horizon API.
///
/// This struct contains detailed information about an individual asset, including its type, code, issuer,
/// and various other statistics and flags.
///
#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
pub struct Asset {
    /// Links related to the asset, including a link to the asset's TOML file.
    _links: AssetTomlLink,
    /// The type of the asset, such as "native" for lumens or "credit_alphanum4" and
    ///   "credit_alphanum12" for other assets.
    asset_type: String,
    /// The code of the asset. For native lumens, this is "XLM".
    asset_code: String,
    /// The issuer of the asset. For native lumens, this field is empty.
    asset_issuer: String,
    /// A token used for paging through results.
    paging_token: String,
    /// The number of accounts holding the asset.
    num_accounts: u32,
    /// The number of claimable balances for the asset.
    num_claimable_balances: u32,
    /// The number of liquidity pools that include the asset.
    num_liquidity_pools: u32,
    /// The number of contracts involving the asset.
    num_contracts: u32,
    /// The total amount of the asset.
    amount: String,
    /// An `AccountInfo` struct detailing the number of accounts holding the asset,
    ///   categorized by their authorization status.
    accounts: AccountInfo,
    /// The total amount of the asset in claimable balances.
    claimable_balances_amount: String,
    /// The total amount of the asset in liquidity pools.
    liquidity_pools_amount: String,
    /// The total amount of the asset in contracts.
    contracts_amount: String,
    /// An `AccountBalances` struct detailing the total balances held in authorized,
    ///   authorized to maintain liabilities, and unauthorized accounts.
    balances: AccountBalances,
    /// A `Flags` struct representing various flags related to asset control and authorization.
    flags: Flags,
}

/// Represents account authorization information within an asset record in the all assets response.
///
/// This struct provides a breakdown of the number of accounts associated with a particular asset,
/// categorized by their trustline flag state.
///
#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
pub struct AccountInfo {
    /// The number of accounts that are authorized to transact with the asset.
    authorized: u32,
    /// The number of accounts authorized to maintain liabilities
    ///   for the asset. This status allows accounts to hold the asset without the ability to transact.
    authorized_to_maintain_liabilities: u32,
    /// The number of accounts that are not authorized to transact with the asset.
    unauthorized: u32,
}

/// Represents the balance details for different authorization states of an account.
///
/// Details the balances held in accounts associated with a particular asset, categorized by
/// their trustline flag state. It provides  insight into the distribution of an asset
/// across various account types in the Stellar network.
///
#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
pub struct AccountBalances {
    /// A `String` representing the total balance of the asset held by accounts that
    ///   are authorized to transact with it.
    authorized: String,
    /// A`String` representing the total balance of the asset
    ///   held by accounts authorized to maintain liabilities but not transact.
    authorized_to_maintain_liabilities: String,
    /// A `String` representing the total balance of the asset held by accounts that
    ///   are not authorized to transact with it.
    unauthorized: String,
}

impl Response for AllAssetsResponse {
    fn from_json(json: String) -> Result<Self, String> {
        let response = serde_json::from_str(&json).map_err(|e| e.to_string())?;

        Ok(response)
    }
}
