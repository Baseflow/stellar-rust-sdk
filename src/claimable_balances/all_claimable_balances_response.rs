use chrono::DateTime;
use chrono::Utc;
use derive_getters::Getters;
use serde::Deserialize;
use serde::Serialize;

use crate::models::Response;

#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct AllClaimableBalancesResponse {
    #[serde(rename = "_links")]
    links: Links,
    #[serde(rename = "_embedded")]
    embedded: Embedded,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(rename = "self")]
    self_field: Self_field,
    next: Next,
    prev: Prev,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Self_field {
    href: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Next {
    href: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Prev {
    href: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Embedded {
    records: Vec<Record>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    #[serde(rename = "_links")]
    links: RecordsLinks,
    id: String,
    asset: String,
    amount: String,
    sponsor: String,
    #[serde(rename = "last_modified_ledger")]
    last_modified_ledger: i64,
    #[serde(rename = "last_modified_time")]
    last_modified_time: String,
    claimants: Vec<Claimant>,
    flags: Flags,
    #[serde(rename = "paging_token")]
    paging_token: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct RecordsLinks {
    #[serde(rename = "self")]
    self_field: Records_Self_field,
    transactions: Transactions,
    operations: Operations,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Records_Self_field {
    href: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Transactions {
    href: String,
    templated: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Operations {
    href: String,
    templated: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Claimant {
    destination: String,
    predicate: Predicate,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Predicate {
    unconditional: Option<bool>,
    or: Option<Vec<Or>>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Or {
    and: Option<Vec<And>>,
    #[serde(rename = "abs_before")]
    abs_before: Option<String>,
    #[serde(rename = "abs_before_epoch")]
    abs_before_epoch: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct And {
    not: Option<Not>,
    #[serde(rename = "abs_before")]
    abs_before: Option<String>,
    #[serde(rename = "abs_before_epoch")]
    abs_before_epoch: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Not {
    #[serde(rename = "abs_before")]
    abs_before: String,
    #[serde(rename = "abs_before_epoch")]
    abs_before_epoch: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Flags {
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
impl Predicate {
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


impl Or {
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

impl And {
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

impl Not {
    // This method checks if the datetime does not fall before the specified date, negating the condition.
    fn is_valid(&self, datetime: DateTime<Utc>) -> bool {
        if let Ok(not_before_date) = DateTime::parse_from_rfc3339(&self.abs_before) {
            datetime >= not_before_date
        } else {
            false
        }
    }
}
