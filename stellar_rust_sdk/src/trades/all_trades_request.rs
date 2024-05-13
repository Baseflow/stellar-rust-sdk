use crate::models::*;
use stellar_rust_sdk_derive::Pagination;
use crate::Paginatable;

// TODO: Documentation.
#[derive(PartialEq, Debug)]
pub struct BaseAsset(AssetType);

// TODO: Documentation.
#[derive(PartialEq, Debug)]
pub struct CounterAsset(AssetType);

// TODO: Documentation.
#[derive(PartialEq, Debug)]
pub struct AssetData {
    pub asset_code: String,
    pub asset_issuer: String,
}

/// Represents the asset type of an asset.
#[derive(PartialEq, Debug)]
pub enum AssetType {
    /// A native asset_type type. It holds no value.
    Native,
    /// An alphanumeric 4 asset_type type. It holds an Asset struct with asset code and asset issuer.
    Alphanumeric4(AssetData),
    /// An alphanumeric 12 asset_type type. It holds an Asset struct with asset code and asset issuer.
    Alphanumeric12(AssetData),
}

// TODO: Improve descriptive comments of base_asset and counter_asset
#[derive(PartialEq, Debug, Default, Pagination)]
pub struct AllTradesRequest {
    /// The base asset of the trade.
    pub base_asset: Option<BaseAsset>,
    /// The counter asset of the trade.
    pub counter_asset: Option<CounterAsset>,
    // ID to filter for trades originating from a specific offer.
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

// TODO: Documentation.
impl AllTradesRequest {
    pub fn new() -> Self {
        AllTradesRequest::default()
    }

    // TODO: Documentation.
    pub fn set_base_asset(
        self,
        base_asset: AssetType,
    ) -> Result<AllTradesRequest, String> {
        Ok(AllTradesRequest {
            base_asset: Some(BaseAsset(base_asset)),
            ..self
        })
    }

    // TODO: Documentation.
    pub fn set_counter_asset(
        self,
        counter_asset: AssetType,
    ) -> Result<AllTradesRequest, String> {
        Ok(AllTradesRequest {
            counter_asset: Some(CounterAsset(counter_asset)),
            ..self
        })
    }
}

// TODO: Documentation.
impl Request for AllTradesRequest {
    fn get_query_parameters(&self) -> String {
        let mut query: Vec<String> = Vec::new();
        
        if self.base_asset.as_ref().is_some() {
            match &self.base_asset.as_ref().unwrap().0 {
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

        if self.counter_asset.as_ref().is_some() {
            match &self.counter_asset.as_ref().unwrap().0 {
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