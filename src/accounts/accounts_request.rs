use crate::{models::*, BuildQueryParametersExt};

/// Defines types for filtering the list of accounts retrieved.
///
/// This module provides a set of filter types used in [`AccountsRequest`]
/// to specify the criteria for filtering the list of accounts returned by the Horizon server.
/// Each filter type corresponds to a potential query parameter that can be used in account-
/// related queries. Exactly one filter is required by the API in the request.
///
/// # Usage
/// To use these filters, create an instance of [`AccountsRequest`]
/// and call one of its setter methods to set exactly one of the filters. The request can then be executed through the `HorizonClient`.
///
/// ```rust
/// # use stellar_rs::accounts::prelude::*;
/// # use stellar_rs::accounts::accounts_request::filters::*;
/// # use stellar_rs::models::Request;
/// # use stellar_rs::horizon_client::HorizonClient;
/// #
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let base_url = "https://horizon-testnet.stellar.org".to_string();
/// # let horizon_client = HorizonClient::new(base_url)
/// #    .expect("Failed to create Horizon Client");
/// let request = AccountsRequest::new()
///     .set_signer_filter("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7")
///     .unwrap();
///
/// // Use with HorizonClient::get_account_list
/// # Ok({})
/// # }
/// ```
///
/// These filter types are designed to be used exclusively with `AccountsRequest` and are not intended
/// for direct use in API calls.
///
pub mod filters {
    use crate::models::Asset;

    /// Represents a filter for accounts sponsored by the given account ID.
    #[derive(Default, Clone)]
    pub struct SponsorFilter(pub String);
    /// Indicates the absence of a sponsor filter in the request.
    #[derive(Default, Clone)]
    pub struct NoSponsorFilter;

    /// Represents a filter for accounts that have the given account ID as a signer.
    #[derive(Default, Clone)]
    pub struct SignerFilter(pub String);
    /// Indicates the absence of a signer filter in the request.
    #[derive(Default, Clone)]
    pub struct NoSignerFilter;

    /// Represents a filter for accounts holding a trustline for the specified asset.
    #[derive(Clone)]
    pub struct AssetFilter<T>(pub Asset<T>);
    /// Indicates the absence of an asset filter in the request.
    #[derive(Default, Clone)]
    pub struct NoAssetFilter;

    /// Represents a filter for accounts associated with the specified liquidity pool.
    #[derive(Default, Clone)]
    pub struct LiquidityPoolFilter(pub String);
    /// Indicates the absence of a liquidity pool filter in the request.
    #[derive(Default, Clone)]
    pub struct NoLiquidityPoolFilter;
}

use filters::*;

/// Macro to implement the `Request` trait for `AccountsRequest` variants.
///
/// This macro generates an implementation of the [`Request`] trait for a specified [`AccountsRequest`] type.
/// It's utilized to create specific request handlers for different account query filters such as by sponsor,
/// signer, asset, or liquidity pool.
///
/// # Parameters
/// - `$type`: The type of [`AccountsRequest`] for which to implement the [`Request`] trait. This type must already
///   conform to the structure expected by the Horizon API for account requests.
/// - `$field`: The field within the `$type` that is being used as a filter for the account request. This field
///   is included as a mandatory parameter in the query.
///
/// # Provided Methods
/// - `get_query_parameters`: Constructs the query string from the fields of the `$type`, including cursor, limit,
///   order, and the specified `$field` as a filter parameter.
/// - `build_url`: Assembles the complete URL for the account request using the base URL and the constructed query
///   parameters.
///
/// # Note
/// - The macro is intended for internal SDK use and contributes to the modularity of the account request system.
/// - The `.$field.0` syntax assumes that the filter field within the [`AccountsRequest`] type is a tuple struct with
///   the actual filter value as its first item.
/// - The macro includes error handling to ensure that only the appropriate fields are included in the query parameters.
///
macro_rules! valid_account_request_impl {
    ($type:ty, $field:ident) => {
        impl Request for $type {
            fn get_query_parameters(&self) -> String {
                let mut params = vec![
                    self.cursor.as_ref().map(|c| format!("cursor={}", c)),
                    self.limit.as_ref().map(|l| format!("limit={}", l)),
                    self.order.as_ref().map(|o| format!("order={}", o)),
                ];

                params.push(Some(format!("{}={}", stringify!($field), self.$field.0)));

                params.build_query_parameters()
            }

            fn build_url(&self, base_url: &str) -> String {
                format!(
                    "{}/{}{}",
                    base_url,
                    super::ACCOUNTS_PATH,
                    self.get_query_parameters()
                )
            }
        }
    };
}

/// Macro to implement the `Request` trait for generic `AccountsRequest` variants.
///
/// This macro generates an implementation of the [`Request`] trait for a specified [`AccountsRequest`] type.
/// It's utilized to create specific request handlers for different account query filters such as by sponsor,
/// signer, asset, or liquidity pool.
///
/// # Parameters
/// - `$type`: The type of [`AccountsRequest`] for which to implement the [`Request`] trait. This type must already
///   conform to the structure expected by the Horizon API for account requests.
/// - `$field`: The field within the `$type` that is being used as a filter for the account request. This field
///   is included as a mandatory parameter in the query.
/// - `$generic` : The generic type used for the [`AssetFilter`] when querying accounts.
///
/// # Provided Methods
/// - `get_query_parameters`: Constructs the query string from the fields of the `$type`, including cursor, limit,
///   order, and the specified `$field` as a filter parameter.
/// - `build_url`: Assembles the complete URL for the account request using the base URL and the constructed query
///   parameters.
///
/// # Note
/// - The macro is intended for internal SDK use and contributes to the modularity of the account request system.
/// - The `.$field.0` syntax assumes that the filter field within the [`AccountsRequest`] type is a tuple struct with
///   the actual filter value as its first item.
/// - The macro includes error handling to ensure that only the appropriate fields are included in the query parameters.
///
macro_rules! valid_generic_account_request_impl {
    ($type:ty, $field:ident, $generic:ident) => {
        impl<$generic> Request for $type
        where
            Asset<T>: std::fmt::Display,
        {
            fn get_query_parameters(&self) -> String {
                let mut params = vec![
                    self.cursor.as_ref().map(|c| format!("cursor={}", c)),
                    self.limit.as_ref().map(|l| format!("limit={}", l)),
                    self.order.as_ref().map(|o| format!("order={}", o)),
                ];

                params.push(Some(format!("{}={}", stringify!($field), self.$field.0)));

                params.build_query_parameters()
            }

            fn build_url(&self, base_url: &str) -> String {
                format!(
                    "{}/{}{}",
                    base_url,
                    super::ACCOUNTS_PATH,
                    self.get_query_parameters()
                )
            }
        }
    };
}

/// Specifies the requirements for a valid account request.
///
/// This trait ensures that any request structure intended to fetch account data from the
/// Horizon server satisfies the necessary interface defined by the `Request` trait. It serves as
/// a marker trait that categorically identifies valid account request types.
///
/// # Implementations
/// The trait is implemented by various configurations of the `AccountsRequest` struct, each tailored
/// to filter the account data based on different criteria:
/// - `AccountsRequest<Sponsor, NoSigner, NoAsset, NoLiquidityPool>`: Requests accounts by sponsor.
/// - `AccountsRequest<NoSponsor, Signer, NoAsset, NoLiquidityPool>`: Requests accounts by signer.
/// - `AccountsRequest<NoSponsor, NoSigner, Asset, NoLiquidityPool>`: Requests accounts by asset.
/// - `AccountsRequest<NoSponsor, NoSigner, NoAsset, LiquidityPool>`: Requests accounts by liquidity pool.
///
/// # Usage
/// You generally do not need to use `ValidAccountsRequest` directly; it is used internally by the SDK.
/// Instead, create an instance of [`AccountsRequest`] with the desired filters and pass it to the
/// [`HorizonClient::get_account_list`](crate::horizon_client::HorizonClient::get_account_list) method.
///
/// ```rust
/// # use stellar_rs::accounts::prelude::AccountsRequest;
/// # use stellar_rs::horizon_client::HorizonClient;
/// # use stellar_rs::models::Request;
/// # let horizon_client = HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();
/// let request = AccountsRequest::new()
///     .set_sponsor_filter("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7".to_string())
///     .unwrap();
/// // Now, you can pass `request` to `horizon_client.get_account_list`.
/// ```
///
pub trait ValidAccountsRequest: Request {}

impl ValidAccountsRequest
    for AccountsRequest<SponsorFilter, NoSignerFilter, NoAssetFilter, NoLiquidityPoolFilter>
{
}
valid_account_request_impl!(AccountsRequest<SponsorFilter, NoSignerFilter, NoAssetFilter, NoLiquidityPoolFilter>, sponsor);

impl ValidAccountsRequest
    for AccountsRequest<NoSponsorFilter, SignerFilter, NoAssetFilter, NoLiquidityPoolFilter>
{
}
valid_account_request_impl!(AccountsRequest<NoSponsorFilter, SignerFilter, NoAssetFilter, NoLiquidityPoolFilter>, signer);

impl<T> ValidAccountsRequest
    for AccountsRequest<NoSponsorFilter, NoSignerFilter, AssetFilter<T>, NoLiquidityPoolFilter>
where
    Asset<T>: std::fmt::Display,
{
}
valid_generic_account_request_impl!(AccountsRequest<NoSponsorFilter, NoSignerFilter, AssetFilter<T>, NoLiquidityPoolFilter>, asset, T);

impl ValidAccountsRequest
    for AccountsRequest<NoSponsorFilter, NoSignerFilter, NoAssetFilter, LiquidityPoolFilter>
{
}
valid_account_request_impl!(AccountsRequest<NoSponsorFilter, NoSignerFilter, NoAssetFilter, LiquidityPoolFilter>, liquidity_pool);

/// Represents a request to fetch multiple accounts from the Horizon API with a specific filter.
///
/// `AccountsRequest` is a struct used to query a list of accounts on the Horizon API, allowing
/// filtering based on various criteria such as sponsor, signer, asset or liquidity pool.
/// This struct is designed to be used in conjunction with the [`HorizonClient::get_account_list`](crate::horizon_client::HorizonClient::get_account_list) method.
///
/// The struct matches the parameters necessary to construct a request for the
/// <a href="https://developers.stellar.org/api/horizon/resources/list-all-accounts">List All Accounts endpoint</a>
/// of the Horizon API.
///
/// # Filters
///
/// At least one of the following filters is required:
/// - `sponsor`: Account ID of the sponsor. Filters for accounts sponsored by the account ID or have a subentry (trustline, offer, or data entry) which is sponsored by the given account ID.
/// - `signer`: Account ID of the signer. Filters for accounts that have the given account ID as a signer.
/// - `asset`: An issued asset in the format “Code:IssuerAccountID”. Filters for accounts with a trustline for the specified asset.
/// - `liquidity_pool`: The liquidity pool ID. Filters for accounts associated with the specified liquidity pool.
///
/// # Optional Parameters
///
/// - `cursor`: A number that points to the current location in the collection of responses and is pulled from the paging_token value of a record.
/// - `limit`: The maximum number of records to return, with a permissible range from 1 to 200.
///   Defaults to 10 if not specified.
/// - `order`: The [`Order`] of the returned records, either ascending ([`Order::Asc`]) or descending ([`Order::Desc`]).
///   Defaults to ascending if not set.
///
#[derive(Default)]
pub struct AccountsRequest<Sp, Si, A, L> {
    /// Filter for accounts sponsored by the account ID or have a subentry
    /// (trustline, offer, or data entry) which is sponsored by the given account ID.
    sponsor: Sp,

    /// Filter for accounts that have the given account ID as a signer.
    signer: Si,

    /// Filter for accounts with a trustline for the specified asset.
    asset: A,

    /// Filter for accounts associated with the specified liquidity pool.
    liquidity_pool: L,

    /// A number that points to the current location in the collection of responses and is pulled from the paging_token value of a record.
    cursor: Option<u32>,

    /// The maximum number of records to return, with a permissible range from 1 to 200.
    ///   Defaults to 10 if not specified.
    limit: Option<u32>,

    /// The [`Order`] of the returned records, either ascending ([`Order::Asc`]) or descending ([`Order::Desc`]).
    order: Option<Order>,
}

impl<Sp, Si, A, L> AccountsRequest<Sp, Si, A, L> {
    /// Sets the cursor for pagination.
    ///
    /// # Arguments
    /// * `cursor` - A `u32` value pointing to a specific location in a collection of responses.
    ///
    pub fn set_cursor(self, cursor: u32) -> Result<Self, String> {
        if cursor < 1 {
            return Err("cursor must be greater than or equal to 1".to_string());
        }

        Ok(Self {
            cursor: Some(cursor),
            ..self
        })
    }

    /// Sets the maximum number of records to return.
    ///
    /// # Arguments
    /// * `limit` - A `u8` value specifying the maximum number of records. Range: 1 to 200. Defaults to 10.
    ///
    pub fn set_limit(self, limit: u32) -> Result<Self, String> {
        if limit < 1 || limit > 200 {
            return Err("limit must be between 1 and 200".to_string());
        }

        Ok(Self {
            limit: Some(limit),
            ..self
        })
    }

    /// Sets the order of the returned records.
    ///
    /// # Arguments
    /// * `order` - An [`Order`] enum value specifying the order (ascending or descending).
    ///
    pub fn set_order(self, order: Order) -> Self {
        Self {
            order: Some(order),
            ..self
        }
    }
}

/// Since the Horizon API only allows for one of the following parameters to be set, we need to
/// create an implementation for a combination of generics which are all unset.
impl AccountsRequest<NoSponsorFilter, NoSignerFilter, NoAssetFilter, NoLiquidityPoolFilter> {
    /// Creates a new `AccountsRequest` with default parameters.
    pub fn new() -> Self {
        AccountsRequest::default()
    }

    /// Sets the sponsor account ID filter.
    ///
    /// # Arguments
    /// * `sponsor` - A `String` specifying the sponsor account ID. Filters for accounts
    /// sponsored by this ID or having a subentry sponsored by this ID.
    ///
    pub fn set_sponsor_filter(
        self,
        sponsor: String,
    ) -> Result<
        AccountsRequest<SponsorFilter, NoSignerFilter, NoAssetFilter, NoLiquidityPoolFilter>,
        String,
    > {
        if let Err(e) = is_public_key(&sponsor) {
            return Err(e.to_string());
        }

        Ok(AccountsRequest {
            sponsor: SponsorFilter(sponsor.into()),
            cursor: self.cursor,
            limit: self.limit,
            order: self.order,
            ..Default::default()
        })
    }

    /// Sets the signer account ID filter.
    ///
    /// # Arguments
    /// * `signer` - A `String` specifying the signer account ID. Filters for accounts
    /// having this ID as a signer.
    ///
    pub fn set_signer_filter(
        self,
        signer: &str,
    ) -> Result<
        AccountsRequest<NoSponsorFilter, SignerFilter, NoAssetFilter, NoLiquidityPoolFilter>,
        String,
    > {
        if let Err(e) = is_public_key(&signer) {
            return Err(e.to_string());
        }

        Ok(AccountsRequest {
            signer: SignerFilter(signer.to_string()),
            cursor: self.cursor,
            limit: self.limit,
            order: self.order,
            ..Default::default()
        })
    }

    /// Sets the asset filter.
    ///
    /// # Arguments
    /// * `asset` - An [`Asset`] specifying the asset. Filters for accounts with a
    /// trustline for this asset.
    ///
    pub fn set_asset_filter<T>(
        self,
        asset: Asset<T>,
    ) -> AccountsRequest<NoSponsorFilter, NoSignerFilter, AssetFilter<T>, NoLiquidityPoolFilter>
    {
        AccountsRequest {
            sponsor: self.sponsor,
            signer: self.signer,
            asset: AssetFilter(asset),
            liquidity_pool: self.liquidity_pool,
            cursor: self.cursor,
            limit: self.limit,
            order: self.order,
        }
    }

    /// Sets the liquidity pool filter.
    ///
    /// # Arguments
    /// * `liquidity_pool` - A `String` representing the liquidity pool ID. Filters for accounts
    /// associated with the specified liquidity pool.
    ///
    pub fn set_liquidity_pool_filter(
        self,
        liquidity_pool: impl Into<String>,
    ) -> AccountsRequest<NoSponsorFilter, NoSignerFilter, NoAssetFilter, LiquidityPoolFilter> {
        AccountsRequest {
            liquidity_pool: LiquidityPoolFilter(liquidity_pool.into()),
            cursor: self.cursor,
            limit: self.limit,
            order: self.order,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accounts_request_set_sponsor_filter() {
        let request = AccountsRequest::new().set_sponsor_filter("sponsor".to_string());

        assert!(request.is_err());
    }

    #[test]
    fn test_accounts_set_sponsor_valid() {
        let request = AccountsRequest::new()
            .set_sponsor_filter(
                "GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7".to_string(),
            )
            .unwrap();
        assert_eq!(
            request.sponsor.0,
            "GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7"
        );
    }

    #[test]
    fn test_set_cursor_valid() {
        let request = AccountsRequest::new().set_cursor(12345).unwrap();
        assert_eq!(request.cursor.unwrap(), 12345);
    }

    #[test]
    fn test_set_cursor_invalid() {
        let request = AccountsRequest::new().set_cursor(0);
        assert_eq!(
            request.err().unwrap(),
            "cursor must be greater than or equal to 1".to_string()
        );
    }

    #[test]
    fn test_set_limit_valid() {
        let request = AccountsRequest::new().set_limit(20).unwrap();
        assert_eq!(request.limit.unwrap(), 20);
    }

    #[test]
    fn test_set_limit_invalid_low() {
        let request = AccountsRequest::new().set_limit(0);
        assert_eq!(
            request.err().unwrap(),
            "limit must be between 1 and 200".to_string()
        );
    }

    #[test]
    fn test_set_limit_invalid_high() {
        let request = AccountsRequest::new().set_limit(201);
        assert_eq!(
            request.err().unwrap(),
            "limit must be between 1 and 200".to_string()
        );
    }
}
