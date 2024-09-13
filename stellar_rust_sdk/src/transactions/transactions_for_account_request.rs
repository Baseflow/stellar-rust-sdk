use crate::{models::*, BuildQueryParametersExt};
use stellar_rust_sdk_derive::pagination;

/// Represents the ID of an account for which the transactions are to be retrieved.
#[derive(Default, Clone)]
pub struct TransactionsAccountId(String);

/// Represents the absence of an ID of an account for which the transactions are to be retrieved.
#[derive(Default, Clone)]
pub struct NoTransactionsAccountId;

#[pagination]
#[derive(Default)]
pub struct TransactionsForAccountRequest<I> {
    /// The ID of the account for which the transactions are to be retrieved.
    account_id: I,
    // Indicates whether or not to include failed operations in the response.
    include_failed: Option<bool>,
}

impl TransactionsForAccountRequest<NoTransactionsAccountId> {
    /// Creates a new `TransactionsForAccountRequest` with default parameters.
    pub fn new() -> Self {
        TransactionsForAccountRequest::default()
    }

    /// Sets the account ID for the request.
    ///
    /// # Arguments
    /// * `account_id` - The account ID for which the transactions are to be retrieved.
    ///
    /// # Returns
    /// A `TransactionsForAccountRequest` with the specified account ID, or an error if the account ID is invalid.
    ///
    pub fn set_account_id(
        self,
        account_id: impl Into<String>,
    ) -> Result<TransactionsForAccountRequest<TransactionsAccountId>, String> {
        Ok(TransactionsForAccountRequest {
            account_id: TransactionsAccountId(account_id.into()),
            include_failed: self.include_failed,
            cursor: self.cursor,
            limit: self.limit,
            order: self.order,
        })
    }
}

impl TransactionsForAccountRequest<TransactionsAccountId> {
    /// Sets the `include_failed` field for the request. Can only be set on a request that
    /// has a set account id.
    ///
    /// # Arguments
    /// * `include_failed` - A `bool` to indicate whether or not to include failed operations.
    ///
    /// # Returns
    /// A `TransactionsForAccountRequest` with the updated `include_failed` field.
    ///
    pub fn set_include_failed(
        self,
        include_failed: bool,
    ) -> Result<TransactionsForAccountRequest<TransactionsAccountId>, String> {
        Ok(TransactionsForAccountRequest {
            account_id: self.account_id,
            include_failed: Some(include_failed),
            cursor: self.cursor,
            limit: self.limit,
            order: self.order,
        })
    }
}

impl Request for TransactionsForAccountRequest<TransactionsAccountId> {
    fn get_query_parameters(&self) -> String {
        vec![
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
            self.include_failed
                .as_ref()
                .map(|i| format!("include_failed={}", i)),
        ]
        .build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        // This URL comprises paths and query parameters.
        // Additionally, this request uses the API endpoint for `accounts`.
        let account_id = &self.account_id.0;
        use crate::accounts::ACCOUNTS_PATH;
        format!(
            "{}/{}/{}/{}{}",
            base_url,
            ACCOUNTS_PATH,
            account_id,
            super::TRANSACTIONS_PATH,
            self.get_query_parameters(),
        )
    }
}
