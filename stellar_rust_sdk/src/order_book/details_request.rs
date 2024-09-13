use crate::models::prelude::AssetType;
use crate::models::Request;
pub struct SellingAsset(AssetType);
pub struct NoSellingAsset;
pub struct BuyingAsset(AssetType);
pub struct NoBuyingAsset;

/// Represents the request for the details of an order book.
#[derive(PartialEq, Debug)]
pub struct DetailsRequest<S, B> {
    /// The selling asset of the order book.
    pub selling_asset: S,
    /// The buying asset of the order book.
    pub buying_asset: B,
}

/// Represents the selling asset of the order book with no buying asset or selling asset
impl DetailsRequest<NoSellingAsset, NoBuyingAsset> {
    pub fn new() -> Self {
        DetailsRequest {
            selling_asset: NoSellingAsset,
            buying_asset: NoBuyingAsset,
        }
    }

    /// Sets the selling asset of the order book.
    ///
    /// # Arguments
    /// * `selling_asset` - An [`AssetType`] enum value specifying the selling asset.
    ///
    pub fn set_selling_asset(
        self,
        selling_asset: AssetType,
    ) -> Result<DetailsRequest<SellingAsset, NoBuyingAsset>, String> {
        Ok(DetailsRequest {
            selling_asset: SellingAsset(selling_asset),
            buying_asset: NoBuyingAsset,
        })
    }

    /// Sets the buying asset of the order book.
    ///
    /// # Arguments
    /// * `buying_asset` - An [`AssetType`] enum value specifying the buying asset.
    pub fn set_buying_asset(
        self,
        buying_asset: AssetType,
    ) -> Result<DetailsRequest<NoSellingAsset, BuyingAsset>, String> {
        Ok(DetailsRequest {
            selling_asset: NoSellingAsset,
            buying_asset: BuyingAsset(buying_asset),
        })
    }
}

/// Implements the setting of a selling asset of the order book with a buying asset and no selling asset
impl DetailsRequest<NoSellingAsset, BuyingAsset> {
    /// Sets the selling asset of the order book.
    ///
    /// # Arguments
    /// * `selling_asset` - An [`AssetType`] enum value specifying the selling asset.
    pub fn set_selling_asset(
        self,
        selling_asset: AssetType,
    ) -> Result<DetailsRequest<SellingAsset, BuyingAsset>, String> {
        Ok(DetailsRequest {
            selling_asset: SellingAsset(selling_asset),
            buying_asset: self.buying_asset,
        })
    }
}

/// Implements the setting of a buying asset of the order book with a selling asset and no buying asset
impl DetailsRequest<SellingAsset, NoBuyingAsset> {
    /// Sets the buying asset of the order book.
    ///
    /// # Arguments
    /// * `buying_asset` - An [`AssetType`] enum value specifying the buying asset.
    pub fn set_buying_asset(
        self,
        buying_asset: AssetType,
    ) -> Result<DetailsRequest<SellingAsset, BuyingAsset>, String> {
        Ok(DetailsRequest {
            selling_asset: self.selling_asset,
            buying_asset: BuyingAsset(buying_asset),
        })
    }
}

impl Request for DetailsRequest<SellingAsset, BuyingAsset> {
    fn get_query_parameters(&self) -> String {
        vec![&self.selling_asset.0, &self.buying_asset.0]
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut parameters, (i, asset)| {
            let asset_type_prefix = if i == 0 {
                "selling_asset_type="
            }
            // no `&` for `base_asset_type`, as the query begins with `?`
            else {
                "&buying_asset_type="
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
                        "&selling_asset_issuer="
                    } else {
                        "&buying_asset_issuer="
                    };
                    let asset_code_prefix = if i == 0 {
                        "&selling_asset_code="
                    } else {
                        "&buying_asset_code="
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
        .join("")
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}?{}",
            base_url,
            super::ORDER_BOOK_PATH,
            self.get_query_parameters()
        )
    }
}

mod tests {

    #[test]
    fn test_details_request() {
        use super::DetailsRequest;
        use crate::models::prelude::{AssetData, AssetType};
        use crate::models::Request;
        let details_request = DetailsRequest::new()
            .set_buying_asset(AssetType::Native)
            .unwrap()
            .set_selling_asset(AssetType::Native)
            .unwrap();

        assert_eq!(
            details_request.get_query_parameters(),
            "selling_asset_type=native&buying_asset_type=native"
        );

        let details_request = DetailsRequest::new()
            .set_buying_asset(AssetType::Native)
            .unwrap()
            .set_selling_asset(AssetType::Alphanumeric4(AssetData {
                asset_code: "USDC".to_string(),
                asset_issuer: "GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5"
                    .to_string(),
            }))
            .unwrap();

        assert_eq!(
            details_request.get_query_parameters(),
            "selling_asset_type=credit_alphanum4&selling_asset_code=USDC&selling_asset_issuer=GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5&buying_asset_type=native"
        );
    }
}
