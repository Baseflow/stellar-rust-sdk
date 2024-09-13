use crate::{models::*, models::prelude::*, BuildQueryParametersExt};
use stellar_rust_sdk_derive::pagination;

/// Represents a request to list all claimable balances from the Stellar Horizon API.
///
/// This structure is used to construct a query to retrieve a comprehensive list of claimable balances, which
/// can be filtered by sponsor, asset, or claimant. Claimable balances are a feature of the Stellar network
/// that allows users to create a balance of assets that can be claimed by another account. It adheres to the structure and parameters required
/// by the Horizon API for retrieving a
/// <a href="https://developers.stellar.org/api/horizon/resources/list-all-claimable-balances">list of claimable balances</a>.
///
/// # Usage
///
/// Create an instance of this struct and set the desired query parameters to filter the list of claimable balances.
/// Pass this request object to the [`HorizonClient::get_all_claimable_balances`](crate::horizon_client::HorizonClient::get_all_claimable_balances)
/// method to fetch the corresponding data from the Horizon API.
///
/// # Example
/// ```
/// use stellar_rs::claimable_balances::all_claimable_balances_request::AllClaimableBalancesRequest;
/// use stellar_rs::models::{Order, prelude::*};
///
/// let request = AllClaimableBalancesRequest::new()
///     .set_sponsor("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7").unwrap() // optional sponsor filter
///     .set_asset(IssuedOrNative::Issued(AssetData{
///         asset_code: "USDC".to_string(),
///         asset_issuer: "GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7".to_string(),
///     })) // optional asset filter
///     .set_claimant("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7".to_string()).unwrap() // optional claimant filter
///     .set_limit(4).unwrap(); // optional limit for response records
///
/// // Use with HorizonClient::get_all_claimable_balances
/// ```
///
#[pagination]
#[derive(Default)]
pub struct AllClaimableBalancesRequest {
    /// Optional. Representing the account ID of the sponsor. When set, the response will
    ///   only include claimable balances sponsored by the specified account.
    sponsor: Option<String>,

    /// Optional. Indicates issued asset for which claimable balances are being queried.
    ///   When set, the response will filter claimable balances that hold this specific asset.
    asset: Option<IssuedOrNative>,

    /// Optional. Represents the account ID of the claimant. If provided, the response will
    ///   include only claimable balances that are claimable by the specified account.
    claimant: Option<String>,
}

impl Request for AllClaimableBalancesRequest {
    fn get_query_parameters(&self) -> String {
        let prefix = "asset=";
        let asset = match &self.asset {
            Some(IssuedOrNative::Native) => format!("{}native", prefix),
            Some(IssuedOrNative::Issued(asset_data)) => {
                format!(
                    "{}{}%3A{}",
                    prefix, asset_data.asset_code, asset_data.asset_issuer
                )
            },
            None => String::new()
        };

        vec![
            self.sponsor.as_ref().map(|s| format!("sponsor={}", s)),
            Some(asset),
            self.claimant.as_ref().map(|c| format!("claimant={}", c)),
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
        ]
        .build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}/{}",
            base_url,
            super::CLAIMABLE_BALANCES_PATH,
            self.get_query_parameters()
        )
    }
}

impl AllClaimableBalancesRequest {
    /// Creates a new `AllClaimableBalancesRequest` with default parameters.
    pub fn new() -> Self {
        AllClaimableBalancesRequest::default()
    }

    /// Specifies the sponsor's public key in the request.
    ///
    /// # Arguments
    /// * `sponsor` - A Stellar public key of the sponsor whose claimable balances are to be retrieved.
    ///
    pub fn set_sponsor(
        self,
        sponsor: impl Into<String>,
    ) -> Result<AllClaimableBalancesRequest, String> {
        let sponsor = sponsor.into();
        if let Err(e) = is_public_key(&sponsor) {
            return Err(e.to_string());
        }

        Ok(AllClaimableBalancesRequest {
            sponsor: Some(sponsor),
            ..self
        })
    }

    /// Specifies the asset in the request.
    ///
    /// # Arguments
    /// * `asset` - The issued asset to filter claimable balances by asset type.
    ///
    pub fn set_asset(self, asset: IssuedOrNative) -> AllClaimableBalancesRequest {
        AllClaimableBalancesRequest {
            asset: Some(asset),
            ..self
        }
    }

    /// Specifies the claimant's public key in the request.
    ///
    /// # Arguments
    /// * `claimant` - A Stellar public key of the claimant whose claimable balances are to be retrieved.
    ///
    pub fn set_claimant(
        self,
        claimant: impl Into<String>,
    ) -> Result<AllClaimableBalancesRequest, String> {
        let claimant = claimant.into();
        if let Err(e) = is_public_key(&claimant) {
            return Err(e.to_string());
        }

        Ok(AllClaimableBalancesRequest {
            claimant: Some(claimant),
            ..self
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_cursor_valid() {
        let request = AllClaimableBalancesRequest::new()
            .set_cursor(12345)
            .unwrap();
        assert_eq!(request.cursor.unwrap(), 12345);
    }

    #[test]
    fn test_set_cursor_invalid() {
        let request = AllClaimableBalancesRequest::new().set_cursor(0);
        assert_eq!(
            request.err().unwrap(),
            "Cursor must be greater than or equal to 1.".to_string()
        );
    }

    #[test]
    fn test_set_limit_valid() {
        let request = AllClaimableBalancesRequest::new().set_limit(20).unwrap();
        assert_eq!(request.limit.unwrap(), 20);
    }

    #[test]
    fn test_set_limit_invalid_low() {
        let request = AllClaimableBalancesRequest::new().set_limit(0);
        assert_eq!(
            request.err().unwrap(),
            "Limit must be between 1 and 200.".to_string()
        );
    }

    #[test]
    fn test_set_limit_invalid_high() {
        let request = AllClaimableBalancesRequest::new().set_limit(201);
        assert_eq!(
            request.err().unwrap(),
            "Limit must be between 1 and 200.".to_string()
        );
    }
}
