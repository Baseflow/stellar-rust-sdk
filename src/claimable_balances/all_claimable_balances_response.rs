use std::string::ParseError;

use chrono::{DateTime, NaiveDateTime, Utc};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::models::Response;

impl Response for AllClaimableBalancesResponse {
    fn from_json(json: String) -> Result<Self, String> {
        let response = serde_json::from_str(&json).map_err(|e| e.to_string())?;

        Ok(response)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct AllClaimableBalancesResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    #[serde(rename = "_embedded")]
    pub embedded: Embedded,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: Self_field,
    pub next: Next,
    pub prev: Prev,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Self_field {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Next {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Prev {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Embedded {
    pub records: Vec<Record>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    #[serde(rename = "_links")]
    pub links: Links2,
    pub id: String,
    pub asset: String,
    pub amount: String,
    pub sponsor: String,
    #[serde(rename = "last_modified_ledger")]
    pub last_modified_ledger: i64,
    #[serde(rename = "last_modified_time")]
    pub last_modified_time: String,
    pub claimants: Vec<Claimant>,
    pub flags: Flags,
    #[serde(rename = "paging_token")]
    pub paging_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Links2 {
    #[serde(rename = "self")]
    pub self_field: Self_field2,
    pub transactions: Transactions,
    pub operations: Operations,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Self_field2 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Transactions {
    pub href: String,
    pub templated: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Operations {
    pub href: String,
    pub templated: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Claimant {
    pub destination: String,
    pub predicate: Predicate,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Predicate {
    pub unconditional: Option<bool>,
    pub and: Option<Vec<And>>,
    pub or: Option<Vec<Or>>,
    pub not: Option<Not>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct And {
    pub not: Option<Not>,
    #[serde(rename = "abs_before")]
    pub abs_before: Option<String>,
    #[serde(rename = "abs_before_epoch")]
    pub abs_before_epoch: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Not {
    #[serde(rename = "abs_before")]
    pub abs_before: String,
    #[serde(rename = "abs_before_epoch")]
    pub abs_before_epoch: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Or {
    #[serde(rename = "abs_before")]
    pub abs_before: Option<String>,
    #[serde(rename = "abs_before_epoch")]
    pub abs_before_epoch: Option<String>,
    pub not: Option<Not>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Flags {
    #[serde(rename = "clawback_enabled")]
    pub clawback_enabled: bool,
}

impl Predicate {
    pub fn is_valid(&self, date: DateTime<Utc>) -> bool {
        match self {
            Predicate {
                unconditional: Some(true),
                ..
            } => true,
            Predicate {
                and: Some(ands), ..
            } => ands.iter().all(|cond| cond.is_valid(date)),
            Predicate { or: Some(ors), .. } => ors.iter().any(|cond| cond.is_valid(date)),
            Predicate { not: Some(not), .. } => !not.is_valid(date),
            _ => false,
        }
    }
}

impl And {
    fn is_valid(&self, date: DateTime<Utc>) -> bool {
        if let Some(not) = &self.not {
            if not.is_valid(date) {
                return false;
            }
        }
        self.abs_before_epoch
            .as_ref()
            .map(|d| date < parse_epoch(d))
            .unwrap_or(true)
    }
}

impl Or {
    fn is_valid(&self, date: DateTime<Utc>) -> bool {
        if let Some(not) = &self.not {
            if not.is_valid(date) {
                return true;
            }
        }
        self.abs_before_epoch
            .as_ref()
            .map(|d| date < parse_epoch(d))
            .unwrap_or(false)
    }
}

impl Not {
    fn is_valid(&self, date: DateTime<Utc>) -> bool {
        date <= parse_epoch(&self.abs_before_epoch)
    }
}

fn parse_epoch(epoch_str: &str) -> DateTime<Utc> {
    // Convert the timestamp string into an i64
    let timestamp = epoch_str.parse::<i64>().unwrap();

    // Create a NaiveDateTime from the timestamp
    let naive = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();

    // Create a normal DateTime from the NaiveDateTime
    let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive, Utc);

    return datetime;
}
