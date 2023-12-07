use crate::{BuildQueryParametersExt, models::*};

use super::super::AssetType;
use super::super::Order;

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
/// - `cursor`: A number that points to a specific location in a collection of responses and is pulled from the paging_token value of a record.
/// - `limit`: The maximum number of records to return, with a permissible range from 1 to 200. 
///   Defaults to 10 if not specified.
/// - `order`: The [`Order`] of the returned records, either ascending ([`Order::Asc`]) or descending ([`Order::Desc`]). 
///   Defaults to ascending if not set.
///
/// # Usage
///
/// Instances of `AccountsRequest` are created and configured using setter methods for each 
/// parameter.
/// ```
/// # use stellar_rust_sdk::accounts::accounts_request::AccountsRequest;
/// # use crate::stellar_rust_sdk::models::Request;
/// let mut request = AccountsRequest::new();
/// request
///     .set_signer("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7")
///     .set_limit(10);
/// // Use with HorizonClient::get_account_list
/// ```
///
pub struct AccountsRequest {
    sponsor: Option<String>,
    signer: Option<String>,
    asset: Option<AssetType>,
    cursor: Option<u32>,
    limit: Option<u8>,
    order: Option<Order>,
    liquidity_pool: Option<String>,
}

impl Request for AccountsRequest {
    fn new() -> Self {
        AccountsRequest {
            sponsor: None,
            signer: None,
            asset: None,
            cursor: None,
            limit: None,
            order: None,
            liquidity_pool: None,
        }
    }

    fn get_query_parameters(&self) -> String {
        vec![
            self.sponsor.as_ref().map(|s| format!("sponsor={}", s)),
            self.signer.as_ref().map(|s| format!("signer={}", s)),
            self.asset.as_ref().map(|a| format!("asset={}", a)),
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
            self.liquidity_pool
                .as_ref()
                .map(|lp| format!("liquidity_pool={}", lp)),
        ].build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}{}",
            base_url,
            super::ACCOUNTS_PATH,
            self.get_query_parameters()
        )
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(sponsor) = &self.sponsor {
            let is_valid = is_public_key(sponsor);
            if is_valid.is_err() {
                return Err(is_valid.unwrap_err());
            }
        }

        if let Some(signer) = &self.signer {
            let is_valid = is_public_key(signer);
            if is_valid.is_err() {
                return Err(is_valid.unwrap_err());
            }
        }

        if let Some(cursor) = &self.cursor {
            if *cursor < 1 {
                return Err("cursor must be greater than or equal to 1".to_string());
            }
        }

        if let Some(limit) = &self.limit {
            if *limit < 1 {
                return Err("limit must be greater than or equal to 1".to_string());
            }
            if *limit > 200 {
                return Err("limit must be less than or equal to 200".to_string());
            }
        }

        if self.signer.is_none() && self.sponsor.is_none() && self.asset.is_none() && self.liquidity_pool.is_none() {
            return Err("Exactly one filter is required. Please ensure that you are including a signer, sponsor, asset, or liquidity pool filter.".to_string());
        }

        Ok(())
    }
}

impl AccountsRequest {
    /// Sets the sponsor account ID filter.
    ///
    /// # Arguments
    /// * `sponsor` - A `String` specifying the sponsor account ID. Filters for accounts 
    /// sponsored by this ID or having a subentry sponsored by this ID.
    ///
    pub fn set_sponsor(&mut self, sponsor: impl Into<String>) -> &mut Self {
        self.sponsor = Some(sponsor.into());
        self
    }

    /// Sets the signer account ID filter.
    ///
    /// # Arguments
    /// * `signer` - A `String` specifying the signer account ID. Filters for accounts 
    /// having this ID as a signer.
    ///
    pub fn set_signer(&mut self, signer: &str) -> &mut Self {
        self.signer = Some(signer.to_owned());
        self
    }

    /// Sets the asset filter.
    ///
    /// # Arguments
    /// * `asset` - An [`AssetType`] specifying the asset. Filters for accounts with a 
    /// trustline for this asset.
    ///
    pub fn set_asset(&mut self, asset: AssetType) -> &mut Self {
        self.asset = Some(asset);
        self
    }

    /// Sets the cursor for pagination.
    ///
    /// # Arguments
    /// * `cursor` - A `u32` value pointing to a specific location in a collection of responses.
    /// 
    pub fn set_cursor(&mut self, cursor: u32) -> &mut Self {
        self.cursor = Some(cursor);
        self
    }

    /// Sets the maximum number of records to return.
    ///
    /// # Arguments
    /// * `limit` - A `u8` value specifying the maximum number of records. Range: 1 to 200. Defaults to 10.
    ///
    pub fn set_limit(&mut self, limit: u8) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the order of the returned records.
    ///
    /// # Arguments
    /// * `order` - An [`Order`] enum value specifying the order (ascending or descending).
    ///
    pub fn set_order(&mut self, order: Order) -> &mut Self {
        self.order = Some(order);
        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accounts_request() {
        let request = AccountsRequest::new();
        assert_eq!(
            request.build_url("https://horizon-testnet.stellar.org"),
            "https://horizon-testnet.stellar.org/accounts"
        );
    }

    #[test]
    fn test_accounts_request_set_sponsor() {
        let mut request = AccountsRequest::new();
        request.set_sponsor("sponsor");
        assert_eq!(request.sponsor, Some("sponsor".to_string()));
        // check that the validate throws an error
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Public key must be 56 characters long");
    }
}
