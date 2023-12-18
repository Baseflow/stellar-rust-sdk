use chrono::DateTime;
use chrono::Utc;
use derive_getters::Getters;
use serde::Deserialize;
use serde::Serialize;

use crate::models::Response;

/// Represents the complete response to a query for all claimable balances from the Horizon API.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct AllClaimableBalancesResponse {
    /// Navigational links related to the response pages.
    #[serde(rename = "_links")]
    links: AllClaimableBalancesLinks,
    /// Contains the actual list of claimable balance records.
    #[serde(rename = "_embedded")]
    embedded: Embedded,
}

/// Contains the navigational links for the response.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct AllClaimableBalancesLinks {
    /// The link to the current response page.
    #[serde(rename = "self")]
    self_field: AllClaimableAssetsReponseSelfField,
    /// The link to the next page of the response.
    next: Next,
    /// The link to the previous page of the response.
    prev: Prev,
}


/// Represents a navigational link in the response.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct AllClaimableAssetsReponseSelfField {
    /// The URL for the current response page.
    href: String,
}


/// Represents a navigational link to the next response page.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Next {
    /// The URL for the next response page.
    href: String,
}

/// Represents a navigational link to the previous response page.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Prev {
    /// The URL for the previous response page.
    href: String,
}


/// Encapsulates the embedded data in the response.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Embedded {
    /// A vector of records, each representing a claimable balance.
    records: Vec<Record>,
}

/// Represents a single claimable balance record within the response.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    /// Navigational links related to the claimable balance.
    #[serde(rename = "_links")]
    links: RecordsLinks,
    /// The unique identifier for the claimable balance.
    id: String,
    /// The asset associated with the claimable balance.
    asset: String,
    /// The total amount of the claimable balance.
    amount: String,
    /// The account that sponsored the creation of the claimable balance.
    sponsor: String,
    /// The ledger number in which the claimable balance was last modified.
    #[serde(rename = "last_modified_ledger")]
    last_modified_ledger: i64,
    /// The time at which the claimable balance was last modified.
    #[serde(rename = "last_modified_time")]
    last_modified_time: String,
    /// A list of claimants eligible to claim the balance.
    claimants: Vec<AllClaimableAssetsResponseClaimant>,
    /// Flags indicating the properties of the claimable balance.
    flags: AllClaimableResponseFlags,
    /// A token used for paging through results.
    #[serde(rename = "paging_token")]
    paging_token: String,
}

/// Provides the links associated with a single claimable balance record.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct RecordsLinks {
    /// The link to the claimable balance record itself.
    #[serde(rename = "self")]
    self_field: RecordsSelfField,
    /// Link to the transactions associated with the claimable balance.
    transactions: AllClaimableAssetsResponseTransactions,
    /// Link to the operations associated with the claimable balance.
    operations: AllClaimableAssetsResponseOperations,
}

/// Represents the self-link for a single claimable balance record.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct RecordsSelfField {
    /// The URL to access the specific claimable balance record.
    href: String,
}

/// Contains the link to transactions related to a specific claimable balance.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct AllClaimableAssetsResponseTransactions {
    /// The URL to the transactions associated with the claimable balance.
    /// May include query parameters for pagination.
    href: String,
    /// Indicates whether the URL can be templated with additional query parameters.
    templated: bool,
}

/// Contains the link to operations related to a specific claimable balance.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct AllClaimableAssetsResponseOperations {
    /// The URL to the operations associated with the claimable balance.
    /// May include query parameters for pagination.
    href: String,
    /// Indicates whether the URL can be templated with additional query parameters.
    templated: bool,
}

/// Represents a claimant within a claimable balance.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct AllClaimableAssetsResponseClaimant {
    /// The account ID of the claimant.
    destination: String,
    /// The claim predicate, defining the conditions under which the claimant is allowed to claim the balance.
    predicate: AllClaimableAssetsResponsePredicate,
}


/// Defines the claim predicate structure, including conditional and unconditional claims.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct AllClaimableAssetsResponsePredicate {
    /// Indicates if the claim is unconditional, meaning no restrictions are placed on when it can be claimed.
    unconditional: Option<bool>,
    /// Contains a list of 'or' predicates providing alternative conditions for claiming the balance.
    or: Option<Vec<AllClaimableAssetsOr>>,
}

/// Represents the logical 'OR' condition within a claimable balance's claim predicate.
///
/// This struct is part of the conditions defining the circumstances under which a claimable balance can be claimed. It includes conditions that are combined using a logical 'OR'—if any condition is met, the claimable balance can be claimed.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct AllClaimableAssetsOr {
    /// A vector of 'AND' conditions. To claim the balance, at least one set of 'AND' conditions must be met.
    and: Option<Vec<AllClaimableAssetsResponseAnd>>,
    /// The absolute time before which the balance can be claimed, represented as a string.
    #[serde(rename = "abs_before")]
    abs_before: Option<String>,
    /// The absolute time before which the balance can be claimed, represented in epoch time as a string.
    #[serde(rename = "abs_before_epoch")]
    abs_before_epoch: Option<String>,
}

/// Represents the logical 'AND' condition within a claimable balance's claim predicate.
///
/// This struct contains conditions that must all be met together for the claimable balance to be claimed.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct AllClaimableAssetsResponseAnd {
    /// The logical 'NOT' condition within the 'AND' conditions. The condition specified in 'NOT' must not be true for the claim to be valid.
    not: Option<AllClaimableBalancesResponseNot>,
    /// The absolute time before which the balance can be claimed, represented as a string.
    #[serde(rename = "abs_before")]
    abs_before: Option<String>,
    /// The absolute time before which the balance can be claimed, represented in epoch time as a string.
    #[serde(rename = "abs_before_epoch")]
    abs_before_epoch: Option<String>,
}

/// Represents the logical 'NOT' condition within a claimable balance's claim predicate.
///
/// This struct negates the condition specified within it. For a claim to be valid, the condition in 'NOT' must not be true.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct AllClaimableBalancesResponseNot {
    /// The absolute time before which the balance cannot be claimed, represented as a string.
    #[serde(rename = "abs_before")]
    abs_before: String,
    /// The absolute time before which the balance cannot be claimed, represented in epoch time as a string.
    #[serde(rename = "abs_before_epoch")]
    abs_before_epoch: String,
}

/// Represents the flags indicating various properties of the claimable balance.
///
/// This struct defines the boolean flags that can be set on a claimable balance, indicating certain features or restrictions.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct AllClaimableResponseFlags {
    /// Indicates if the clawback feature is enabled for the claimable balance, which allows the asset issuer to clawback the balance under certain conditions.
    #[serde(rename = "clawback_enabled")]
    clawback_enabled: bool,
}

impl Response for AllClaimableBalancesResponse {
    fn from_json(json: String) -> Result<Self, String> {
        let response = serde_json::from_str(&json).map_err(|e| e.to_string())?;

        Ok(response)
    }
}

/// This method checks if a claim is valid at a specific datetime.
impl AllClaimableAssetsResponsePredicate {
    // This method checks if a claim is valid at a specific datetime.
    pub fn is_valid_claim(&self, datetime: DateTime<Utc>) -> bool {
        // If the predicate is marked as unconditional, the claim is always valid.
        if let Some(true) = self.unconditional {
            true
        } 
        // If there are 'or' conditions, check if any of these conditions validate the claim.
        else if let Some(or_conditions) = &self.or {
            or_conditions.iter().any(|or| or.is_valid(datetime))
        } 
        // If there are no conditions, the claim is valid.
        else {
            true
        }
    }
}


impl AllClaimableAssetsOr {
    // This method checks if any condition under 'or' validates the claim.
    fn is_valid(&self, datetime: DateTime<Utc>) -> bool {
        // If there are 'and' conditions, check if any combination of these conditions is valid.
        if let Some(and_conditions) = &self.and {
            and_conditions.iter().any(|and| and.is_valid(datetime))
        } 
        // If there is an 'abs_before' condition, check if the datetime is before this date.
        else if let Some(abs_before) = &self.abs_before {
            if let Ok(abs_before_date) = DateTime::parse_from_rfc3339(abs_before) {
                datetime < abs_before_date
            } else {
                false
            }
        } 
        // If no specific condition is found, the claim is valid.
        else {
            true
        }
    }
}

impl AllClaimableAssetsResponseAnd {
    // This method checks if all conditions under 'and' are met.
    fn is_valid(&self, datetime: DateTime<Utc>) -> bool {
        let mut is_valid = true;

        // If there is an 'abs_before' condition, check if the datetime is before this date.
        if let Some(abs_before) = &self.abs_before {
            if let Ok(abs_before_date) = DateTime::parse_from_rfc3339(abs_before) {
                is_valid &= datetime < abs_before_date;
            }
        }

        // If there is a 'not' condition, it should also validate the datetime.
        if let Some(not_condition) = &self.not {
            is_valid &= not_condition.is_valid(datetime);
        }

        is_valid
    }
}

impl AllClaimableBalancesResponseNot {
    // This method checks if the datetime does not fall before the specified date, negating the condition.
    fn is_valid(&self, datetime: DateTime<Utc>) -> bool {
        if let Ok(not_before_date) = DateTime::parse_from_rfc3339(&self.abs_before) {
            datetime >= not_before_date
        } else {
            false
        }
    }
}
