use crate::models::Request;

use super::super::Order;

// AllAssetsRequest is the request for the /assets endpoint
// [More Details] https://www.stellar.org/developers/horizon/reference/endpoints/assets-all.html "Assets"
pub struct AllAssetsRequest {
    asset_code: Option<String>,
    asset_issuer: Option<String>,
    cursor: Option<u32>,
    limit: Option<u32>,
    order: Option<Order>,
}

impl Request for AllAssetsRequest {
    fn new() -> Self {
        AllAssetsRequest {
            asset_code: None,
            asset_issuer: None,
            cursor: None,
            limit: None,
            order: None,
        }
    }

    fn get_path(&self) -> &str {
        "/assets"
    }

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
    pub fn set_asset_code(&mut self, asset_code: String) {
        self.asset_code = Some(asset_code);
    }

    pub fn set_asset_issuer(&mut self, asset_issuer: String) {
        self.asset_issuer = Some(asset_issuer);
    }

    pub fn set_cursor(&mut self, cursor: u32) {
        self.cursor = Some(cursor);
    }

    pub fn set_limit(&mut self, limit: u32) {
        self.limit = Some(limit);
    }

    pub fn set_order(&mut self, order: Order) {
        self.order = Some(order);
    }
}
