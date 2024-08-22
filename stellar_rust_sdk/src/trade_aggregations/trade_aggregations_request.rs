use crate::{models::*, BuildQueryParametersExt};

/// Represents the base asset. Contains an enum of one of the possible asset types.
#[derive(Clone, PartialEq, Debug)]
pub struct BaseAsset(AssetType);

/// Represents the absence of a base asset.
#[derive(PartialEq, Debug)]
pub struct NoBaseAsset;

/// Represents the counter asset. Contains an enum of one of the possible asset types.
#[derive(Clone, PartialEq, Debug)]
pub struct CounterAsset(AssetType);

/// Represents the absence of a counter asset.
#[derive(PartialEq, Debug)]
pub struct NoCounterAsset;

/// Contains the details of a non-native asset.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct AssetData {
    pub asset_code: String,
    pub asset_issuer: String,
}

/// Represents the asset type of an asset.
#[derive(Clone, PartialEq, Debug)]
pub enum AssetType {
    /// A native asset_type type. It holds no value.
    // #[default]
    Native,
    /// An alphanumeric 4 asset_type type. It holds an Asset struct with asset code and asset issuer.
    Alphanumeric4(AssetData),
    /// An alphanumeric 12 asset_type type. It holds an Asset struct with asset code and asset issuer.
    Alphanumeric12(AssetData),
}

/// Represents the absense of a resolution value.
#[derive(Default, Clone)]
pub struct NoResolution;

/// Represents the resolution value. It can contain a [`ResolutionData`] enum type.
#[derive(PartialEq, Debug, Default, Clone)]
pub struct Resolution(pub ResolutionData);

/// Represents the supported segment duration times in milliseconds.
#[derive(PartialEq, Debug, Default, Clone)]
pub enum ResolutionData {
    #[default]
    Duration60000,
    Duration300000,
    Duration900000,
    Duration3600000,
    Duration604800000,
}

impl std::fmt::Display for ResolutionData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ResolutionData::Duration60000 => write!(f, "60000"), // 1 minute
            ResolutionData::Duration300000 => write!(f, "300000"), // 5 minutes
            ResolutionData::Duration900000 => write!(f, "900000"), // 15 minutes
            ResolutionData::Duration3600000 => write!(f, "3600000"), // 1 day
            ResolutionData::Duration604800000 => write!(f, "604800000"), // 1 week
        }
    }
}

/// Represents a request to list trade aggregations from the Stellar Horizon API.
///
/// This structure is used to construct a query to retrieve a comprehensive list of trade aggregations, which will be filtered
/// by the mandatory base asset, counter asset and resolution fields. Additional filters such as start time, end time, limit
/// and order can be set. It adheres to the structure and parameters required by the Horizon API for retrieving a
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
/// use stellar_rs::{trade_aggregations::prelude::*, models::*};
///
/// let request = TradeAggregationsRequest::new()
///     .set_base_asset(AssetType::Native).unwrap()
///     .set_counter_asset(AssetType::Alphanumeric4(AssetData{
///        asset_issuer: "GBZXN7PIRZGNMHGA7MUUUF4GWPY5AYPV6LY4UV2GL6VJGIQRXFDNMADI".to_string(),
///        asset_code: "XETH".to_string(),
///     })).unwrap()
///     .set_resolution(Resolution(ResolutionData::Duration604800000)).unwrap()
///     .set_limit(100).unwrap() // Optional limit for response records
///     .set_order(Order::Desc); // Optional order of records
///
/// // Use with HorizonClient::get_trade_aggregations
/// ```
///
#[derive(Clone, Debug, PartialEq, Default)]
pub struct TradeAggregationsRequest<B = NoBaseAsset, C = NoCounterAsset, R = NoResolution> {
    /// The base asset of the trade aggregation.
    pub base_asset: B,
    /// The counter asset of the trade.
    pub counter_asset: C,
    /// The lower time boundary represented as milliseconds since epoch. Optional.
    pub start_time: Option<i64>,
    /// The upper time boundary represented as milliseconds since epoch. Optional.
    pub end_time: Option<i64>,
    /// The segment duration represented as milliseconds. It must contain one of the `ResolutionData` enum types.
    pub resolution: R,
    /// Sgments can be offset using this parameter. Expressed in milliseconds. Optional.
    pub offset: Option<String>,
    /// Specifies the maximum number of records to be returned in a single response.
    /// The range for this parameter is from 1 to 200. The default value is set to 10.
    pub limit: Option<u8>,
    /// Determines the [`Order`] of the records in the response. Valid options are [`Order::Asc`] (ascending)
    /// and [`Order::Desc`] (descending). If not specified, it defaults to ascending.
    pub order: Option<Order>,
}

impl TradeAggregationsRequest<NoBaseAsset, NoCounterAsset, NoResolution> {
    /// Constructor with default values.
    pub fn new() -> Self {
        TradeAggregationsRequest {
            base_asset: NoBaseAsset,
            counter_asset: NoCounterAsset,
            resolution: NoResolution,
            start_time: None,
            end_time: None,
            offset: None,
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
            offset: self.offset,
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
            offset: self.offset,
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
            offset: self.offset,
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
        Ok(Self { start_time, ..self })
    }

    /// Specifies the end time in the request.
    ///
    /// # Arguments
    ///
    /// * `end_time` - The upper time boundary represented as milliseconds since epoch.
    ///
    pub fn set_end_time(self, end_time: Option<i64>) -> Result<Self, String> {
        Ok(Self { end_time, ..self })
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
            Ok(Self {
                limit: Some(limit),
                ..self
            })
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
        Ok(Self {
            order: Some(order),
            ..self
        })
    }
}

impl<B, C> TradeAggregationsRequest<B, C, Resolution> {
    /// Sets the `offset` field in the request.
    ///
    /// Can only be used if the resolution is greater than 1 hour. Offset value must be in whole hours,
    /// smaller than the provided resolution, and smaller than 24 hours. These conditions are first
    /// checked before setting the offset field of the struct. Can only be set if the `resolution`
    /// field has been set.
    ///
    /// # Arguments
    ///
    /// * `offset` - The offset represented as milliseconds. Note: although the `offset` field in the
    ///     [`TradeAggregationsRequest`] struct is of the type `String`, the `offset` argument is
    ///     of the type `u64` as a part of the condition check.
    ///
    /// # Returns
    ///
    /// A `Result` containing either the updated `TradeAggregationsRequest` or an error.
    ///
    pub fn set_offset(self, offset: u64) -> Result<Self, String> {
        const ONE_HOUR: &u64 = &360000;
        const ONE_DAY: &u64 = &86400000;
        let resolution = format!("{}", &self.resolution.0).parse::<u64>().unwrap();

        let conditions = [
            (
                &resolution < ONE_HOUR,
                "Resolution must be greater than 1 hour when setting offset.",
            ),
            (&offset % ONE_HOUR != 0, "Offset must be in whole hours."),
            (
                &offset > &resolution,
                "Offset must be smaller than the resolution.",
            ),
            (&offset > ONE_DAY, "Offset must be smaller than 24 hours."),
        ];

        for (condition, message) in conditions {
            if condition {
                return Err(message.to_string());
            }
        }

        Ok(Self {
            offset: Some(offset.to_string()),
            ..self
        })
    }
}

impl Request for TradeAggregationsRequest<BaseAsset, CounterAsset, Resolution> {
    fn get_query_parameters(&self) -> String {
        let asset_parameters = vec![&self.base_asset.0, &self.counter_asset.0]
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut parameters, (i, asset)| {
                let asset_type_prefix = if i == 0 {
                    "base_asset_type="
                }
                // no `&` for `base_asset_type`, as the query begins with `?`
                else {
                    "&counter_asset_type="
                };
                match asset {
                    AssetType::Native => parameters.push(format!("{}native", asset_type_prefix)),
                    AssetType::Alphanumeric4(asset_data)
                    | AssetType::Alphanumeric12(asset_data) => {
                        let asset_type = match asset {
                            AssetType::Alphanumeric4(_) => "credit_alphanum4",
                            AssetType::Alphanumeric12(_) => "credit_alphanum12",
                            _ => "", // should not be reached
                        };
                        let asset_issuer_prefix = if i == 0 {
                            "&base_asset_issuer="
                        } else {
                            "&counter_asset_issuer="
                        };
                        let asset_code_prefix = if i == 0 {
                            "&base_asset_code="
                        } else {
                            "&counter_asset_code="
                        };
                        parameters.push(format!(
                            "{}{}{}{}{}{}",
                            asset_type_prefix,
                            asset_type,
                            asset_code_prefix,
                            asset_data.asset_code,
                            asset_issuer_prefix,
                            asset_data.asset_issuer
                        ));
                    }
                }
                parameters
            })
            .join("");

        vec![
            Some(asset_parameters),
            Some(format!("resolution={}", self.resolution.0)),
            self.start_time
                .as_ref()
                .map(|s| format!("start_time={}", s)),
            self.end_time.as_ref().map(|e| format!("end_time={}", e)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
        ]
        .build_query_parameters()
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
