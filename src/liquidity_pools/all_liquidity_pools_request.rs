use crate::liquidity_pools::LIQUIDITY_POOLS_PATH;
use crate::models::{Order, Request};
pub struct Reserve {
    pub asset_code: String,
    pub asset_issuer: String,
}

pub enum ReserveType {
    Native,
    Alphanumeric4(Reserve),
    Alphanumeric12(Reserve),
}

pub struct AllLiquidityPoolsRequest {
    cursor: Option<u32>,
    limit: Option<u8>,
    order: Option<Order>,
    reserves: Option<Vec<ReserveType>>,
}

impl AllLiquidityPoolsRequest {
    pub fn new() -> AllLiquidityPoolsRequest {
        AllLiquidityPoolsRequest {
            cursor: None,
            limit: None,
            order: None,
            reserves: None,
        }
    }

    pub fn set_cursor(self, cursor: u32) -> AllLiquidityPoolsRequest {
        AllLiquidityPoolsRequest {
            cursor: Some(cursor),
            ..self
        }
    }

    pub fn set_limit(self, limit: u8) -> AllLiquidityPoolsRequest {
        AllLiquidityPoolsRequest {
            limit: Some(limit),
            ..self
        }
    }

    pub fn set_order(self, order: Order) -> AllLiquidityPoolsRequest {
        AllLiquidityPoolsRequest {
            order: Some(order),
            ..self
        }
    }

    pub fn add_native_reserve(mut self) -> AllLiquidityPoolsRequest {
        match self.reserves {
            Some(ref mut reserves) => reserves.push(ReserveType::Native),
            None => self.reserves = Some(vec![ReserveType::Native]),
        }
        self
    }

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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_all_liquidity_pools_request_build_url() {
//         let cursor: u32 = 123;
//         let limit: u8 = 10;
//         let order = Order::Desc;
//         let compare_order = Order::Desc;
//         let asset_code = "USD".to_string();
//         let asset_issuer = "G....".to_string();

//         let request = AllLiquidityPoolsRequest::new()
//             .set_cursor(cursor)
//             .set_limit(limit)
//             .set_order(order)
//             .add_native_reserve()
//             .add_alphanumeric4_reserve(asset_code.clone(), asset_issuer.clone());

//         let url = request.build_url("https://horizon-testnet.stellar.org");

//         assert_eq!(
//             url,
//             format!(
//                 "https://horizon-testnet.stellar.org/{}?cursor={}&limit={}&order={}&reserves=native&reserves={}-{}",
//                 super::LIQUIDITY_POOLS_PATH,
//                 cursor,
//                 limit,
//                 compare_order,
//                 asset_code,
//                 asset_issuer
//             )
//         );
//     }
// }
