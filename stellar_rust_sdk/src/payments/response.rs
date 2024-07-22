use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use crate::models::prelude::{Embedded, ResponseLinks};
use crate::models::Response;

/// Represents the response from the Horizon server when querying for all payments.
///
/// This struct represents the response from the Horizon server when querying for all payments.
/// It includes the links to the current, next, and previous pages of the response, as well as the
/// embedded records of payments.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct PaymentsResponse {
    /// The links to the current, next, and previous pages of the response.
    #[serde(rename = "_links")]
    pub links: ResponseLinks,
    /// The embedded records of liquidity pools.
    #[serde(rename = "_embedded")]
    pub embedded: Embedded<Payment>,
}

/// Represents the payment record in the Horizon API response.
///
/// This struct encapsulates detailed information about a single payment, including its ID, paging token,
/// transaction success status, source account, type, creation date, transaction hash, starting balance,
/// funder, and account.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
pub struct Payment {
    pub id: String,
    pub paging_token: String,
    pub transaction_successful: bool,
    pub source_account: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub type_i: i64,
    pub created_at: String,
    pub transaction_hash: String,
    #[serde(rename = "starting_balance")]
    pub starting_balance: String,
    pub funder: String,
    pub account: String,
}

impl Response for PaymentsResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}