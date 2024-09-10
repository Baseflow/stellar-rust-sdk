use crate::models::{is_public_key, Request};
use crate::paths::*;
use crate::BuildQueryParametersExt;

/// Represents the absence of a source asset for a payment path request.
#[derive(Default, Clone, Debug)]
pub struct NoSourceAsset;

/// Represents the source asset for a payment path request.
#[derive(Default, Clone, Debug)]
pub struct SourceAsset(AssetType);

/// Represents the absence of a source amount for a payment path request.
#[derive(Default, Clone, Debug)]
pub struct NoSourceAmount;

/// Represents the source amount for a payment path request.
#[derive(Default, Clone, Debug)]
pub struct SourceAmount(String);

/// Represents the absence of a source amount for a payment path request.
#[derive(Default, Clone, Debug)]
pub struct NoDestination;

/// Represents the destination which can be either a vector of assets, or an account.
/// Exactly one of these must be set, in order to make a valid request.
#[derive(Clone, Debug)]
pub enum Destination {
    DestinationAssets(Vec<IssuedOrNative>),
    DestinationAccount(String),
}

impl Default for Destination {
    fn default() -> Self {
        Destination::DestinationAssets(Vec::new())
    }
}

/// Represents a request to list strict send payment paths on the Stellar Horizon API.
///
/// This struct is designed to construct a query for listing the paths a payment can take based
/// on the amount of an asset you want the recipient to receive. The destination asset amount
/// stays constant, and the type and amount of an asset sent varies based on offers in the order books.
///
/// # Usage
///
/// Create an instance using the `new` method, and then specify the source asset, source amount,
/// and the destination using the provided setters. Once the required parameters are set, optional
/// parameters can be set, and the request object can then be passed to the appropriate method
/// in the Horizon client to fetch the available strict send payment paths.
///
/// # Example
/// ```
/// use stellar_rs::paths::prelude::*;
/// use stellar_rs::models::prelude::*;
///
/// let request = ListStrictSendPaymentPathsRequest::new()
///     .set_source_asset(AssetType::Native).unwrap() // Sets the source asset to native XLM.
///     .set_source_amount("100".to_string()).unwrap() // Sets the amount of the source asset.
///     .set_destination(Destination::DestinationAccount("GAZD7JY7RCZN7KJ27SMUGKDPF7GQTYPXLDU7TFTJNSDB3MLO3M22DEIV".to_string())).unwrap(); // Sets an account as destination.
/// ```
///
#[derive(Default, Clone)]
pub struct ListStrictSendPaymentPathsRequest<
    SAs = NoSourceAsset,
    SAm = NoSourceAmount,
    D = Destination,
> {
    /// Represents the asset type being received by the source account.
    source_asset: SAs,
    /// Specifies the amount of the source asset to be received.
    source_amount: SAm,
    /// Represents the destination which can be either a vector of assets, or an account.
    destination: D,
}

impl ListStrictSendPaymentPathsRequest<NoSourceAsset, NoSourceAmount, NoDestination> {
    /// Creates a new `ListStrictSendPaymentPathsRequest` with default parameters.
    pub fn new() -> Self {
        ListStrictSendPaymentPathsRequest {
            source_asset: NoSourceAsset,
            source_amount: NoSourceAmount,
            destination: NoDestination,
        }
    }
}

impl<SAs, SAm, D> ListStrictSendPaymentPathsRequest<SAs, SAm, D> {
    /// Sets the source asset for the payment path request.
    ///
    /// # Arguments
    /// * `source_asset_type` - The type of asset being received by the source account.
    ///
    /// # Returns
    /// A new instance of `ListStrictSendPaymentPathsRequest` with the source asset set.
    ///
    pub fn set_source_asset(
        self,
        source_asset_type: AssetType,
    ) -> Result<ListStrictSendPaymentPathsRequest<SourceAsset, SAm, D>, String> {
        Ok(ListStrictSendPaymentPathsRequest {
            source_asset: SourceAsset(source_asset_type),
            source_amount: self.source_amount,
            destination: self.destination,
        })
    }

    /// Sets the source amount for the payment path request.
    ///
    /// # Arguments
    /// * `source_amount` - The amount of the asset to be received by the source account.
    ///
    /// # Returns
    /// A new instance of `ListStrictSendPaymentPathsRequest` with the source amount set.
    ///
    pub fn set_source_amount(
        self,
        source_amount: impl Into<String>,
    ) -> Result<ListStrictSendPaymentPathsRequest<SAs, SourceAmount, D>, String> {
        Ok(ListStrictSendPaymentPathsRequest {
            source_asset: self.source_asset,
            source_amount: SourceAmount(source_amount.into()),
            destination: self.destination,
        })
    }

    /// Sets the destination for the payment path request.
    ///
    /// # Arguments
    /// * `destination` - One of the `Destination` enum types.
    ///
    /// # Returns
    /// A new instance of `ListStrictSendPaymentPathsRequest` with the destination set.
    ///
    pub fn set_destination(
        self,
        destination: Destination,
    ) -> Result<ListStrictSendPaymentPathsRequest<SAs, SAm, Destination>, String> {
        match &destination {
            Destination::DestinationAssets(assets) => {
                if assets.is_empty() {
                    return Err("DestinationAssets cannot be empty".to_string());
                }
            }
            Destination::DestinationAccount(account) => {
                if let Err(e) = is_public_key(&account) {
                    return Err(e.to_string());
                }
            }
        }

        Ok(ListStrictSendPaymentPathsRequest {
            source_asset: self.source_asset,
            source_amount: self.source_amount,
            destination: destination,
        })
    }
}

impl Request for ListStrictSendPaymentPathsRequest<SourceAsset, SourceAmount, Destination> {
    fn get_query_parameters(&self) -> String {
        let asset_type_prefix = "source_asset_type=";
        let asset_code_prefix = "&source_asset_code=";
        let asset_issuer_prefix = "&source_asset_issuer=";

        // Construct parameters for source asset.
        let source_asset_parameters = match &self.source_asset {
            SourceAsset(AssetType::Native) => format!("{}native", asset_type_prefix),
            SourceAsset(AssetType::Alphanumeric4(asset_data))
            | SourceAsset(AssetType::Alphanumeric12(asset_data)) => {
                let asset_type = match self.source_asset {
                    SourceAsset(AssetType::Alphanumeric4(_)) => "credit_alphanum4",
                    SourceAsset(AssetType::Alphanumeric12(_)) => "credit_alphanum12",
                    _ => "", // should not be reached
                };

                format!(
                    "{}{}{}{}{}{}",
                    asset_type_prefix,
                    asset_type,
                    asset_issuer_prefix,
                    asset_data.asset_issuer,
                    asset_code_prefix,
                    asset_data.asset_code,
                )
            }
        };

        let destination = match &self.destination {
            Destination::DestinationAssets(destination_assets) => {
                // Construct destination asset parameters, if any.
                // If no destination asset parameters are set, return an empty vector which will later be ignored.
                destination_assets
                    .iter()
                    .enumerate()
                    .map(|(i, asset)| {
                        let prefix = if i == 0 { "destination_assets=" } else { "%2C" };
                        match asset {
                            IssuedOrNative::Native => format!("{}native", prefix),
                            IssuedOrNative::Issued(asset_data) => {
                                format!(
                                    "{}{}%3A{}",
                                    prefix, asset_data.asset_code, asset_data.asset_issuer
                                )
                            }
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("")
            }
            Destination::DestinationAccount(account) => {
                format!("destination_account={}", account)
            }
        };

        // Create query parameters vector.
        let query_parameters = vec![
            Some(format!("source_amount={}", self.source_amount.0)),
            Some(destination),
            Some(source_asset_parameters),
        ];

        query_parameters.build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}/{}{}",
            base_url,
            super::PATHS_PATH,
            super::PATHS_STRICT_SEND_PATH,
            self.get_query_parameters()
        )
    }
}
