use crate::models::Request;

use super::super::Order;

// AllAssetsRequest is the request for the /assets endpoint
// [More Details] https://www.stellar.org/developers/horizon/reference/endpoints/assets-all.html "Assets"
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
    /// Creates a new request object
    /// # Returns
    /// A new request object
    /// [AllAssetsRequest](struct.AllAssetsRequest.html)
    fn new() -> Self {
        AllAssetsRequest {
            asset_code: None,
            asset_issuer: None,
            cursor: None,
            limit: None,
            order: None,
        }
    }

    /// Gets the relative URL for the request
    fn get_path(&self) -> &str {
        "/assets"
    }

    /// Gets the query parameters for the request
    fn get_query_parameters(&self) -> String {
        let mut query = String::new();
        if let Some(asset_code) = &self.asset_code {
            query.push_str(&format!("asset_code={}&", asset_code));
        }
        if let Some(asset_issuer) = &self.asset_issuer {
            query.push_str(&format!("asset_issuer={}&", asset_issuer));
        }
        if let Some(cursor) = &self.cursor {
            query.push_str(&format!("cursor={}&", cursor));
        }
        if let Some(limit) = &self.limit {
            query.push_str(&format!("limit={}&", limit));
        }
        if let Some(order) = &self.order {
            query.push_str(&format!("order={}&", order));
        }

        query.trim_end_matches('&').to_string()
    }

    /// Validates the request
    fn validate(&self) -> Result<(), String> {
        if let Some(asset_code) = &self.asset_code {
            // TODO: implement full asset code regex
            if asset_code.len() > 12 {
                return Err("asset_code must be 12 characters or less".to_string());
            }
        }

        if let Some(asset_issuer) = &self.asset_issuer {
            // TODO: implement full asset issuer regex
            if asset_issuer.len() != 56 {
                return Err("asset_issuer must be 56 characters".to_string());
            }
        }

        if let Some(limit) = &self.limit {
            if *limit < 1 || *limit > 200 {
                return Err("limit must be between 1 and 200".to_string());
            }
        }

        if let Some(cursor) = &self.cursor {
            if *cursor < 1 {
                return Err("cursor must be greater than or equal to 1".to_string());
            }
        }

        Ok(())
    }

    /// Builds the URL for the request
    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}{}?{}",
            base_url,
            self.get_path(),
            self.get_query_parameters()
        )
    }
}

impl AllAssetsRequest {
    /// Sets the asset code
    /// # Arguments
    /// * `asset_code` - The asset code
    /// # Returns
    /// The request object
    /// [AllAssetsRequest](struct.AllAssetsRequest.html)
    pub fn set_asset_code(&mut self, asset_code: &str) -> &mut Self {
        self.asset_code = Some(asset_code.to_owned());
        self
    }

    /// Sets the asset issuer
    /// # Arguments
    /// * `asset_issuer` - The asset issuer
    /// # Returns
    /// The request object
    /// [AllAssetsRequest](struct.AllAssetsRequest.html)
    pub fn set_asset_issuer(&mut self, asset_issuer: &str) -> &mut Self {
        self.asset_issuer = Some(asset_issuer.to_owned());
        self
    }

    /// Sets the cursor
    /// # Arguments
    /// * `cursor` - The cursor
    /// # Returns
    /// The request object
    /// [AllAssetsRequest](struct.AllAssetsRequest.html)
    pub fn set_cursor(&mut self, cursor: u32) -> &mut Self {
        self.cursor = Some(cursor);
        self
    }

    /// Sets the limit
    /// # Arguments
    /// * `limit` - The limit
    /// # Returns
    /// The request object
    /// [AllAssetsRequest](struct.AllAssetsRequest.html)
    pub fn set_limit(&mut self, limit: u32) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the order
    /// # Arguments
    /// * `order` - The order
    /// # Returns
    /// The request object
    /// [AllAssetsRequest](struct.AllAssetsRequest.html)
    pub fn set_order(&mut self, order: Order) -> &mut Self {
        self.order = Some(order);
        self
    }
}
