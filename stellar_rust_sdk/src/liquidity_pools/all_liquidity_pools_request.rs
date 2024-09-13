use crate::{
    models::{
        prelude::{AssetData, AssetType},
        Order, Request,
    },
    BuildQueryParametersExt,
};
use stellar_rust_sdk_derive::pagination;

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
/// # use stellar_rs::liquidity_pools::all_liquidity_pools_request::AllLiquidityPoolsRequest;
/// # use stellar_rs::models::*;
///
/// let request = AllLiquidityPoolsRequest::new()
///     .set_cursor(1234).unwrap()
///     .set_limit(20).unwrap()
///     .set_order(Order::Desc).unwrap()
///     .add_native_reserve()
///     .add_alphanumeric4_reserve("USD", "GAXLYH...");
///
/// // The request can now be used with a Horizon client to fetch liquidity pools.
/// ```
///
#[pagination]
#[derive(Default)]
pub struct AllLiquidityPoolsRequest {
    /// A list of reserves to filter by.
    reserves: Option<Vec<AssetType>>,
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

    /// Adds a native reserve to the request.
    pub fn add_native_reserve(mut self) -> AllLiquidityPoolsRequest {
        match self.reserves {
            Some(ref mut reserves) => reserves.push(AssetType::Native),
            None => self.reserves = Some(vec![AssetType::Native]),
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
        asset_code: impl Into<String>,
        asset_issuer: impl Into<String>,
    ) -> AllLiquidityPoolsRequest {
        let asset_code = asset_code.into();
        let asset_issuer = asset_issuer.into();
        match self.reserves {
            Some(ref mut reserves) => reserves.push(AssetType::Alphanumeric4(AssetData {
                asset_code,
                asset_issuer,
            })),
            None => {
                self.reserves = Some(vec![AssetType::Alphanumeric4(AssetData {
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
        asset_code: impl Into<String>,
        asset_issuer: impl Into<String>,
    ) -> AllLiquidityPoolsRequest {
        let asset_code = asset_code.into();
        let asset_issuer = asset_issuer.into();
        match self.reserves {
            Some(ref mut reserves) => reserves.push(AssetType::Alphanumeric12(AssetData {
                asset_code,
                asset_issuer,
            })),
            None => {
                self.reserves = Some(vec![AssetType::Alphanumeric12(AssetData {
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
                            AssetType::Native => acc.push(format!("{}native", separator)),
                            AssetType::Alphanumeric4(reserve)
                            | AssetType::Alphanumeric12(reserve) => {
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
        let request = AllLiquidityPoolsRequest::new().set_cursor(1234).unwrap();
        assert_eq!(request.cursor, Some(1234));
    }

    #[test]
    fn test_set_limit() {
        let request = AllLiquidityPoolsRequest::new().set_limit(20).unwrap();
        assert_eq!(request.limit, Some(20));
    }

    #[test]
    fn test_set_order() {
        let request = AllLiquidityPoolsRequest::new()
            .set_order(Order::Desc)
            .unwrap();
        assert_eq!(request.order, Some(Order::Desc));
    }

    #[test]
    fn test_add_native_reserve() {
        let request = AllLiquidityPoolsRequest::new().add_native_reserve();
        assert_eq!(request.reserves, Some(vec![AssetType::Native]));
    }

    #[test]
    fn test_add_native_reserve_twice() {
        let request = AllLiquidityPoolsRequest::new()
            .add_native_reserve()
            .add_native_reserve();
        assert_eq!(
            request.reserves,
            Some(vec![AssetType::Native, AssetType::Native])
        );
    }

    #[test]
    fn test_add_alphanumeric4_reserve() {
        let mut request = AllLiquidityPoolsRequest::new();
        request = request.add_alphanumeric4_reserve("USD", "issuer");

        if let Some(reserves) = request.reserves {
            assert_eq!(reserves.len(), 1);
            match &reserves[0] {
                AssetType::Alphanumeric4(reserve) => {
                    assert_eq!(reserve.asset_code, "USD");
                    assert_eq!(reserve.asset_issuer, "issuer");
                }
                _ => panic!("AssetData type is not Alphanumeric4"),
            }
        } else {
            panic!("Reserves is None");
        }
    }

    #[test]
    fn test_add_alphanumeric12_reserve() {
        let mut request = AllLiquidityPoolsRequest::new();
        request = request.add_alphanumeric12_reserve("LONGASSET", "issuer");

        if let Some(reserves) = request.reserves {
            assert_eq!(reserves.len(), 1);
            match &reserves[0] {
                AssetType::Alphanumeric12(reserve) => {
                    assert_eq!(reserve.asset_code, "LONGASSET");
                    assert_eq!(reserve.asset_issuer, "issuer");
                }
                _ => panic!("AssetData type is not Alphanumeric12"),
            }
        } else {
            panic!("Reserves is None");
        }
    }

    #[test]
    fn test_get_query_parameters() {
        let mut request = AllLiquidityPoolsRequest::new();
        request = request.add_alphanumeric4_reserve("USD", "issuer");
        let query_parameters = request.get_query_parameters();

        assert_eq!(query_parameters, "?reserves=USD%3Aissuer");
    }
}
