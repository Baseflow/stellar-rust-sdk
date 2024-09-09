use crate::models::{is_public_key, Request};
use crate::paths::*;
use crate::BuildQueryParametersExt;

/// Represents the absence of either a source account or source asset(s).
#[derive(Default, Clone, Debug)]
pub struct NoSource;

/// Represents the source which can be either a vector of assets, or an account.
/// Exactly one of these must be set, in order to make a valid request.
#[derive(Clone, Debug)]
pub enum Source {
    /// A vector of assets available to the sender. Any returned path must start with an asset in this list.
    SourceAssets(Vec<IssuedOrNative>),
    /// The Stellar address of the sender. Any returned path must start with an asset that the sender holds.
    SourceAccount(String),
}

impl Default for Source {
    fn default() -> Self {
        Source::SourceAssets(Vec::new())
    }
}

/// Represents a request to list strict receive payment paths on the Stellar Horizon API.
///
/// This struct is designed to construct a query for listing payment paths that allow
/// a specified destination amount of a specified asset to be received, considering one or more
/// source assets and accounts. It adheres to the structure and parameters required by the Horizon API
/// for retrieving such payment paths.
///
/// # Usage
///
/// Create an instance using the `new` method, and then specify the destination asset, the destination
/// amount, and the source using the provided setters. Once the required parameters are set, optional
/// parameters can be set, and the request object can then be passed to the appropriate method
/// in the Horizon client to fetch the available strict receive payment paths.
///
/// # Example
/// ```
/// use stellar_rs::paths::prelude::*;
/// use stellar_rs::paths::{AssetType, IssuedOrNative};
///
/// let request = ListStrictReceivePaymentPathsRequest::new()
///     .set_destination_asset(AssetType::Native).unwrap() // Sets the destination asset to native XLM.
///     .set_destination_amount("100.0".to_string()).unwrap() // Sets the amount of the destination asset.
///     .set_source(Source::SourceAccount("GCDNJUBQSXK57MSKJ4NSXK5DT5CJMMXMWUE7BN6NTJ6JTH23HQVYXG2C".to_string())).unwrap() // Sets the source account.
///     .set_destination_account("GAZD7JY7RCZN7KJ27SMUGKDPF7GQTYPXLDU7TFTJNSDB3MLO3M22DEIV".to_string()).unwrap(); // Sets the destination account.
/// ```
///
#[derive(Default, Clone)]
pub struct ListStrictReceivePaymentPathsRequest<
    DAs = NoDestinationAsset,
    DAm = NoDestinationAmount,
    S = Source,
> {
    /// Represents the asset type being received by the destination account.
    destination_asset: DAs,
    /// Specifies the amount of the destination asset to be received.
    destination_amount: DAm,
    /// Optionally contains the public key of the destination account.
    destination_account: Option<String>,
    /// Represents the source which can be either a vector of assets, or an account.
    source: S,
}

impl ListStrictReceivePaymentPathsRequest<NoDestinationAsset, NoDestinationAmount, NoSource> {
    /// Creates a new `ListStrictReceivePaymentPathsRequest` with default parameters.
    pub fn new() -> Self {
        ListStrictReceivePaymentPathsRequest {
            destination_asset: NoDestinationAsset,
            destination_amount: NoDestinationAmount,
            destination_account: None,
            source: NoSource,
        }
    }
}

impl<DAs, DAm, S> ListStrictReceivePaymentPathsRequest<DAs, DAm, S> {
    /// Sets the destination asset for the payment path request.
    ///
    /// # Arguments
    /// * `destination_asset_type` - The type of asset being received by the destination account.
    ///
    /// # Returns
    /// A new instance of `ListStrictReceivePaymentPathsRequest` with the destination asset set.
    ///
    pub fn set_destination_asset(
        self,
        destination_asset_type: AssetType,
    ) -> Result<ListStrictReceivePaymentPathsRequest<DestinationAsset, DAm, S>, String> {
        Ok(ListStrictReceivePaymentPathsRequest {
            destination_asset: DestinationAsset(destination_asset_type),
            destination_amount: self.destination_amount,
            destination_account: self.destination_account,
            source: self.source,
        })
    }

    /// Sets the destination amount for the payment path request.
    ///
    /// # Arguments
    /// * `destination_amount` - The amount of the asset to be received by the destination account.
    ///
    /// # Returns
    /// A new instance of `ListStrictReceivePaymentPathsRequest` with the destination amount set.
    ///
    pub fn set_destination_amount(
        self,
        destination_amount: impl Into<String>,
    ) -> Result<ListStrictReceivePaymentPathsRequest<DAs, DestinationAmount, S>, String> {
        Ok(ListStrictReceivePaymentPathsRequest {
            destination_asset: self.destination_asset,
            destination_amount: DestinationAmount(destination_amount.into()),
            destination_account: self.destination_account,
            source: self.source,
        })
    }

    /// Sets the source for the payment path request.
    ///
    /// # Arguments
    /// * `source` - One of the `Source` enum types.
    ///
    /// # Returns
    /// A new instance of `ListStrictReceivePaymentPathsRequest` with the source set.
    ///
    pub fn set_source(
        self,
        source: Source,
    ) -> Result<ListStrictReceivePaymentPathsRequest<DAs, DAm, Source>, String> {
        match &source {
            Source::SourceAssets(assets) => {
                if assets.is_empty() {
                    return Err("SourceAssets cannot be empty".to_string());
                }
            }
            Source::SourceAccount(account) => {
                if let Err(e) = is_public_key(&account) {
                    return Err(e.to_string());
                }
            }
        }

        Ok(ListStrictReceivePaymentPathsRequest {
            destination_asset: self.destination_asset,
            destination_amount: self.destination_amount,
            destination_account: self.destination_account,
            source: source,
        })
    }
}

impl ListStrictReceivePaymentPathsRequest<DestinationAsset, DestinationAmount, Source> {
    /// Sets the destination account for the payment path request.
    ///
    /// # Arguments
    /// * `destination_account` - The Stellar public key of the destination account.
    ///
    /// # Returns
    /// A new instance of `ListStrictReceivePaymentPathsRequest` with the destination account set.
    ///
    pub fn set_destination_account(
        self,
        destination_account: impl Into<String>,
    ) -> Result<ListStrictReceivePaymentPathsRequest<DestinationAsset, DestinationAmount>, String>
    {
        let destination_account = destination_account.into();
        if let Err(e) = is_public_key(&destination_account) {
            return Err(e.to_string());
        }

        Ok(ListStrictReceivePaymentPathsRequest {
            destination_asset: self.destination_asset,
            destination_amount: self.destination_amount,
            destination_account: Some(destination_account.into()),
            source: self.source,
        })
    }
}

impl Request for ListStrictReceivePaymentPathsRequest<DestinationAsset, DestinationAmount, Source> {
    fn get_query_parameters(&self) -> String {
        let asset_type_prefix = "destination_asset_type=";
        let asset_code_prefix = "&destination_asset_code=";
        let asset_issuer_prefix = "&destination_asset_issuer=";

        // Construct parameters for destination asset.
        let destination_asset_parameters = match &self.destination_asset {
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
                    asset_issuer_prefix,
                    asset_data.issuer_account_id,
                    asset_code_prefix,
                    asset_data.asset_code,
                )
            }
        };

        let source = match &self.source {
            Source::SourceAssets(source_assets) => {
                // Construct source asset parameters, if any.
                // If no source asset parameters are set, return an empty vector which will later be ignored.
                source_assets
                    .iter()
                    .enumerate()
                    .map(|(i, asset)| {
                        let prefix = if i == 0 { "source_assets=" } else { "%2C" };
                        match asset {
                            IssuedOrNative::Native => format!("{}native", prefix),
                            IssuedOrNative::Issued(asset_data) => {
                                format!(
                                    "{}{}%3A{}",
                                    prefix, asset_data.asset_code, asset_data.issuer_account_id
                                )
                            }
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("")
            }
            Source::SourceAccount(account) => {
                format!("source_account={}", account)
            }
        };

        // Create query parameters vector.
        let query_parameters = vec![
            Some(destination_asset_parameters),
            Some(format!("destination_amount={}", self.destination_amount.0)),
            self.destination_account
                .as_ref()
                .map(|d| format!("destination_account={}", d)),
            Some(source),
        ];

        query_parameters.build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}/{}{}",
            base_url,
            super::PATHS_PATH,
            super::PATHS_STRICT_RECEIVE_PATH,
            self.get_query_parameters()
        )
    }
}
