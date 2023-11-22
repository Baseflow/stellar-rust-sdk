use crate::models::*;

use super::super::Order;

pub struct LedgersRequest {
    /// The cursor for the page
    cursor: Option<u32>,
    /// The maximum number of records to return
    limit: Option<u32>,
    /// The order of the records
    order: Option<Order>,
}

impl Request for LedgersRequest {
    fn new() -> Self {
        Self {
            cursor: None,
            limit: None,
            order: None,
        }
    }

    fn get_path(&self) -> &str {
        "/ledgers"
    }

    fn get_query_parameters(&self) -> String {
        let mut query_parameters = vec![];

        if let Some(cursor) = &self.cursor {
            query_parameters.push(format!("cursor={}", cursor));
        }

        if let Some(limit) = &self.limit {
            query_parameters.push(format!("limit={}", limit));
        }

        if let Some(order) = &self.order {
            query_parameters.push(format!("order={}", order));
        }

        query_parameters.join("&")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(cursor) = &self.cursor {
            if *cursor < 1 {
                return Err("cursor must be greater than or equal to 1".to_string());
            }
        }

        if let Some(limit) = &self.limit {
            if *limit < 1 {
                return Err("limit must be greater than or equal to 1".to_string());
            }
            if *limit > 200 {
                return Err("limit must be less than or equal to 200".to_string());
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

impl LedgersRequest {
    /// Sets the cursor
    /// # Arguments
    /// * `cursor` - The cursor
    /// # Returns
    /// The request object
    /// [AllLedgersRequest](struct.AllLedgersRequest.html)
    pub fn set_cursor(&mut self, cursor: u32) {
        self.cursor = Some(cursor);
    }
    /// Sets the limit
    /// # Arguments
    /// * `limit` - The limit
    /// # Returns
    /// The request object
    /// [AllAssetsRequest](struct.AllAssetsRequest.html)
    pub fn set_limit(&mut self, limit: u32) {
        self.limit = Some(limit);
    }

    /// Sets the order
    /// # Arguments
    /// * `order` - The order
    /// # Returns
    /// The request object
    /// [AllAssetsRequest](struct.AllAssetsRequest.html)
    pub fn set_order(&mut self, order: Order) {
        self.order = Some(order);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ledgers_request() {
        let request = LedgersRequest::new();

        assert_eq!(request.get_path(), "/ledgers");
    }
}
