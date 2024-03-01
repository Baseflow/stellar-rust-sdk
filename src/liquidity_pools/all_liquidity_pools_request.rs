use crate::liquidity_pools::LIQUIDITY_POOLS_PATH;
use crate::models::{Order, Request};

/// Represents a reserve for a liquidity pool. This struct is used to specify the asset code and
pub struct Reserve {
    pub asset_code: String,
    pub asset_issuer: String,
}

/// Represents a reserve type for a liquidity pool. This enum is used to specify the type of reserve
/// to filter by when querying the Horizon server for liquidity pool records.
pub enum ReserveType {
    /// A native reserve type. It holds no Value
    Native,
    /// An alphanumeric 4 reserve type. It holds a Reserve struct with asset code and asset issuer.
    Alphanumeric4(Reserve),
    /// An alphanumeric 12 reserve type. It holds a Reserve struct with asset code and asset issuer.
    Alphanumeric12(Reserve),
}

/// Represents a request for listing all liquidity pools on the Stellar Horizon API.
///
/// `AllLiquidityPoolsRequest` is a struct used to construct queries for retrieving information about liquidity pools from the Horizon server. It includes parameters that allow for pagination control and sorting of the liquidity pool records.
/// It includes parameters that allow for pagination control and sorting of the liquidity pool records.
///
/// # Usage
/// Instances of `AllLiquidityPoolsRequest` are created and optionally configured using the builder pattern.
/// Once the desired parameters are set, the request can be passed to the Horizon client to fetch liquidity pool data.
///
/// # Example
/// ```rust
/// use stellar_rs::liquidity_pools::all_liquidity_pools_request::AllLiquidityPoolsRequest;
/// use stellar_rs::models::*;
///
/// let request = AllLiquidityPoolsRequest::new()
///     .set_cursor(1234)
///     .set_limit(20)
///     .set_order(Order::Desc)
///     .add_native_reserve()
///     .add_alphanumeric4_reserve("USD".to_string(), "GAXLYH...".to_string());
///
/// // The request can now be used with a Horizon client to fetch liquidity pools.
/// ```
///
#[derive(Default)]
pub struct AllLiquidityPoolsRequest {
    /// A pointer to a specific location in a collection of responses, derived from the
    ///   `paging_token` value of a record. Used for pagination control in the API response.
    cursor: Option<u32>,

    /// Specifies the maximum number of records to be returned in a single response.
    ///   The range for this parameter is from 1 to 200. The default value is set to 10.
    limit: Option<u8>,

    /// Determines the [`Order`] of the records in the response. Valid options are [`Order::Asc`] (ascending)
    ///   and [`Order::Desc`] (descending). If not specified, it defaults to ascending.
    order: Option<Order>,

    /// A list of reserves to filter by.
    reserves: Option<Vec<ReserveType>>,
}

impl AllLiquidityPoolsRequest {
    /// Creates a new `AllLiquidityPoolsRequest` with default parameters.
    pub fn new() -> AllLiquidityPoolsRequest {
        AllLiquidityPoolsRequest {
            cursor: None,
            limit: None,
            order: None,
            reserves: None,
        }
    }

    /// Sets the cursor for pagination.
    ///
    /// # Arguments
    /// * `cursor` - A `u32` value pointing to a specific location in a collection of responses.
    ///
    pub fn set_cursor(self, cursor: u32) -> AllLiquidityPoolsRequest {
        AllLiquidityPoolsRequest {
            cursor: Some(cursor),
            ..self
        }
    }

    /// Sets the maximum number of records to return.
    ///
    /// # Arguments
    /// * `limit` - A `u8` value specifying the maximum number of records. Range: 1 to 200. Defaults to 10.
    ///
    pub fn set_limit(self, limit: u8) -> AllLiquidityPoolsRequest {
        AllLiquidityPoolsRequest {
            limit: Some(limit),
            ..self
        }
    }

    /// Sets the order of the returned records.
    ///
    /// # Arguments
    /// * `order` - An [`Order`] enum value specifying the order (ascending or descending).
    ///
    pub fn set_order(self, order: Order) -> AllLiquidityPoolsRequest {
        AllLiquidityPoolsRequest {
            order: Some(order),
            ..self
        }
    }

    /// Adds a native reserve to the request.
    pub fn add_native_reserve(mut self) -> AllLiquidityPoolsRequest {
        match self.reserves {
            Some(ref mut reserves) => reserves.push(ReserveType::Native),
            None => self.reserves = Some(vec![ReserveType::Native]),
        }
        self
    }

    /// Adds an alphanumeric 4 reserve to the request.
    ///
    /// # Arguments
    /// * `asset_code` - A `String` value representing the asset code of the reserve.
    /// * `asset_issuer` - A `String` value representing the asset issuer of the reserve.
    ///
    pub fn add_alphanumeric4_reserve(
        mut self,
        asset_code: String,
        asset_issuer: String,
    ) -> AllLiquidityPoolsRequest {
        match self.reserves {
            Some(ref mut reserves) => reserves.push(ReserveType::Alphanumeric4(Reserve {
                asset_code,
                asset_issuer,
            })),
            None => {
                self.reserves = Some(vec![ReserveType::Alphanumeric4(Reserve {
                    asset_code,
                    asset_issuer,
                })])
            }
        }
        self
    }

    /// Adds an alphanumeric 12 reserve to the request.
    ///
    /// # Arguments
    /// * `asset_code` - A `String` value representing the asset code of the reserve.
    /// * `asset_issuer` - A `String` value representing the asset issuer of the reserve.
    ///
    pub fn add_alphanumeric12_reserve(
        mut self,
        asset_code: String,
        asset_issuer: String,
    ) -> AllLiquidityPoolsRequest {
        match self.reserves {
            Some(ref mut reserves) => reserves.push(ReserveType::Alphanumeric12(Reserve {
                asset_code,
                asset_issuer,
            })),
            None => {
                self.reserves = Some(vec![ReserveType::Alphanumeric12(Reserve {
                    asset_code,
                    asset_issuer,
                })])
            }
        }
        self
    }
}

impl Request for AllLiquidityPoolsRequest {
    fn get_query_parameters(&self) -> String {
        let mut query_parameters = Vec::new();
        let mut query_reserve_parameters: Vec<String> = Vec::new();

        if let Some(reserves) = &self.reserves {
            for (i, reserve) in reserves.iter().enumerate() {
                if i == 0 {
                    match reserve {
                        ReserveType::Native => {
                            query_reserve_parameters.push("reserves=native".to_string())
                        }
                        ReserveType::Alphanumeric4(reserve) => {
                            query_reserve_parameters.push(format!(
                                "reserves={}%3A{}",
                                reserve.asset_code, reserve.asset_issuer
                            ));
                        }
                        ReserveType::Alphanumeric12(reserve) => {
                            query_reserve_parameters.push(format!(
                                "reserves={}%3A{}",
                                reserve.asset_code, reserve.asset_issuer
                            ));
                        }
                    }
                } else {
                    match reserve {
                        ReserveType::Native => {
                            query_reserve_parameters.push("%2Cnative".to_string())
                        }
                        ReserveType::Alphanumeric4(reserve) => {
                            query_reserve_parameters.push(format!(
                                "%2C{}%3A{}",
                                reserve.asset_code, reserve.asset_issuer
                            ));
                        }
                        ReserveType::Alphanumeric12(reserve) => {
                            query_reserve_parameters.push(format!(
                                "%2C{}%3A{}",
                                reserve.asset_code, reserve.asset_issuer
                            ));
                        }
                    }
                }
            }
        }

        if let Some(cursor) = self.cursor {
            query_parameters.push(format!("cursor={}", cursor));
        }

        if let Some(limit) = self.limit {
            query_parameters.push(format!("limit={}", limit));
        }

        if let Some(order) = &self.order {
            query_parameters.push(format!("order={}", order));
        }

        query_parameters.push(query_reserve_parameters.join(""));

        query_parameters.join("&")
    }

    fn build_url(&self, base_url: &str) -> String {
        println!(
            "{}/{}?{}",
            base_url,
            super::LIQUIDITY_POOLS_PATH,
            self.get_query_parameters()
        );

        format!(
            "{}/{}?{}",
            base_url,
            super::LIQUIDITY_POOLS_PATH,
            self.get_query_parameters()
        )
    }
}
