/// Provides the `FindPaymentPathsRequest`.
///
/// # Usage
/// This module provides the `FindPaymentPathsRequest` struct, specifically designed for
/// constructing requests to find payment paths based on certain criteria. It is tailored for
/// use with the [`HorizonClient::get_find_payment_paths`](crate::horizon_client::HorizonClient::get_find_payment_paths)
/// method.
///
pub mod find_payment_paths_request;

/// Provides the `ListStrictReceivePaymentPathsRequest`.
///
/// # Usage
/// This module provides the `ListStrictReceivePaymentPathsRequest` struct, specifically designed for
/// constructing requests to list strict receive payment paths. It is tailored for
/// use with the [`HorizonClient::get_list_strict_receive_payment_paths`](crate::horizon_client::HorizonClient::get_list_strict_receive_payment_paths)
/// method.
///
pub mod list_strict_receive_payment_paths_request;

/// Provides the `ListStrictSendPaymentPathsRequest`.
///
/// # Usage
/// This module provides the `ListStrictSendPaymentPathsRequest` struct, specifically designed for
/// constructing requests to list strict send payment paths. It is tailored for
/// use with the [`HorizonClient::get_list_strict_send_payment_paths`](crate::horizon_client::HorizonClient::get_list_strict_send_payment_paths)
/// method.
///
pub mod list_strict_send_payment_paths_request;

/// Provides the response structures.
///
/// This module defines structures representing the responses from the payment path API.
/// The structures are designed to deserialize the JSON response into Rust objects, enabling
/// straightforward access to various details of payment paths.
///
/// # Usage
/// These structures are equipped with serialization capabilities to handle the JSON data from the
/// payment path server and with getter methods for easy field access.
///
pub mod response;

/// The base paths for path-related endpoints in the Horizon API.
///
/// # Usage
/// This variable is intended to be used internally by the request-building logic
/// to ensure consistent and accurate path construction for offer-related API calls.
///
pub(crate) static PATHS_PATH: &str = "paths"; // the base API path
pub(crate) static PATHS_STRICT_RECEIVE_PATH: &str = "strict-receive";
pub(crate) static PATHS_STRICT_SEND_PATH: &str = "strict-send";

/// Represents the absence of a destination asset for a payment path request.
#[derive(Default, Clone, Debug)]
pub struct NoDestinationAsset;

/// Represents a source asset for a payment path request.
#[derive(Default, Clone, Debug)]
pub struct DestinationAsset(AssetType);

/// Represents the absence of a destination amount for a payment path request.
#[derive(Default, Clone, Debug)]
pub struct NoDestinationAmount;

/// Represents the destination amount for a payment path request.
#[derive(Default, Clone, Debug)]
pub struct DestinationAmount(String);

/// Represents the absence of a source account for a payment path request.
#[derive(Default, Clone, Debug)]
pub struct NoSourceAccount;

/// Represents the source account for a payment path request.
#[derive(Default, Clone, Debug)]
pub struct SourceAccount(String);

/// Represents structure of the required asset.
#[derive(Default, Clone, Debug)]
pub enum AssetType {
    #[default]
    Native,
    CreditAlphanum4(Asset),
    CreditAlphanum12(Asset),
}

/// Represents an asset containing an asset code and issuer account ID.
#[derive(Clone, Debug)]
pub struct Asset {
    pub asset_code: String,
    pub issuer_account_id: String,
}

/// Represents structure of an asset used in the vector of optional assets.
#[derive(Default, Clone, Debug)]
pub enum IssuedOrNative {
    #[default]
    Native,
    Issued(Asset),
}

/// The `prelude` module of the `paths` module.
///
/// # Usage
/// This module serves as a convenience for users of the payment path Rust SDK, allowing for easy and
/// ergonomic import of the most commonly used items across various modules. It re-exports
/// key structs and traits from the sibling modules, simplifying access to these components
/// when using the library.
///
/// By importing the contents of `prelude`, users can conveniently access the primary
/// functionalities of the payment path-related modules without needing to import each item
/// individually.
///
/// # Contents
///
/// The `prelude` includes the following re-exports:
///
/// * From `find_payment_paths_request`: All items (e.g. `FindPaymentPathsRequest`).
/// * From `list_strict_receive_payment_paths_request`: All items (e.g. `ListStrictReceivePaymentPathsRequest`).
/// * From `list_strict_send_payment_paths_request`: All items (e.g. `ListStrictSendPaymentPathsRequest`).
/// * From `response`: All items (e.g. `PaymentPathResponse`, etc.).
///
pub mod prelude {
    pub use super::find_payment_paths_request::*;
    pub use super::list_strict_receive_payment_paths_request::*;
    pub use super::list_strict_send_payment_paths_request::*;
    pub use super::response::*;
    pub use super::{
        DestinationAmount, DestinationAsset, NoDestinationAmount, NoDestinationAsset,
        NoSourceAccount, SourceAccount,
    };
}

#[cfg(test)]
mod tests {
    use super::prelude::*;
    use super::{AssetType, IssuedOrNative};
    use crate::{horizon_client::HorizonClient, models::*};

    #[tokio::test]
    async fn test_find_payment_paths_request() {
        use crate::paths::{Asset, PATHS_PATH};

        // Test creating and sending a request with source assets. Only the response status will be checked, as the request will not yield comparable data.
        let request = FindPaymentsPathRequest::new()
            .set_destination_asset(AssetType::CreditAlphanum4(Asset {
                asset_code: "USDC".to_string(),
                issuer_account_id: "GBJJ5OCBXNZWHSJJ4YQ6ECK24MBJSZMLEMINHKGGEWUA5RU2EDMPN6MS"
                    .to_string(),
            }))
            .unwrap()
            .set_destination_amount("42".to_string())
            .unwrap()
            .set_source_account(
                "GBAC4BTW6UIJOCCUOZ7QATQPVWX6UQVH3ESQ6NEHBMCXJ3MVP4GMT77H".to_string(),
            )
            .unwrap()
            .set_destination_account(
                "GBAKINTNEGR7PO6Z6XW2S5ITT5VARNW6DZ5K4OYSLFNEA2CSMUM2UEF4".to_string(),
            )
            .unwrap();

        let expected_parameters =
            "?destination_asset_type=credit_alphanum4&destination_asset_code=USDC&destination_asset_issuer=GBJJ5OCBXNZWHSJJ4YQ6ECK24MBJSZMLEMINHKGGEWUA5RU2EDMPN6MS&destination_amount=42&destination_account=GBAKINTNEGR7PO6Z6XW2S5ITT5VARNW6DZ5K4OYSLFNEA2CSMUM2UEF4&source_account=GBAC4BTW6UIJOCCUOZ7QATQPVWX6UQVH3ESQ6NEHBMCXJ3MVP4GMT77H";

        assert_eq!(expected_parameters, request.get_query_parameters());

        let url = "base_url";
        assert_eq!(
            format!("{}/{}{}", url, PATHS_PATH, request.get_query_parameters()),
            request.build_url(url)
        );

        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        let response = horizon_client.get_find_payment_paths(&request).await;

        assert!(response.clone().is_ok());

        // Test creating and sending a request with source account.
        let request = FindPaymentsPathRequest::new()
            .set_destination_asset(AssetType::Native)
            .unwrap()
            .set_destination_amount("100".to_string())
            .unwrap()
            .set_source_account(
                "GCDE6MVFIOYF7YZCSVA6V7MDCFTNWMIOF5PQU3DWPH27AHNX4ERY6AKS".to_string(),
            )
            .unwrap();

        let expected_parameters: &str =
            "?destination_asset_type=native&destination_amount=100&source_account=GCDE6MVFIOYF7YZCSVA6V7MDCFTNWMIOF5PQU3DWPH27AHNX4ERY6AKS";
        assert_eq!(request.get_query_parameters(), expected_parameters);

        let url = "base_url";
        assert_eq!(
            format!("{}/{}{}", url, PATHS_PATH, request.get_query_parameters()),
            request.build_url(url)
        );

        let response = horizon_client.get_find_payment_paths(&request).await;

        const SOURCE_ASSET_TYPE: &str = "native";
        const SOURCE_AMOUNT: &str = "100.0000000";
        const DESTINATION_ASSET_TYPE: &str = "native";
        const DESTINATION_AMOUNT: &str = "100.0000000";

        assert!(response.clone().is_ok());
        let binding = response.unwrap();
        let response = &binding.embedded().records()[0];
        assert_eq!(response.source_asset_type(), SOURCE_ASSET_TYPE);
        assert_eq!(response.source_amount(), SOURCE_AMOUNT);
        assert_eq!(response.destination_asset_type(), DESTINATION_ASSET_TYPE);
        assert_eq!(response.destination_amount(), DESTINATION_AMOUNT);

        // Test creating a request with an invalid source account ID.
        let request = FindPaymentsPathRequest::new()
            .set_destination_asset(AssetType::Native)
            .unwrap()
            .set_destination_amount("42".to_string())
            .unwrap()
            .set_source_account("invalid_account_id".to_string());
        assert_eq!(
            request.err().unwrap(),
            "Public key must be 56 characters long"
        );
    }

    #[tokio::test]
    async fn test_list_strict_receive_payment_paths_request() {
        use crate::paths::{Asset, PATHS_PATH, PATHS_STRICT_RECEIVE_PATH};

        // Test creating and sending a request with source assets. Only the response status will be checked, as the request will not yield comparable data.
        let request = ListStrictReceivePaymentPathsRequest::new()
            .set_destination_asset(AssetType::CreditAlphanum4(Asset {
                asset_code: "USDC".to_string(),
                issuer_account_id: "GBJJ5OCBXNZWHSJJ4YQ6ECK24MBJSZMLEMINHKGGEWUA5RU2EDMPN6MS"
                    .to_string(),
            }))
            .unwrap()
            .set_destination_amount("42".to_string())
            .unwrap()
            .set_source(Source::SourceAssets(vec![
                IssuedOrNative::Native,
                IssuedOrNative::Native,
                IssuedOrNative::Issued(Asset {
                    asset_code: "USDC".to_string(),
                    issuer_account_id: "GBAKINTNEGR7PO6Z6XW2S5ITT5VARNW6DZ5K4OYSLFNEA2CSMUM2UEF4"
                        .to_string(),
                }),
            ]))
            .unwrap();

        let expected_parameters: &str =
            "?destination_asset_type=credit_alphanum4&destination_asset_issuer=GBJJ5OCBXNZWHSJJ4YQ6ECK24MBJSZMLEMINHKGGEWUA5RU2EDMPN6MS&destination_asset_code=USDC&destination_amount=42&source_assets=native%2Cnative%2CUSDC%3AGBAKINTNEGR7PO6Z6XW2S5ITT5VARNW6DZ5K4OYSLFNEA2CSMUM2UEF4";

        assert_eq!(request.get_query_parameters(), expected_parameters);

        let url = "base_url";
        assert_eq!(
            format!(
                "{}/{}/{}{}",
                url,
                PATHS_PATH,
                PATHS_STRICT_RECEIVE_PATH,
                request.get_query_parameters()
            ),
            request.build_url(url)
        );

        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        let response = horizon_client
            .get_list_strict_receive_payment_paths(&request)
            .await;

        assert!(response.clone().is_ok());

        // Test creating and sending a request with destination account.
        let request = ListStrictReceivePaymentPathsRequest::new()
            .set_destination_asset(AssetType::Native)
            .unwrap()
            .set_destination_amount("100".to_string())
            .unwrap()
            .set_source(Source::SourceAccount(
                "GCDE6MVFIOYF7YZCSVA6V7MDCFTNWMIOF5PQU3DWPH27AHNX4ERY6AKS".to_string(),
            ))
            .unwrap();

        let expected_parameters: &str =
            "?destination_asset_type=native&destination_amount=100&source_account=GCDE6MVFIOYF7YZCSVA6V7MDCFTNWMIOF5PQU3DWPH27AHNX4ERY6AKS";
        assert_eq!(request.get_query_parameters(), expected_parameters);

        let url = "base_url";
        assert_eq!(
            format!(
                "{}/{}/{}{}",
                url,
                PATHS_PATH,
                PATHS_STRICT_RECEIVE_PATH,
                request.get_query_parameters()
            ),
            request.build_url(url)
        );

        let response = horizon_client
            .get_list_strict_receive_payment_paths(&request)
            .await;

        const SOURCE_ASSET_TYPE: &str = "native";
        const SOURCE_AMOUNT: &str = "100.0000000";
        const DESTINATION_ASSET_TYPE: &str = "native";
        const DESTINATION_AMOUNT: &str = "100.0000000";

        assert!(response.clone().is_ok());
        let binding = response.unwrap();
        let response = &binding.embedded().records()[0];
        assert_eq!(response.source_asset_type(), SOURCE_ASSET_TYPE);
        assert_eq!(response.source_amount(), SOURCE_AMOUNT);
        assert_eq!(response.destination_asset_type(), DESTINATION_ASSET_TYPE);
        assert_eq!(response.destination_amount(), DESTINATION_AMOUNT);

        // Test creating a request with an empty source assets vector.
        let request = ListStrictReceivePaymentPathsRequest::new()
            .set_destination_asset(AssetType::Native)
            .unwrap()
            .set_destination_amount("42".to_string())
            .unwrap()
            .set_source(Source::SourceAssets(Vec::new()));
        assert_eq!(request.err().unwrap(), "SourceAssets cannot be empty");

        // Test creating a request with an invalid asset source account ID.
        let request = ListStrictReceivePaymentPathsRequest::new()
            .set_destination_asset(AssetType::Native)
            .unwrap()
            .set_destination_amount("42".to_string())
            .unwrap()
            .set_source(Source::SourceAccount("invalid_account_id".to_string()));
        assert_eq!(
            request.err().unwrap(),
            "Public key must be 56 characters long"
        );

        // Test creating a request with an invalid source account ID.
        let request = ListStrictReceivePaymentPathsRequest::new()
            .set_destination_asset(AssetType::Native)
            .unwrap()
            .set_destination_amount("42".to_string())
            .unwrap()
            .set_source(Source::SourceAssets(vec![IssuedOrNative::Native]))
            .unwrap()
            .set_destination_account("invalid_account_id");
        assert_eq!(
            request.err().unwrap(),
            "Public key must be 56 characters long"
        );
    }

    #[tokio::test]
    async fn test_list_strict_send_payment_paths_request() {
        use crate::paths::{Asset, PATHS_PATH, PATHS_STRICT_SEND_PATH};

        // Test creating and sending a request with destination assets. Only the response status will be checked, as the request will not yield comparable data.
        let request = ListStrictSendPaymentPathsRequest::new()
            .set_source_asset(AssetType::CreditAlphanum4(Asset {
                asset_code: "USDC".to_string(),
                issuer_account_id: "GBJJ5OCBXNZWHSJJ4YQ6ECK24MBJSZMLEMINHKGGEWUA5RU2EDMPN6MS"
                    .to_string(),
            }))
            .unwrap()
            .set_source_amount("42".to_string())
            .unwrap()
            .set_destination(Destination::DestinationAssets(vec![
                IssuedOrNative::Native,
                IssuedOrNative::Native,
                IssuedOrNative::Issued(Asset {
                    asset_code: "USDC".to_string(),
                    issuer_account_id: "GBAKINTNEGR7PO6Z6XW2S5ITT5VARNW6DZ5K4OYSLFNEA2CSMUM2UEF4"
                        .to_string(),
                }),
            ]))
            .unwrap();

        let expected_parameters: &str =
            "?source_amount=42&destination_assets=native%2Cnative%2CUSDC%3AGBAKINTNEGR7PO6Z6XW2S5ITT5VARNW6DZ5K4OYSLFNEA2CSMUM2UEF4&source_asset_type=credit_alphanum4&source_asset_issuer=GBJJ5OCBXNZWHSJJ4YQ6ECK24MBJSZMLEMINHKGGEWUA5RU2EDMPN6MS&source_asset_code=USDC";

        assert_eq!(request.get_query_parameters(), expected_parameters);

        let url = "base_url";
        assert_eq!(
            format!(
                "{}/{}/{}{}",
                url,
                PATHS_PATH,
                PATHS_STRICT_SEND_PATH,
                request.get_query_parameters()
            ),
            request.build_url(url)
        );

        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        let response = horizon_client
            .get_list_strict_send_payment_paths(&request)
            .await;

        assert!(response.clone().is_ok());

        // Test creating and sending a request with destination account.
        let request = ListStrictSendPaymentPathsRequest::new()
            .set_source_asset(AssetType::Native)
            .unwrap()
            .set_source_amount("100".to_string())
            .unwrap()
            .set_destination(Destination::DestinationAccount(
                "GBAKINTNEGR7PO6Z6XW2S5ITT5VARNW6DZ5K4OYSLFNEA2CSMUM2UEF4".to_string(),
            ))
            .unwrap();

        let expected_parameters: &str =
            "?source_amount=100&destination_account=GBAKINTNEGR7PO6Z6XW2S5ITT5VARNW6DZ5K4OYSLFNEA2CSMUM2UEF4&source_asset_type=native";

        assert_eq!(request.get_query_parameters(), expected_parameters);

        let url = "base_url";
        assert_eq!(
            format!(
                "{}/{}/{}{}",
                url,
                PATHS_PATH,
                PATHS_STRICT_SEND_PATH,
                request.get_query_parameters()
            ),
            request.build_url(url)
        );

        let response = horizon_client
            .get_list_strict_send_payment_paths(&request)
            .await;

        const SOURCE_ASSET_TYPE: &str = "native";
        const SOURCE_AMOUNT: &str = "100.0000000";
        const DESTINATION_ASSET_TYPE: &str = "native";
        const DESTINATION_AMOUNT: &str = "100.0000000";

        assert!(response.clone().is_ok());
        let binding = response.unwrap();
        let response = &binding.embedded().records()[0];
        assert_eq!(response.source_asset_type(), SOURCE_ASSET_TYPE);
        assert_eq!(response.source_amount(), SOURCE_AMOUNT);
        assert_eq!(response.destination_asset_type(), DESTINATION_ASSET_TYPE);
        assert_eq!(response.destination_amount(), DESTINATION_AMOUNT);

        // Test creating a request with an empty destination assets vector.
        let request = ListStrictSendPaymentPathsRequest::new()
            .set_source_asset(AssetType::Native)
            .unwrap()
            .set_source_amount("42".to_string())
            .unwrap()
            .set_destination(Destination::DestinationAssets(Vec::new()));
        assert_eq!(request.err().unwrap(), "DestinationAssets cannot be empty");

        // Test creating a request with an invalid destination asset account ID.
        let request = ListStrictSendPaymentPathsRequest::new()
            .set_source_asset(AssetType::Native)
            .unwrap()
            .set_source_amount("42".to_string())
            .unwrap()
            .set_destination(Destination::DestinationAccount(
                "invalid_account_id".to_string(),
            ));
        assert_eq!(
            request.err().unwrap(),
            "Public key must be 56 characters long"
        );

        // Test creating a request with an invalid destination account ID.
        let request = ListStrictSendPaymentPathsRequest::new()
            .set_source_asset(AssetType::Native)
            .unwrap()
            .set_source_amount("42".to_string())
            .unwrap()
            .set_destination(Destination::DestinationAccount(
                "invalid_account_id".to_string(),
            ));
        assert_eq!(
            request.err().unwrap(),
            "Public key must be 56 characters long"
        );
    }
}
