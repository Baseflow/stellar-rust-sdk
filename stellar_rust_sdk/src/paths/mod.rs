/// Provides the `FindPaymentPathsRequest`.
///
/// # Usage
/// This module provides the `FindPaymentPathsRequest` struct, specifically designed for
/// constructing requests to find payment paths based on certain criteria. It is tailored for
/// use with the [`PaymentClient::find_payment_paths`](crate::payment_client::PaymentClient::find_payment_paths)
/// method.
///
pub mod find_payment_paths_request;

/// Provides the `ListStrictReceivePaymentPathsRequest`.
///
/// # Usage
/// This module provides the `ListStrictReceivePaymentPathsRequest` struct, specifically designed for
/// constructing requests to list strict receive payment paths. It is tailored for
/// use with the [`PaymentClient::list_strict_receive_payment_paths`](crate::payment_client::PaymentClient::list_strict_receive_payment_paths)
/// method.
///
pub mod list_strict_receive_payment_paths_request;

/// Provides the `ListStrictSendPaymentPathsRequest`.
///
/// # Usage
/// This module provides the `ListStrictSendPaymentPathsRequest` struct, specifically designed for
/// constructing requests to list strict send payment paths. It is tailored for
/// use with the [`PaymentClient::list_strict_send_payment_paths`](crate::payment_client::PaymentClient::list_strict_send_payment_paths)
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
pub mod response;

/// The base paths for offer-related endpoints in the Horizon API.
///
/// # Usage
/// This variable is intended to be used internally by the request-building logic
/// to ensure consistent and accurate path construction for offer-related API calls.
///
pub(crate) static PATHS_PATH: &str = "paths"; // the base API path
pub(crate) static PATHS_STRICT_RECEIVE_PATH: &str = "strict-receive";
pub(crate) static PATHS_STRICT_SEND_PATH: &str = "strict-send";

/// Represents the destination asset for a payment path request.
#[derive(Default, Clone, Debug)]
pub struct DestinationAsset(AssetType);

/// Represents the absence of a destination asset for a payment path request.
#[derive(Default, Clone, Debug)]
pub struct NoDestinationAsset;

/// Represents the destination amount for a payment path request.
#[derive(Default, Clone, Debug)]
pub struct DestinationAmount(String);

/// Represents the absence of a destination amount for a payment path request.
#[derive(Default, Clone, Debug)]
pub struct NoDestinationAmount;

/// Represents the source account for a payment path request.
#[derive(Default, Clone, Debug)]
pub struct SourceAccount(String);

/// Represents the absence of a source account for a payment path request.
#[derive(Default, Clone, Debug)]
pub struct NoSourceAccount;

/// Represents different types of assets for payment path requests.
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

/// Represents a source asset for a payment path request.
#[derive(Default, Clone, Debug)]
pub struct SourceAsset(pub IssuedOrNative);

/// Represents whether the source asset is native or issued.
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
}

#[cfg(test)]
mod tests {
    use super::prelude::{
        FindPaymentsPathRequest, ListStrictReceivePaymentPathsRequest,
        ListStrictSendPaymentPathsRequest,
    };
    use super::{AssetType, IssuedOrNative, SourceAsset};
    use crate::models::Request;

    #[test]
    fn test_find_payment_paths_request() {
        use crate::paths::{Asset, PATHS_PATH};
        const TARGET_PARAMETERS: &str =
            "?destination_asset_type=credit_alphanum4&destination_asset_code=USDC&destination_asset_issuer=GBJJ5OCBXNZWHSJJ4YQ6ECK24MBJSZMLEMINHKGGEWUA5RU2EDMPN6MS&destination_amount=42&destination_account=GBAKINTNEGR7PO6Z6XW2S5ITT5VARNW6DZ5K4OYSLFNEA2CSMUM2UEF4&source_account=GBAC4BTW6UIJOCCUOZ7QATQPVWX6UQVH3ESQ6NEHBMCXJ3MVP4GMT77H";

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

        // Test the construction of the parameters string.
        assert_eq!(TARGET_PARAMETERS, request.get_query_parameters());

        // Test the construction of the url.
        let url = "base_url";
        assert_eq!(
            format!("{}/{}{}", url, PATHS_PATH, request.get_query_parameters()),
            request.build_url(url)
        );
    }

    #[test]
    fn test_list_strict_receive_payment_paths_request() {
        use crate::paths::{Asset, PATHS_PATH, PATHS_STRICT_RECEIVE_PATH};

        // Create a base request without optional source assets. This request should be valid on its own, and is used to test several combinations of source assets.
        let request_base = ListStrictReceivePaymentPathsRequest::new()
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

        // Test base request.
        let expected_parameters: &str =
            "?destination_asset_type=credit_alphanum4&destination_asset_issuer=GBJJ5OCBXNZWHSJJ4YQ6ECK24MBJSZMLEMINHKGGEWUA5RU2EDMPN6MS&destination_asset_code=USDC&destination_amount=42&destination_account=GBAKINTNEGR7PO6Z6XW2S5ITT5VARNW6DZ5K4OYSLFNEA2CSMUM2UEF4&source_account=GBAC4BTW6UIJOCCUOZ7QATQPVWX6UQVH3ESQ6NEHBMCXJ3MVP4GMT77H";
        assert_eq!(expected_parameters, request_base.get_query_parameters());

        let url = "base_url";
        assert_eq!(
            format!(
                "{}/{}/{}{}",
                url,
                PATHS_PATH,
                PATHS_STRICT_RECEIVE_PATH,
                request_base.get_query_parameters()
            ),
            request_base.build_url(url)
        );

        // Test base request with source assets, the first asset being of the `Native` type.
        let expected_parameters: &str =
        "?destination_asset_type=credit_alphanum4&destination_asset_issuer=GBJJ5OCBXNZWHSJJ4YQ6ECK24MBJSZMLEMINHKGGEWUA5RU2EDMPN6MS&destination_asset_code=USDC&destination_amount=42&destination_account=GBAKINTNEGR7PO6Z6XW2S5ITT5VARNW6DZ5K4OYSLFNEA2CSMUM2UEF4&source_account=GBAC4BTW6UIJOCCUOZ7QATQPVWX6UQVH3ESQ6NEHBMCXJ3MVP4GMT77H&source_assets=native%2Cnative%2CUSDC%3AGBAKINTNEGR7PO6Z6XW2S5ITT5VARNW6DZ5K4OYSLFNEA2CSMUM2UEF4";

        let test1 = request_base
            .clone()
            .set_source_assets(vec![
                SourceAsset(IssuedOrNative::Native),
                SourceAsset(IssuedOrNative::Native),
                SourceAsset(IssuedOrNative::Issued(Asset {
                    asset_code: "USDC".to_string(),
                    issuer_account_id: "GBAKINTNEGR7PO6Z6XW2S5ITT5VARNW6DZ5K4OYSLFNEA2CSMUM2UEF4"
                        .to_string(),
                })),
            ])
            .unwrap();

        assert_eq!(expected_parameters, test1.get_query_parameters());

        let url = "base_url";
        assert_eq!(
            format!(
                "{}/{}/{}{}",
                url,
                PATHS_PATH,
                PATHS_STRICT_RECEIVE_PATH,
                test1.get_query_parameters()
            ),
            test1.build_url(url)
        );

        // Test base request with source assets, the first asset being of the `Issued` type.
        let expected_parameters: &str =
        "?destination_asset_type=credit_alphanum4&destination_asset_issuer=GBJJ5OCBXNZWHSJJ4YQ6ECK24MBJSZMLEMINHKGGEWUA5RU2EDMPN6MS&destination_asset_code=USDC&destination_amount=42&destination_account=GBAKINTNEGR7PO6Z6XW2S5ITT5VARNW6DZ5K4OYSLFNEA2CSMUM2UEF4&source_account=GBAC4BTW6UIJOCCUOZ7QATQPVWX6UQVH3ESQ6NEHBMCXJ3MVP4GMT77H&source_assets=USDC%3AGBAKINTNEGR7PO6Z6XW2S5ITT5VARNW6DZ5K4OYSLFNEA2CSMUM2UEF4%2Cnative%2Cnative";

        let test1 = request_base
            .clone()
            .set_source_assets(vec![
                SourceAsset(IssuedOrNative::Issued(Asset {
                    asset_code: "USDC".to_string(),
                    issuer_account_id: "GBAKINTNEGR7PO6Z6XW2S5ITT5VARNW6DZ5K4OYSLFNEA2CSMUM2UEF4"
                        .to_string(),
                })),
                SourceAsset(IssuedOrNative::Native),
                SourceAsset(IssuedOrNative::Native),
            ])
            .unwrap();

        assert_eq!(expected_parameters, test1.get_query_parameters());

        let url = "base_url";
        assert_eq!(
            format!(
                "{}/{}/{}{}",
                url,
                PATHS_PATH,
                PATHS_STRICT_RECEIVE_PATH,
                test1.get_query_parameters()
            ),
            test1.build_url(url)
        );
    }

    #[test]
    fn test_list_strict_send_payment_paths_request() {
        use crate::paths::{Asset, PATHS_PATH, PATHS_STRICT_SEND_PATH};

        // Create a base request without optional source assets. This request should be valid on its own, and is used to test several combinations of source assets.
        let request_base = ListStrictSendPaymentPathsRequest::new()
            .set_destination_asset(AssetType::CreditAlphanum4(Asset {
                asset_code: "USDC".to_string(),
                issuer_account_id: "GBJJ5OCBXNZWHSJJ4YQ6ECK24MBJSZMLEMINHKGGEWUA5RU2EDMPN6MS"
                    .to_string(),
            }))
            .unwrap()
            .set_destination_amount("42".to_string())
            .unwrap()
            .set_destination_account(
                "GBAKINTNEGR7PO6Z6XW2S5ITT5VARNW6DZ5K4OYSLFNEA2CSMUM2UEF4".to_string(),
            )
            .unwrap();

        // Test base request.
        let expected_parameters: &str =
            "?destination_asset_type=credit_alphanum4&destination_asset_issuer=GBJJ5OCBXNZWHSJJ4YQ6ECK24MBJSZMLEMINHKGGEWUA5RU2EDMPN6MS&destination_asset_code=USDC&destination_amount=42&destination_account=GBAKINTNEGR7PO6Z6XW2S5ITT5VARNW6DZ5K4OYSLFNEA2CSMUM2UEF4";
        assert_eq!(expected_parameters, request_base.get_query_parameters());

        let url = "base_url";
        assert_eq!(
            format!(
                "{}/{}/{}{}",
                url,
                PATHS_PATH,
                PATHS_STRICT_SEND_PATH,
                request_base.get_query_parameters()
            ),
            request_base.build_url(url)
        );

        // Test base request with source assets, the first asset being of the `Native` type.
        let expected_parameters: &str =
        "?destination_asset_type=credit_alphanum4&destination_asset_issuer=GBJJ5OCBXNZWHSJJ4YQ6ECK24MBJSZMLEMINHKGGEWUA5RU2EDMPN6MS&destination_asset_code=USDC&destination_amount=42&destination_account=GBAKINTNEGR7PO6Z6XW2S5ITT5VARNW6DZ5K4OYSLFNEA2CSMUM2UEF4&source_assets=native%2Cnative%2CUSDC%3AGBAKINTNEGR7PO6Z6XW2S5ITT5VARNW6DZ5K4OYSLFNEA2CSMUM2UEF4";

        let test1 = request_base
            .clone()
            .set_source_assets(vec![
                SourceAsset(IssuedOrNative::Native),
                SourceAsset(IssuedOrNative::Native),
                SourceAsset(IssuedOrNative::Issued(Asset {
                    asset_code: "USDC".to_string(),
                    issuer_account_id: "GBAKINTNEGR7PO6Z6XW2S5ITT5VARNW6DZ5K4OYSLFNEA2CSMUM2UEF4"
                        .to_string(),
                })),
            ])
            .unwrap();

        assert_eq!(expected_parameters, test1.get_query_parameters());

        let url = "base_url";
        assert_eq!(
            format!(
                "{}/{}/{}{}",
                url,
                PATHS_PATH,
                PATHS_STRICT_SEND_PATH,
                test1.get_query_parameters()
            ),
            test1.build_url(url)
        );

        // Test base request with source assets, the first asset being of the `Issued` type.
        let expected_parameters: &str =
        "?destination_asset_type=credit_alphanum4&destination_asset_issuer=GBJJ5OCBXNZWHSJJ4YQ6ECK24MBJSZMLEMINHKGGEWUA5RU2EDMPN6MS&destination_asset_code=USDC&destination_amount=42&destination_account=GBAKINTNEGR7PO6Z6XW2S5ITT5VARNW6DZ5K4OYSLFNEA2CSMUM2UEF4&source_assets=USDC%3AGBAKINTNEGR7PO6Z6XW2S5ITT5VARNW6DZ5K4OYSLFNEA2CSMUM2UEF4%2Cnative%2Cnative";

        let test1 = request_base
            .clone()
            .set_source_assets(vec![
                SourceAsset(IssuedOrNative::Issued(Asset {
                    asset_code: "USDC".to_string(),
                    issuer_account_id: "GBAKINTNEGR7PO6Z6XW2S5ITT5VARNW6DZ5K4OYSLFNEA2CSMUM2UEF4"
                        .to_string(),
                })),
                SourceAsset(IssuedOrNative::Native),
                SourceAsset(IssuedOrNative::Native),
            ])
            .unwrap();

        assert_eq!(expected_parameters, test1.get_query_parameters());

        let url = "base_url";
        assert_eq!(
            format!(
                "{}/{}/{}{}",
                url,
                PATHS_PATH,
                PATHS_STRICT_SEND_PATH,
                test1.get_query_parameters()
            ),
            test1.build_url(url)
        );
    }
}
