use crate::models::Request;
use crate::paths::*;
use crate::BuildQueryParametersExt;

/// Represents a request to list strict send payment paths on the Stellar Horizon API.
///
/// This struct is designed to construct a query for discovering payment paths that allow
/// a specified destination amount of a specified asset to be sent, considering one or more
/// source assets. It adheres to the structure and parameters required by the Horizon API
/// for retrieving such payment paths.
///
/// # Usage
///
/// Create an instance using the `new` method, and then specify the destination asset, amount,
/// destination account, and source assets using the provided setters. Once the required parameters are set,
/// you can pass this request object to the appropriate method in the Horizon client to fetch
/// the available strict send payment paths.
///
/// # Example
/// ```
/// use stellar_rs::paths::prelude::*;
/// use stellar_rs::paths::{AssetType, SourceAsset, IssuedOrNative};
///
/// let request = ListStrictSendPaymentPathsRequest::new()
///     .set_destination_asset(AssetType::Native).unwrap() // Sets the destination asset to native XLM.
///     .set_destination_amount("100".to_string()).unwrap() // Sets the amount of the destination asset.
///     .set_destination_account("GAZD7JY7RCZN7KJ27SMUGKDPF7GQTYPXLDU7TFTJNSDB3MLO3M22DEIV".to_string()).unwrap() // Sets the destination account.
///     .set_source_assets(vec![SourceAsset(IssuedOrNative::Native)]).unwrap(); // Sets the source assets.
/// ```
///
#[derive(Default, Clone)]
pub struct ListStrictSendPaymentPathsRequest<DAs = NoDestinationAsset, DAm = NoDestinationAmount> {
    /// Represents the asset type being received by the destination account.
    destination_asset: DAs,
    /// Specifies the amount of the destination asset to be received.
    destination_amount: DAm,
    /// Optionally contains the public key of the destination account.
    destination_account: Option<String>,
    /// Optionally contains a list of source assets to consider when finding payment paths.
    source_assets: Option<Vec<SourceAsset>>,
}

impl ListStrictSendPaymentPathsRequest<NoDestinationAsset, NoDestinationAmount> {
    /// Creates a new `ListStrictSendPaymentPathsRequest` with default parameters.
    pub fn new() -> Self {
        ListStrictSendPaymentPathsRequest {
            destination_asset: NoDestinationAsset,
            destination_amount: NoDestinationAmount,
            destination_account: None,
            source_assets: None,
        }
    }
}

impl<DAs, DAm> ListStrictSendPaymentPathsRequest<DAs, DAm> {
    /// Sets the destination asset for the payment path request.
    ///
    /// # Arguments
    /// * `destination_asset_type` - The type of asset being received by the destination account.
    ///
    /// # Returns
    /// A new instance of `ListStrictSendPaymentPathsRequest` with the destination asset set.
    ///
    pub fn set_destination_asset(
        self,
        destination_asset_type: AssetType,
    ) -> Result<ListStrictSendPaymentPathsRequest<DestinationAsset, DAm>, String> {
        Ok(ListStrictSendPaymentPathsRequest {
            destination_asset: DestinationAsset(destination_asset_type),
            destination_amount: self.destination_amount,
            destination_account: self.destination_account,
            source_assets: self.source_assets,
        })
    }

    /// Sets the destination amount for the payment path request.
    ///
    /// # Arguments
    /// * `destination_amount` - The amount of the asset to be received by the destination account.
    ///
    /// # Returns
    /// A new instance of `ListStrictSendPaymentPathsRequest` with the destination amount set.
    ///
    pub fn set_destination_amount(
        self,
        destination_amount: String,
    ) -> Result<ListStrictSendPaymentPathsRequest<DAs, DestinationAmount>, String> {
        Ok(ListStrictSendPaymentPathsRequest {
            destination_asset: self.destination_asset,
            destination_amount: DestinationAmount(destination_amount),
            destination_account: self.destination_account,
            source_assets: self.source_assets,
        })
    }
}

impl ListStrictSendPaymentPathsRequest<DestinationAsset, DestinationAmount> {
    /// Sets the destination account for the payment path request.
    ///
    /// # Arguments
    /// * `destination_account` - The Stellar public key of the destination account.
    ///
    /// # Returns
    /// A new instance of `ListStrictSendPaymentPathsRequest` with the destination account set.
    ///
    pub fn set_destination_account(
        self,
        destination_account: String,
    ) -> Result<ListStrictSendPaymentPathsRequest<DestinationAsset, DestinationAmount>, String>
    {
        Ok(ListStrictSendPaymentPathsRequest {
            destination_asset: self.destination_asset,
            destination_amount: self.destination_amount,
            destination_account: Some(destination_account),
            source_assets: self.source_assets,
        })
    }
    /// Sets the source assets for the payment path request.
    ///
    /// # Arguments
    /// * `source_assets` - A list of source assets to consider when finding payment paths.
    ///
    /// # Returns
    /// A new instance of `ListStrictSendPaymentPathsRequest` with the source assets set.
    ///
    pub fn set_source_assets(
        self,
        source_assets: Vec<SourceAsset>,
    ) -> Result<ListStrictSendPaymentPathsRequest<DestinationAsset, DestinationAmount>, String>
    {
        Ok(ListStrictSendPaymentPathsRequest {
            destination_asset: self.destination_asset,
            destination_amount: self.destination_amount,
            destination_account: self.destination_account,
            source_assets: Some(source_assets),
        })
    }
}

impl Request for ListStrictSendPaymentPathsRequest<DestinationAsset, DestinationAmount> {
    fn get_query_parameters(&self) -> String {
        let asset_type_prefix = "destination_asset_type=";
        let asset_code_prefix = "&destination_asset_code=";
        let asset_issuer_prefix = "&destination_asset_issuer=";

        // Construct parameters for destination asset.
        let destination_asset_parameters = match &self.destination_asset {
            DestinationAsset(AssetType::Native) => format!("{}=native", asset_type_prefix),
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

        // Construct source asset parameters, if any.
        // If no source asset parameters are set, return an empty vector which will later be ignored.
        let source_assets_parameters =
            self.source_assets.as_ref().map_or(String::new(), |assets| {
                assets
                    .iter()
                    .enumerate()
                    .map(|(i, asset)| {
                        let prefix = if i == 0 { "source_assets=" } else { "%2C" };
                        match asset {
                            SourceAsset(IssuedOrNative::Native) => format!("{}native", prefix),
                            SourceAsset(IssuedOrNative::Issued(asset_data)) => {
                                format!(
                                    "{}{}%3A{}",
                                    prefix, asset_data.asset_code, asset_data.issuer_account_id
                                )
                            }
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("")
            });

        // Create query parameters vector.
        let mut query_parameters = vec![
            Some(destination_asset_parameters),
            Some(format!("destination_amount={}", self.destination_amount.0)),
            self.destination_account
                .as_ref()
                .map(|d| format!("destination_account={}", d)),
        ];

        // Only add source assets parameters if the vector is not empty, to prevent a trailing `&`.
        if !source_assets_parameters.is_empty() {
            query_parameters.push(Some(source_assets_parameters));
        }

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