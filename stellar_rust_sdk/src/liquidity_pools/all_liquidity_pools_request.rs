use crate::{models::{Order, Request}, BuildQueryParametersExt};

/// Represents a reserve for a liquidity pool. This struct is used to specify the asset code and
#[derive(PartialEq, Debug)]
pub struct Reserve {
    pub asset_code: String,
    pub asset_issuer: String,
}

/// Represents a reserve type for a liquidity pool. This enum is used to specify the type of reserve
/// to filter by when querying the Horizon server for liquidity pool records.
#[derive(PartialEq, Debug)]
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
        let query_reserve_parameters = self
            .reserves
            .as_ref()
            .map_or_else(Vec::new, |reserves| {
                reserves
                    .iter()
                    .enumerate()
                    .fold(Vec::new(), |mut acc, (i, reserve)| {
                        let separator = if i == 0 { "reserves=" } else { "%2C" };
                        match reserve {
                            ReserveType::Native => acc.push(format!("{}native", separator)),
                            ReserveType::Alphanumeric4(reserve)
                            | ReserveType::Alphanumeric12(reserve) => {
                                acc.push(format!(
                                    "{}{}%3A{}",
                                    separator, reserve.asset_code, reserve.asset_issuer
                                ));
                            }
                        }
                        acc
                    })
            })
            .join("");
        vec![
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
            Some(query_reserve_parameters),
        ]
        .build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}?{}",
            base_url,
            super::LIQUIDITY_POOLS_PATH,
            self.get_query_parameters()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let request = AllLiquidityPoolsRequest::new();
        assert_eq!(request.cursor, None);
        assert_eq!(request.limit, None);
        assert_eq!(request.order, None);
        assert_eq!(request.reserves, None);
    }

    #[test]
    fn test_set_cursor() {
        let request = AllLiquidityPoolsRequest::new().set_cursor(1234);
        assert_eq!(request.cursor, Some(1234));
    }

    #[test]
    fn test_set_limit() {
        let request = AllLiquidityPoolsRequest::new().set_limit(20);
        assert_eq!(request.limit, Some(20));
    }

    #[test]
    fn test_set_order() {
        let request = AllLiquidityPoolsRequest::new().set_order(Order::Desc);
        assert_eq!(request.order, Some(Order::Desc));
    }

    #[test]
    fn test_add_native_reserve() {
        let request = AllLiquidityPoolsRequest::new().add_native_reserve();
        assert_eq!(request.reserves, Some(vec![ReserveType::Native]));
    }

    #[test]
    fn test_add_native_reserve_twice() {
        let request = AllLiquidityPoolsRequest::new()
            .add_native_reserve()
            .add_native_reserve();
        assert_eq!(
            request.reserves,
            Some(vec![ReserveType::Native, ReserveType::Native])
        );
    }

    #[test]
    fn test_add_alphanumeric4_reserve() {
        let mut request = AllLiquidityPoolsRequest::new();
        request = request.add_alphanumeric4_reserve("USD".to_string(), "issuer".to_string());

        if let Some(reserves) = request.reserves {
            assert_eq!(reserves.len(), 1);
            match &reserves[0] {
                ReserveType::Alphanumeric4(reserve) => {
                    assert_eq!(reserve.asset_code, "USD");
                    assert_eq!(reserve.asset_issuer, "issuer");
                }
                _ => panic!("Reserve type is not Alphanumeric4"),
            }
        } else {
            panic!("Reserves is None");
        }
    }

    #[test]
    fn test_add_alphanumeric12_reserve() {
        let mut request = AllLiquidityPoolsRequest::new();
        request = request.add_alphanumeric12_reserve("LONGASSET".to_string(), "issuer".to_string());

        if let Some(reserves) = request.reserves {
            assert_eq!(reserves.len(), 1);
            match &reserves[0] {
                ReserveType::Alphanumeric12(reserve) => {
                    assert_eq!(reserve.asset_code, "LONGASSET");
                    assert_eq!(reserve.asset_issuer, "issuer");
                }
                _ => panic!("Reserve type is not Alphanumeric12"),
            }
        } else {
            panic!("Reserves is None");
        }
    }

    #[test]
    fn test_get_query_parameters() {
        let mut request = AllLiquidityPoolsRequest::new();
        request = request.add_alphanumeric4_reserve("USD".to_string(), "issuer".to_string());
        let query_parameters = request.get_query_parameters();

        assert_eq!(query_parameters, "?reserves=USD%3Aissuer");
    }
}
