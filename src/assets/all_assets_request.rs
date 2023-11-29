use crate::{models::Request, BuildQueryParametersExt, Order};

// AllAssetsRequest is the request for the /assets endpoint
// [More Details] https://developers.stellar.org/api/horizon/resources/list-all-assets "Assets"
#[derive(Default)]
pub struct AllAssetsRequest {
    /// The assets identifying code. For example, if the asset is a credit issued on the Stellar network,
    /// the code will be the asset’s code. If the asset is a native asset, the code will be XLM.
    asset_code: Option<String>,
    /// The account ID of the asset’s issuer. For example, if the asset is a credit issued on the Stellar
    /// network, the issuer will be the account ID of the credit’s issuer.
    asset_issuer: Option<String>,
    /// The paging token of the next page of results. If this value is not provided, the results will
    /// begin at the first page.
    cursor: Option<u32>,
    /// The maximum number of records returned. The limit can range from 1 to 200 - an upper limit that
    /// is hardcoded in Horizon for performance reasons. If this argument isn’t designated, it defaults
    /// to 10.
    limit: Option<u32>,
    /// A designation of the order in which records should appear. Options include asc (ascending) or
    /// desc (descending). If this argument isn’t set, it defaults to asc.
    order: Option<Order>,
}

impl Request for AllAssetsRequest {
    fn get_query_parameters(&self) -> String {
        vec![
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
            self.asset_code.as_ref().map(|ac| format!("asset_code={}", ac)),
            self.asset_issuer.as_ref().map(|ac| format!("asset_issuer={}", ac))
        ].build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}{}",
            base_url,
            super::ASSET_PATH,
            self.get_query_parameters()
        )
    }
}

impl AllAssetsRequest {
    pub fn new() -> AllAssetsRequest {
        AllAssetsRequest::default()
    }

    /// Sets the asset code
    /// # Arguments
    /// * `asset_code` - The asset code
    /// # Returns
    /// The request object
    /// [AllAssetsRequest](struct.AllAssetsRequest.html)
    pub fn set_asset_code(self, asset_code: &str) -> Result<AllAssetsRequest, String> {
        if asset_code.len() > 12 {
            return Err("asset_code must be 12 characters or less".to_string());
        }

        Ok(AllAssetsRequest {
            asset_code: Some(asset_code.to_string()),
            ..self
        })
    }

    /// Sets the asset issuer
    /// # Arguments
    /// * `asset_issuer` - The asset issuer
    /// # Returns
    /// The request object
    /// [AllAssetsRequest](struct.AllAssetsRequest.html)
    pub fn set_asset_issuer(self, asset_issuer: &str) -> Result<AllAssetsRequest, String> {
        if asset_issuer.len() != 56 {
            return Err("asset_issuer must be 56 characters".to_string());
        }

        Ok(AllAssetsRequest {
            asset_issuer: Some(asset_issuer.to_string()),
            ..self
        })
    }

    /// Sets the cursor
    /// # Arguments
    /// * `cursor` - The cursor
    /// # Returns
    /// The request object
    /// [AllAssetsRequest](struct.AllAssetsRequest.html)
    pub fn set_cursor(self, cursor: u32) -> Result<AllAssetsRequest, String> {
        if cursor < 1 {
            return Err("cursor must be greater than or equal to 1".to_string());
        }

        Ok(AllAssetsRequest {
            cursor: Some(cursor),
            ..self
        })
    }

    /// Sets the limit
    /// # Arguments
    /// * `limit` - The limit
    /// # Returns
    /// The request object
    /// [AllAssetsRequest](struct.AllAssetsRequest.html)
    pub fn set_limit(self, limit: u32) -> Result<AllAssetsRequest, String> {
        if limit < 1 || limit > 200 {
            return Err("limit must be between 1 and 200".to_string());
        }

        Ok(AllAssetsRequest {
            limit: Some(limit),
            ..self
        })
    }

    /// Sets the order
    /// # Arguments
    /// * `order` - The order
    /// # Returns
    /// The request object
    /// [AllAssetsRequest](struct.AllAssetsRequest.html)
    pub fn set_order(self, order: Order) -> AllAssetsRequest {
        AllAssetsRequest {
            order: Some(order),
            ..self
        }
    }
}
