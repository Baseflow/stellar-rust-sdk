use crate::models::{IncludeFailed, Order, Request};
use crate::payments::PAYMENTS_PATH;
use crate::BuildQueryParametersExt;
use stellar_rust_sdk_derive::pagination;

#[pagination]
#[derive(Default)]
pub struct PaymentsForAccountRequest {
    /// The Stellar address of the account for which you want to retrieve payments.
    account_id: Option<String>,
    /// A boolean value that determines whether failed transactions should be included in the response.
    include_failed: Option<IncludeFailed>,
}

impl PaymentsForAccountRequest {
    /// Creates a new `PaymentsForAccountRequest` with default parameters.
    pub fn new() -> PaymentsForAccountRequest {
        PaymentsForAccountRequest {
            account_id: None,
            cursor: None,
            limit: None,
            order: None,
            include_failed: Option::from(IncludeFailed::False),
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
    pub fn set_include_failed(
        mut self,
        include_failed: IncludeFailed,
    ) -> PaymentsForAccountRequest {
        self.include_failed = Option::from(include_failed);
        self
    }
}

impl Request for PaymentsForAccountRequest {
    fn get_query_parameters(&self) -> String {
        vec![
            self.include_failed.as_ref().map(|s| format!("include_failed={}", s)),
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
        ]
            .build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        let binding = "".to_string();
        let account_id = self.account_id.as_ref().unwrap_or(&binding);
        format!(
            "{}/accounts/{}/{}?{}",
            base_url,
            account_id,
            PAYMENTS_PATH,
            self.get_query_parameters()
        )
    }
}
