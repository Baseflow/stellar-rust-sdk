use crate::{models::*, BuildQueryParametersExt};

/// Represents the base and counter assets. Contains an enum of one of the possible asset types.
#[derive(PartialEq, Debug, Default)]
pub struct TradeAsset(AssetType);

// Contains the details of a non-native asset.
#[derive(PartialEq, Debug, Default)]
pub struct AssetData {
    pub asset_code: String,
    pub asset_issuer: String,
}

/// Represents the asset type of an asset.
#[derive(PartialEq, Debug, Default)]
pub enum AssetType {
    /// A native asset_type type. It holds no value.
    #[default]
    Native,
    /// An alphanumeric 4 asset_type type. It holds an Asset struct with asset code and asset issuer.
    Alphanumeric4(AssetData),
    /// An alphanumeric 12 asset_type type. It holds an Asset struct with asset code and asset issuer.
    Alphanumeric12(AssetData),
}

// TODO: Documentation
#[derive(PartialEq, Debug, Default)]
pub enum Resolution {
    #[default]
    Value60000,
    Value300000,
    Value900000,
    Value3600000,
    Value604800000,
}

impl std::fmt::Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Resolution::Value60000 => write!(f, "60000"),
            Resolution::Value300000 => write!(f, "300000"),
            Resolution::Value900000 => write!(f, "900000"),
            Resolution::Value3600000 => write!(f, "3600000"),
            Resolution::Value604800000 => write!(f, "604800000"),
        }
    }
}

// TODO: Documentation
#[derive(PartialEq, Default)]
pub struct TradeAggregationsRequest {
    /// The base asset of the trade aggregation.
    pub base_asset: TradeAsset,
    /// The counter asset of the trade.
    pub counter_asset: TradeAsset,
    // TODO: Documentation
    // TODO: test if this field is optional
    pub start_time: Option<i64>,
    // TODO: Documentation
    // TODO: test if this field is optional
    pub end_time: Option<i64>,
    // TODO: Documentation
    pub resolution: String,
    /// Specifies the maximum number of records to be returned in a single response.
    /// The range for this parameter is from 1 to 200. The default value is set to 10.
    pub limit: Option<u8>,
    /// Determines the [`Order`] of the records in the response. Valid options are [`Order::Asc`] (ascending)
    /// and [`Order::Desc`] (descending). If not specified, it defaults to ascending.
    pub order: Option<Order>,
}

impl TradeAggregationsRequest {
    /// Creates a new `TradeAggregationsRequest` with default parameters.
    pub fn new() -> Self {
        TradeAggregationsRequest::default()
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
    /// The updated `TradeAggregationsRequest` with the base asset set.    
    pub fn set_base_asset(
        self,
        base_asset: AssetType,
    ) -> Result<TradeAggregationsRequest, String> {
        Ok(TradeAggregationsRequest {
            base_asset: TradeAsset(base_asset),
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
    /// The updated `TradeAggregationsRequest` with the counter asset set.    
    pub fn set_counter_asset(
        self,
        counter_asset: AssetType,
    ) -> Result<TradeAggregationsRequest, String> {
        Ok(TradeAggregationsRequest {
            counter_asset: TradeAsset(counter_asset),
            ..self
        })
    }

    // TODO: Documentation
    pub fn set_start_time(
        self,
        start_time: Option<i64>,
    ) -> Result<TradeAggregationsRequest, String> {
        Ok(TradeAggregationsRequest {
            start_time,
            ..self
        })
    }

    // TODO: Documentation
    pub fn set_end_time(
        self,
        end_time: Option<i64>,
    ) -> Result<TradeAggregationsRequest, String> {
        Ok(TradeAggregationsRequest {
            end_time,
            ..self
        })
    }

    // TODO: Documentation
    pub fn set_resolution(
        self,
        resolution: String,
    ) -> Result<TradeAggregationsRequest, String> {
        Ok(TradeAggregationsRequest {
            resolution,
            ..self
        })
    }

    // TODO: Documentation
    pub fn set_limit(self, limit: u8) -> Result<Self, String> {
        // Validate limit if necessary
        if !(1..=200).contains(&limit) {
            Err("Limit must be between 1 and 200.".to_string())
        } else {
            Ok(Self { limit: Some(limit), ..self })
        }
    }

    // TODO: Documentation
    pub fn set_order(self, order: Order) -> Result<Self, String> {
        // No validation required for setting the order in this context
        Ok(Self { order: Some(order), ..self })
    }

}

impl Request for TradeAggregationsRequest {
    fn get_query_parameters(&self) -> String {
        // TODO: Documentation - describe query parameters string construction
        let mut asset_parameters: Vec<String> = Vec::new();
        match &self.base_asset.0 {
            AssetType::Native => {
                asset_parameters.push(format!("base_asset_type=native"));
            }
            AssetType::Alphanumeric4(asset) => {
                asset_parameters.push(format!("base_asset_type=credit_alphanum4"));
                asset_parameters.push(format!("&base_asset_code={}", asset.asset_code));
                asset_parameters.push(format!("&base_asset_issuer={}", asset.asset_issuer));
            }
            AssetType::Alphanumeric12(asset) => {
                asset_parameters.push(format!("base_asset_type=credit_alphanum12"));
                asset_parameters.push(format!("&base_asset_code={}", asset.asset_code));
                asset_parameters.push(format!("&base_asset_issuer={}", asset.asset_issuer));
            }
        }

        match &self.counter_asset.0 {
            AssetType::Native => {
                asset_parameters.push(format!("&counter_asset_type=native"));
            }
            AssetType::Alphanumeric4(asset) => {
                asset_parameters.push(format!("&counter_asset_type=credit_alphanum4"));
                asset_parameters.push(format!("&counter_asset_code={}", asset.asset_code));
                asset_parameters.push(format!("&counter_asset_issuer={}", asset.asset_issuer));
            }
            AssetType::Alphanumeric12(asset) => {
                asset_parameters.push(format!("&counter_asset_type=credit_alphanum12"));
                asset_parameters.push(format!("&counter_asset_code={}", asset.asset_code));
                asset_parameters.push(format!("&counter_asset_issuer={}", asset.asset_issuer));
            }
        }

        vec![
            Some(asset_parameters.join("")),
            Some(format!("resolution={}", self.resolution)),
            self.start_time.as_ref().map(|s| format!("start_time={}", s)),
            self.end_time.as_ref().map(|e| format!("end_time={}", e)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
        ].build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}{}",
            base_url,
            super::TRADE_AGGREGATIONS_PATH,
            self.get_query_parameters()
        )
    }
}