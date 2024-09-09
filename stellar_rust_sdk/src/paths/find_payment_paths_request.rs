use crate::models::{is_public_key, Request};
use crate::paths::*;
use crate::BuildQueryParametersExt;

/// Represents a request to find payment paths on the Stellar Horizon API.
///
/// This struct is designed to construct a query for discovering viable payment paths from a
/// source account to a destination account, given specific destination assets and amounts.
/// It adheres to the structure and parameters required by the Horizon API for retrieving
/// payment paths.
///
/// # Usage
///
/// Create an instance using the `new` method, and then specify the destination asset, amount,
/// and source account using the provided setters. Once the required parameters are set, you
/// can pass this request object to the appropriate method in the Horizon client to fetch
/// the available payment paths.
///
/// # Example
/// ```
/// use stellar_rs::paths::prelude::*;
/// use stellar_rs::paths::{AssetType};
///
/// let request = FindPaymentsPathRequest::new()
///     .set_destination_asset(AssetType::Native).unwrap() // Sets the destination asset to native XLM.
///     .set_destination_amount("100.0".to_string()).unwrap() // Sets the amount of the destination asset.
///     .set_source_account("GCDNJUBQSXK57MSKJ4NSXK5DT5CJMMXMWUE7BN6NTJ6JTH23HQVYXG2C".to_string()).unwrap() // Sets the source account.
///     .set_destination_account("GAZD7JY7RCZN7KJ27SMUGKDPF7GQTYPXLDU7TFTJNSDB3MLO3M22DEIV".to_string()).unwrap(); // Sets the destination account.
/// ```
///
#[derive(Default)]
pub struct FindPaymentsPathRequest<
    DAs = NoDestinationAsset,
    DAm = NoDestinationAmount,
    S = NoSourceAccount,
> {
    /// Represents the asset type being sent to the destination account.
    pub destination_asset: DAs,
    /// Specifies the amount of the destination asset to be received.
    pub destination_amount: DAm,
    /// Optionally contains the public key of the destination account.
    pub destination_account: Option<String>,
    /// Identifies the source account from which the payment path originates.
    pub source_account: S,
}

impl FindPaymentsPathRequest<NoDestinationAsset, NoDestinationAmount, NoSourceAccount> {
    /// Creates a new `FindPaymentsPathRequest` with default parameters.
    pub fn new() -> Self {
        FindPaymentsPathRequest {
            destination_asset: NoDestinationAsset,
            destination_amount: NoDestinationAmount,
            destination_account: None,
            source_account: NoSourceAccount,
        }
    }
}

impl<DAs, DAm, S> FindPaymentsPathRequest<DAs, DAm, S> {
    /// Sets the destination asset for the payment path request.
    ///
    /// # Arguments
    /// * `destination_asset_type` - The type of asset being sent to the destination account.
    ///
    /// # Returns
    /// A new instance of `FindPaymentsPathRequest` with the destination asset set.
    ///
    pub fn set_destination_asset(
        self,
        destination_asset_type: AssetType,
    ) -> Result<FindPaymentsPathRequest<DestinationAsset, DAm, S>, String> {
        Ok(FindPaymentsPathRequest {
            destination_asset: DestinationAsset(destination_asset_type),
            destination_amount: self.destination_amount,
            destination_account: self.destination_account,
            source_account: self.source_account,
        })
    }

    /// Sets the destination amount for the payment path request.
    ///
    /// # Arguments
    /// * `destination_amount` - The amount of the asset to be received by the destination account.
    ///
    /// # Returns
    /// A new instance of `FindPaymentsPathRequest` with the destination amount set.
    ///
    pub fn set_destination_amount(
        self,
        destination_amount: impl Into<String>,
    ) -> Result<FindPaymentsPathRequest<DAs, DestinationAmount, S>, String> {
        Ok(FindPaymentsPathRequest {
            destination_asset: self.destination_asset,
            destination_amount: DestinationAmount(destination_amount.into()),
            destination_account: self.destination_account,
            source_account: self.source_account,
        })
    }

    /// Sets the source account for the payment path request.
    ///
    /// # Arguments
    /// * `source_account` - The Stellar public key of the source account.
    ///
    /// # Returns
    /// A new instance of `FindPaymentsPathRequest` with the source account set
    ///
    pub fn set_source_account(
        self,
        source_account: impl Into<String>,
    ) -> Result<FindPaymentsPathRequest<DAs, DAm, SourceAccount>, String> {
        let source_account = source_account.into();
        if let Err(e) = is_public_key(&source_account) {
            return Err(e.to_string());
        }

        Ok(FindPaymentsPathRequest {
            destination_asset: self.destination_asset,
            destination_amount: self.destination_amount,
            destination_account: self.destination_account,
            source_account: SourceAccount(source_account),
        })
    }
}

impl FindPaymentsPathRequest<DestinationAsset, DestinationAmount, SourceAccount> {
    /// Sets the destination account for the payment path request.
    ///
    /// # Arguments
    /// * `destination_account` - The Stellar public key of the destination account.
    ///
    /// # Returns
    /// A new instance of `FindPaymentsPathRequest` with the destination account set.
    ///
    pub fn set_destination_account(
        self,
        destination_account: impl Into<String>,
    ) -> Result<FindPaymentsPathRequest<DestinationAsset, DestinationAmount, SourceAccount>, String>
    {
        let destination_account = destination_account.into();
        if let Err(e) = is_public_key(&destination_account) {
            return Err(e.to_string());
        }

        Ok(FindPaymentsPathRequest {
            destination_asset: self.destination_asset,
            destination_amount: self.destination_amount,
            destination_account: Some(destination_account),
            source_account: self.source_account,
        })
    }
}

impl Request for FindPaymentsPathRequest<DestinationAsset, DestinationAmount, SourceAccount> {
    fn get_query_parameters(&self) -> String {
        let asset_type_prefix = "destination_asset_type=";
        let asset_code_prefix = "&destination_asset_code=";
        let asset_issuer_prefix = "&destination_asset_issuer=";

        // Construct parameters for destination asset.
        let parameters = match &self.destination_asset {
            DestinationAsset(AssetType::Native) => format!("{}native", asset_type_prefix),
            DestinationAsset(AssetType::CreditAlphanum4(asset_data))
            | DestinationAsset(AssetType::CreditAlphanum12(asset_data)) => {
                let asset_type = match self.destination_asset {
                    DestinationAsset(AssetType::CreditAlphanum4(_)) => "credit_alphanum4",
                    DestinationAsset(AssetType::CreditAlphanum12(_)) => "credit_alphanum12",
                    _ => "", // should not be reached
                };

                format!(
                    "{}{}{}{}{}{}",
                    asset_type_prefix,
                    asset_type,
                    asset_code_prefix,
                    asset_data.asset_code,
                    asset_issuer_prefix,
                    asset_data.issuer_account_id
                )
            }
        };

        // Construct and return the query parameters.
        vec![
            Some(parameters),
            Some(format!("destination_amount={}", self.destination_amount.0)),
            self.destination_account
                .as_ref()
                .map(|d| format!("destination_account={}", d)),
            Some(format!("source_account={}", self.source_account.0)),
        ]
        .build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}{}",
            base_url,
            super::PATHS_PATH,
            self.get_query_parameters()
        )
    }
}
