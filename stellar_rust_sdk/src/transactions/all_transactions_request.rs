use crate::{models::*, BuildQueryParametersExt};
use stellar_rust_sdk_derive::pagination;

/// Represents a request to list all transactions from the Stellar Horizon API.
///
/// This structure is used to construct a query to retrieve a comprehensive list of transactions, which
/// can be filtered by setting `include_failed`. It adheres to the structure and parameters required
/// by the Horizon API for retrieving a
/// <a href="https://developers.stellar.org/network/horizon/api-reference/resources/list-all-transactions">list of all transactions</a>.
///
/// # Usage
///
/// Create an instance of this struct and set the desired query parameters to filter the list of transactions.
/// Pass this request object to the [`HorizonClient::get_all_transactions`](crate::horizon_client::HorizonClient::get_all_transactions)
/// method to fetch the corresponding data from the Horizon API.
///
/// # Example
/// ```
/// use stellar_rs::transactions::all_transactions_request::AllTransactionsRequest;
/// use stellar_rs::models::{Order, IncludeFailed};
///
/// let request = AllTransactionsRequest::new()
///     .set_include_failed(IncludeFailed::True).unwrap() // Optional flag to include failed transactions
///     .set_cursor(123).unwrap() // Optional cursor for pagination
///     .set_limit(100).unwrap() // Optional limit for response records
///     .set_order(Order::Desc); // Optional order of records
///
/// // Use with HorizonClient::get_all_transactions
/// ```
///
#[pagination]
#[derive(Default)]
pub struct AllTransactionsRequest {
    // Indicates whether or not to include failed operations in the response.
    include_failed: Option<IncludeFailed>,
}

impl Request for AllTransactionsRequest {
    fn get_query_parameters(&self) -> String {
        vec![
            self.include_failed
                .as_ref()
                .map(|i| format!("include_failed={}", i)),
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
        ]
        .build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}{}",
            base_url,
            super::TRANSACTIONS_PATH,
            self.get_query_parameters()
        )
    }
}

impl AllTransactionsRequest {
    /// Creates a new `AllTransactionsRequest` with default parameters.
    pub fn new() -> Self {
        AllTransactionsRequest::default()
    }

    /// Specifies whether to include failed operations in the response.
    ///
    /// # Arguments
    /// * `include_failed` (bool) - when set to `true`, failed operations will be included.
    pub fn set_include_failed(
        self,
        include_failed: IncludeFailed,
    ) -> Result<AllTransactionsRequest, String> {
        Ok(AllTransactionsRequest {
            include_failed: Some(include_failed),
            ..self
        })
    }
}
