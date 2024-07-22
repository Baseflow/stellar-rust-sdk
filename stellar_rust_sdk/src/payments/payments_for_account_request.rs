use crate::Paginatable;
use stellar_rust_sdk_derive::Pagination;
use crate::models::{IncludeFailed, Order, Request};

#[derive(Default, Pagination)]
pub struct PaymentsForAccountRequest {
    /// The Stellar address of the account for which you want to retrieve payments.
    account_id: Option<String>,
    /// A pointer to a specific location in a collection of responses, derived from the
    ///  `paging_token` value of a record. Used for pagination control in the API response.
    cursor: Option<u32>,
    /// Specifies the maximum number of records to be returned in a single response.
    limit: Option<u8>,
    /// Determines the [`Order`] of the records in the response. Valid options are [`Order::Asc`] (ascending)
    /// and [`Order::Desc`] (descending). If not specified, it defaults to ascending.
    order: Option<Order>,
    /// A boolean value that determines whether failed transactions should be included in the response.
    include_failed: IncludeFailed,
}

impl PaymentsForAccountRequest {
    /// Creates a new `PaymentsForAccountRequest` with default parameters.
    pub fn new() -> PaymentsForAccountRequest {
        PaymentsForAccountRequest {
            account_id: None,
            cursor: None,
            limit: None,
            order: None,
            include_failed: IncludeFailed::False,
        }
    }

    /// Sets the Stellar address of the account for which you want to retrieve payments.
    /// 
    /// # Arguments
    /// * `account_id` - The Stellar address of the account for which you want to retrieve payments.
    /// 
    pub fn set_account_id(mut self, account_id: String) -> PaymentsForAccountRequest {
        self.account_id = Some(account_id);
        self
    }

    /// Sets a pointer to a specific location in a collection of responses, derived from the
    /// 
    /// # Arguments
    /// * `cursor` - A pointer to a specific location in a collection of responses, derived from the
    ///  `paging_token` value of a record. Used for pagination control in the API response.
    /// 
    pub fn include_failed(mut self, include_failed: IncludeFailed) -> PaymentsForAccountRequest {
        self.include_failed = include_failed;
        self
    }
}

impl Request for PaymentsForAccountRequest {
    fn get_query_parameters(&self) -> String {
        let mut params = String::new();
        if let Some(cursor) = self.cursor {
            params.push_str(&format!("cursor={}&", cursor));
        }
        if let Some(limit) = self.limit {
            params.push_str(&format!("limit={}&", limit));
        }
        if let Some(order) = &self.order {
            params.push_str(&format!("order={}&", order));
        }
        // if let Some(account_id) = &self.account_id {
        //     params.push_str(&format!("account_id={}&", account_id));
        // }
        params.push_str(&format!("include_failed={}", self.include_failed));
        params
    }

    fn build_url(&self, base_url: &str) -> String {
        let binding = "".to_string();
        let account_id = self.account_id.as_ref().unwrap_or(&binding);
        println! ("{}/accounts/{}/payments?{}", base_url, account_id, self.get_query_parameters());
        format!  ("{}/accounts/{}/payments?{}", base_url, account_id, self.get_query_parameters())
    }
}