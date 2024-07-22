use crate::{models::*, BuildQueryParametersExt};

/// Represents the base asset. Contains an enum of one of the possible asset types.
#[derive(PartialEq, Debug)]
pub struct BaseAsset(AssetType);

/// Represents the absence of a base asset.
#[derive(PartialEq, Debug)]
pub struct NoBaseAsset;

/// Represents the counter asset. Contains an enum of one of the possible asset types.
#[derive(PartialEq, Debug)]
pub struct CounterAsset(AssetType);

/// Represents the absence of a counter asset.
#[derive(PartialEq, Debug)]
pub struct NoCounterAsset;

/// Contains the details of a non-native asset.
#[derive(PartialEq, Debug, Default)]
pub struct AssetData {
    pub asset_code: String,
    pub asset_issuer: String,
}

/// Represents the asset type of an asset.
#[derive(PartialEq, Debug)]
pub enum AssetType {
    /// A native asset_type type. It holds no value.
    // #[default]
    Native,
    /// An alphanumeric 4 asset_type type. It holds an Asset struct with asset code and asset issuer.
    Alphanumeric4(AssetData),
    /// An alphanumeric 12 asset_type type. It holds an Asset struct with asset code and asset issuer.
    Alphanumeric12(AssetData),
}

/// Represents the abcense of a resolution value.
#[derive(Default, Clone)]
pub struct NoResolution;

/// Represents the resolution value. It can contain a [`ResolutionData`] enum type.
#[derive(Default, Clone)]
pub struct Resolution(pub ResolutionData);

/// Represents the supported segment duration times in milliseconds. 
#[derive(PartialEq, Debug, Default, Clone)]
pub enum ResolutionData {
    #[default]
    Value60000,
    Value300000,
    Value900000,
    Value3600000,
    Value604800000,
}

impl std::fmt::Display for ResolutionData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ResolutionData::Value60000 => write!(f, "60000"), // 1 minute
            ResolutionData::Value300000 => write!(f, "300000"), // 5 minutes
            ResolutionData::Value900000 => write!(f, "900000"), // 15 minutes
            ResolutionData::Value3600000 => write!(f, "3600000"), // 1 day
            ResolutionData::Value604800000 => write!(f, "604800000"), // 1 week
        }
    }
}

/// Represents a request to list trade aggregations from the Stellar Horizon API.
///
/// This structure is used to construct a query to retrieve a comprehensive list of trade aggregations, which will be filtered
/// by the mandatory base asset, counter asset and resolution fields. Additional filters such as start time, end time, limit
/// and order can be supplied. It adheres to the structure and parameters required by the Horizon API for retrieving a 
/// <a href="https://developers.stellar.org/docs/data/horizon/api-reference/list-trade-aggregations">list of trade aggregations</a>.
///
/// # Usage
///
/// Create an instance of this struct and set the desired query parameters to filter the list of trade aggregations.
/// Pass this request object to the [`HorizonClient::get_trade_aggregations`](crate::horizon_client::HorizonClient::get_trade_aggregations)
/// method to fetch the corresponding data from the Horizon API.
///
/// # Example
/// ```
/// use stellar_rs::{trade_aggregations::prelude::*, models::*, Paginatable};
///
/// let request = TradeAggregationsRequest::new()
///     .set_base_asset(AssetType::Native).unwrap()
///     .set_counter_asset(AssetType::Alphanumeric4(AssetData{
///        asset_issuer: "GBZXN7PIRZGNMHGA7MUUUF4GWPY5AYPV6LY4UV2GL6VJGIQRXFDNMADI".to_string(),
///        asset_code: "XETH".to_string(),
///     })).unwrap()
///     .set_resolution(Resolution(ResolutionData::Value604800000)).unwrap()
///     .set_limit(100).unwrap() // Optional limit for response records
///     .set_order(Order::Desc); // Optional order of records
///
/// // Use with HorizonClient::get_trade_aggregations
/// ```
///
#[derive(PartialEq, Default)]
pub struct TradeAggregationsRequest<B = NoBaseAsset, C = NoCounterAsset, R = NoResolution> {
    /// The base asset of the trade aggregation.
    pub base_asset: B,
    /// The counter asset of the trade.
    pub counter_asset: C,
    // The lower time boundary represented as milliseconds since epoch. Optional.
    pub start_time: Option<i64>,
    // The upper time boundary represented as milliseconds since epoch. Optional.
    pub end_time: Option<i64>,
    // The segment duration represented as milliseconds. It must contain one of the `ResolutionData` enum types.
    pub resolution: R,
    /// Specifies the maximum number of records to be returned in a single response.
    /// The range for this parameter is from 1 to 200. The default value is set to 10.
    pub limit: Option<u8>,
    /// Determines the [`Order`] of the records in the response. Valid options are [`Order::Asc`] (ascending)
    /// and [`Order::Desc`] (descending). If not specified, it defaults to ascending.
    pub order: Option<Order>,
}

impl TradeAggregationsRequest<NoBaseAsset, NoCounterAsset, NoResolution> {
    // Constructor with default values
    pub fn new() -> Self {
        TradeAggregationsRequest {
            base_asset: NoBaseAsset,
            counter_asset: NoCounterAsset,
            resolution: NoResolution,
            start_time: None,
            end_time: None,
            limit: None,
            order: None,
        }
    }
}

impl<B, C, R> TradeAggregationsRequest<B, C, R> {
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
    ///    
    pub fn set_base_asset(
        self,
        base_asset: AssetType,
    ) -> Result<TradeAggregationsRequest<BaseAsset, C, R>, String> {
        Ok(TradeAggregationsRequest {
            base_asset: BaseAsset(base_asset),
            counter_asset: self.counter_asset,
            start_time: self.start_time,
            end_time: self.end_time,
            resolution: self.resolution,
            limit: self.limit,
            order: self.order,
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
    ///  
    pub fn set_counter_asset(
        self,
        counter_asset: AssetType,
    ) -> Result<TradeAggregationsRequest<B, CounterAsset, R>, String> {
        Ok(TradeAggregationsRequest {
            base_asset: self.base_asset,
            counter_asset: CounterAsset(counter_asset),
            start_time: self.start_time,
            end_time: self.end_time,
            resolution: self.resolution,
            limit: self.limit,
            order: self.order,
        })
    }

    /// Specifies the resolution in the request.
    ///
    /// # Arguments
    ///
    /// * `resolution` - The segment duration represented as milliseconds.
    ///
    /// # Returns
    ///
    /// The updated `TradeAggregationsRequest` with the resolution set.
    ///  
    pub fn set_resolution(
        self,
        resolution: Resolution,
    ) -> Result<TradeAggregationsRequest<B, C, Resolution>, String> {
        Ok(TradeAggregationsRequest {
            base_asset: self.base_asset,
            counter_asset: self.counter_asset,
            start_time: self.start_time,
            end_time: self.end_time,
            resolution,
            limit: self.limit,
            order: self.order,
        })
    }

    /// Specifies the start time in the request.
    ///
    /// # Arguments
    ///
    /// * `start_time` - The lower time boundary represented as milliseconds since epoch.
    ///
    pub fn set_start_time(self, start_time: Option<i64>) -> Result<Self, String> {
        Ok(Self {
            start_time,
            ..self
        })
    }

    /// Specifies the end time in the request.
    ///
    /// # Arguments
    ///
    /// * `end_time` - The upper time boundary represented as milliseconds since epoch.
    ///
    pub fn set_end_time(self, end_time: Option<i64>) -> Result<Self, String> {
        Ok(Self {
            end_time,
            ..self
        })
    }

    /// Specifies the maximum number of records to be returned.
    ///
    /// # Arguments
    ///
    /// * `limit` - The maximum number of records.
    ///
    pub fn set_limit(self, limit: u8) -> Result<Self, String> {
        // Validate limit if necessary
        if !(1..=200).contains(&limit) {
            Err("Limit must be between 1 and 200.".to_string())
        } else {
            Ok(Self { limit: Some(limit), ..self })
        }
    }

    /// Specifies the order of records in the record.
    /// Valid options are [`Order::Asc`] (ascending)
    /// and [`Order::Desc`] (descending). If not specified, it defaults to ascending.    /// # Arguments
    ///
    /// * `order` - A variant of the  [`Order`] enum.
    ///
    pub fn set_order(self, order: Order) -> Result<Self, String> {
        // No validation required for setting the order in this context
        Ok(Self { order: Some(order), ..self })
    }
}

impl Request for TradeAggregationsRequest<BaseAsset, CounterAsset, Resolution> {
    fn get_query_parameters(&self) -> String {
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
            Some(format!("resolution={}", self.resolution.0)),
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