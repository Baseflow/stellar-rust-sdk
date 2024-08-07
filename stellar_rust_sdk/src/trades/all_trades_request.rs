use crate::models::*;
use stellar_rust_sdk_derive::Pagination;
use crate::models::prelude::AssetType;
use crate::Paginatable;

/// Represents the base and counter assets. Contains an enum of one of the possible asset types.
#[derive(PartialEq, Debug)]
pub struct TradeAsset(AssetType);

/// Represents a request to list all trades from the Stellar Horizon API.
///
/// This structure is used to construct a query to retrieve a comprehensive list of trades, which
/// can be filtered by the base asset, the counter asset and offer id. It adheres to the structure and parameters required
/// by the Horizon API for retrieving a
/// <a href="https://developers.stellar.org/network/horizon/api-reference/resources/get-all-trades">list of all trades</a>.
///
/// # Usage
///
/// Create an instance of this struct and set the desired query parameters to filter the list of trades.
/// Pass this request object to the [`HorizonClient::get_all_trades`](crate::horizon_client::HorizonClient::get_all_trades)
/// method to fetch the corresponding data from the Horizon API.
///
/// # Example
/// ```
/// use stellar_rs::{trades::prelude::*, models::*, Paginatable};
/// use stellar_rs::models::prelude::AssetType;
/// use stellar_rust_sdk_derive::Pagination;
///
/// let request = AllTradesRequest::new()
///     .set_base_asset(AssetType::Native).unwrap() // Optional selling asset filter
///     .set_cursor(123).unwrap() // Optional cursor for pagination
///     .set_limit(100).unwrap() // Optional limit for response records
///     .set_order(Order::Desc); // Optional order of records
///
/// // Use with HorizonClient::get_all_offers
/// ```
///
#[derive(PartialEq, Default, Pagination)]
pub struct AllTradesRequest {
    /// The base asset of the trade.
    pub base_asset: Option<TradeAsset>,
    /// The counter asset of the trade.
    pub counter_asset: Option<TradeAsset>,
    // The offer ID. Used to filter for trades originating from a specific offer.
    pub offer_id: Option<String>,
    /// A pointer to a specific location in a collection of responses, derived from the
    /// `paging_token` value of a record. Used for pagination control in the API response.
    pub cursor: Option<u32>,
    /// Specifies the maximum number of records to be returned in a single response.
    /// The range for this parameter is from 1 to 200. The default value is set to 10.
    pub limit: Option<u8>,
    /// Determines the [`Order`] of the records in the response. Valid options are [`Order::Asc`] (ascending)
    /// and [`Order::Desc`] (descending). If not specified, it defaults to ascending.
    pub order: Option<Order>,
}


impl AllTradesRequest {
    /// Creates a new `AllOffersRequest` with default parameters.
    pub fn new() -> Self {
        AllTradesRequest::default()
    }

    /// Specifies the base asset in the request.
    ///
    /// # Arguments
    ///
    /// * `base_asset` - The base asset type to filter the trades. It can be one of the following:
    ///     - `AssetType::Native`
    ///     - `AssetType::Alphanumeric4(AssetData)`
    ///     - `AssetType::Alphanumeric12(AssetData)`
    ///
    /// # Returns
    ///
    /// The updated `AllTradesRequest` with the base asset set.    
    pub fn set_base_asset(
        self,
        base_asset: AssetType,
    ) -> Result<AllTradesRequest, String> {
        Ok(AllTradesRequest {
            base_asset: Some(TradeAsset(base_asset)),
            ..self
        })
    }

    /// Specifies the counter asset in the request.
    ///
    /// # Arguments
    ///
    /// * `counter_asset` - The counter asset type to filter the trades. It can be one of the following:
    ///     - `AssetType::Native`
    ///     - `AssetType::Alphanumeric4(AssetData)`
    ///     - `AssetType::Alphanumeric12(AssetData)`
    ///
    /// # Returns
    ///
    /// The updated `AllTradesRequest` with the counter asset set.    
    pub fn set_counter_asset(
        self,
        counter_asset: AssetType,
    ) -> Result<AllTradesRequest, String> {
        Ok(AllTradesRequest {
            counter_asset: Some(TradeAsset(counter_asset)),
            ..self
        })
    }
}

impl Request for AllTradesRequest {
    fn get_query_parameters(&self) -> String {
        let mut query: Vec<String> = Vec::new();
        
        if let Some(base_asset) = &self.base_asset {
            match &base_asset.0 {
                AssetType::Native => {
                    query.push(format!("base_asset_type=native"));
                }
                AssetType::Alphanumeric4(asset) => {
                    query.push(format!("base_asset_type=credit_alphanum4"));
                    query.push(format!("&base_asset_code={}", asset.asset_code));
                    query.push(format!("&base_asset_issuer={}", asset.asset_issuer));
                }
                AssetType::Alphanumeric12(asset) => {
                    query.push(format!("base_asset_type=credit_alphanum12"));
                    query.push(format!("&base_asset_code={}", asset.asset_code));
                    query.push(format!("&base_asset_issuer={}", asset.asset_issuer));
                }
            }
        }

        if let Some(counter_asset) = &self.counter_asset {
            match &counter_asset.0 {
                AssetType::Native => {
                    query.push(format!("&counter_asset_type=native"));
                }
                AssetType::Alphanumeric4(asset) => {
                    query.push(format!("&counter_asset_type=credit_alphanum4"));
                    query.push(format!("&counter_asset_code={}", asset.asset_code));
                    query.push(format!("&counter_asset_issuer={}", asset.asset_issuer));
                }
                AssetType::Alphanumeric12(asset) => {
                    query.push(format!("&counter_asset_type=credit_alphanum12"));
                    query.push(format!("&counter_asset_code={}", asset.asset_code));
                    query.push(format!("&counter_asset_issuer={}", asset.asset_issuer));
                }
            }
        }
        query.join("")
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}?{}",
            base_url,
            super::TRADES_PATH,
            self.get_query_parameters()
        )
    }
}