mod response_models;

pub mod prelude {
    pub use super::response_models::*;
    pub use super::Request;
    pub use super::Response;
}

/// Defines methods for creating HTTP requests to the Horizon server.
///
/// Implementors of this trait represent different types of requests that can be made to the server.
/// The trait provides methods for assembling the request's query parameters and building the
/// full URL for the request.
///
/// Implementors of this trait should provide the specific logic for these methods based on the
/// type of request they represent.
///
pub trait Request {
    /// Generates a query string from the request's parameters.
    ///
    /// This method is responsible for constructing the query part of a URL for an HTTP request.
    /// It processes the request's parameters and converts them into a properly formatted query string.
    /// The method should consider all relevant fields of the request and serialize them as necessary,
    /// adhering to the standard URL-encoded format (`key=value` pairs joined by `&`).
    ///
    /// # Returns
    /// Returns a `String` representing the query parameters of the request. If the request does not
    /// have any parameters, or if they are not applicable, this method may return an empty string.
    ///
    /// # Usage
    /// This method is typically used internally when building the full URL for the request, specifically
    /// in the `build_url` method. It abstracts the complexity of query string construction, ensuring a
    /// consistent and error-free process.
    ///
    /// Implementors should ensure that the query string is correctly encoded and formatted,
    /// particularly in cases where parameters include special characters or spaces.
    ///
    fn get_query_parameters(&self) -> String;

    /// Constructs the complete URL for the HTTP request.
    ///
    /// This method combines the base URL of the Horizon server with the query parameters specific
    /// to the request. It is responsible for assembling the full URL used to make the HTTP request
    /// to the server. The method should appropriately format the URL, ensuring that the base URL
    /// and query parameters are correctly concatenated.
    ///
    /// # Arguments
    /// * `base_url` - A string slice representing the base URL of the Horizon server. This URL
    ///   provides the foundational part of the request URL.
    ///
    /// # Returns
    /// Returns a `String` representing the full URL for the request. This URL includes the base
    /// URL and any query parameters, correctly formatted for use in an HTTP request.
    ///
    /// # Usage
    /// This method is typically called when an HTTP request is being prepared. The returned URL
    /// is used as the target for the request.
    ///
    /// Implementors of this method should ensure that the full URL is correctly structured,
    /// particularly in cases where the base URL has specific path components or the request
    /// includes complex query parameters.
    ///
    fn build_url(&self, base_url: &str) -> String;
}

/// Handles deserialization of HTTP responses from the Horizon server.
///
/// Types implementing this trait represent various kinds of responses that can be received
/// from the server. The primary responsibility of this trait is to provide a way to convert
/// a JSON string (the raw response from the server) into a Rust object.
///
/// Implementors of this trait are typically structs that mirror the JSON structure of responses
/// from the Horizon API, providing a type-safe way to interact with the response data.
///
pub trait Response: Sized {
    /// Deserializes a JSON string into a response object.
    ///
    /// This method is responsible for converting a JSON string, typically received as a response
    /// from the Horizon server, into an instance of the implementing type. The method must handle any
    /// inconsistencies or errors in the JSON format, returning a `Result` that indicates either
    /// successful deserialization or an error.
    ///
    /// # Arguments
    /// * `json` - A `String` containing the JSON data to be deserialized into the response object.
    ///
    /// # Returns
    /// Returns a `Result<Self, String>`, where `Self` is the type that implements the `Response` trait.
    /// On successful deserialization, it returns `Ok(Self)`, containing the constructed object. If the
    /// deserialization fails (due to invalid JSON format, missing fields, etc.), it returns an `Err(String)`,
    /// with an error message describing the issue.
    ///
    /// Implementors of this method should ensure that the deserialization logic is robust and can handle
    /// various edge cases, especially considering the diverse and complex nature of responses from the Horizon API.
    ///
    fn from_json(json: String) -> Result<Self, String>;
}

/// Validates the format of a Stellar public key.
///
/// This function checks whether the provided string is a valid Stellar public key. A valid
/// public key must be 56 characters in length and start with the letter 'G'.
///
/// # Arguments
/// * `public_key` - A string slice representing the public key to validate.
///
/// # Returns
/// * `Ok(())` if the public key meets the format criteria.
/// * `Err(String)` with an error message if the public key is invalid.
///
/// # Examples
/// ```
/// # use stellar_rs::models::is_public_key;
/// assert!(is_public_key("GAVCBYUQSQA77EOOQMSDDXE6VSWDZRGOZOGMLWGFR6YR4TR243VWBDFO").is_ok());
/// assert!(is_public_key("invalid_key").is_err());
/// ```
///
/// It is a utility function that can be used throughout the system where public key validation is necessary.

pub fn is_public_key(public_key: &str) -> Result<(), String> {
    if public_key.len() != 56 {
        return Err("Public key must be 56 characters long".to_string());
    }

    if !public_key.starts_with("G") {
        return Err("Public key must start with G".to_string());
    }

    Ok(())
}

/// Represents an issued asset. Contains both the asset code and the issuer account ID,
///   formatted as "asset_code:issuer_account_id".
#[derive(Default, Clone, Debug)]
pub struct IssuedAsset(String);

/// A marker type to represent the native asset (XLM) without additional data.
#[derive(Default, Clone, PartialEq, Debug)]
pub struct NativeAsset;

/// Represents the variants of assets in the Stellar network.
///
/// `Asset` is a generic struct used to specify the variant of an asset in Stellar-related requests,
/// allowing for differentiation between native assets and issued assets within the Stellar ecosystem.
/// It encapsulates the details of issued assets, including their code and issuer account ID.
///
/// # Examples
///
/// ```
/// use stellar_rs::models::Asset;
/// // Creating a native asset
/// let native_asset = Asset::new();
///
/// // Creating an issued asset with a valid asset code and issuer account ID
/// let issued_asset = native_asset.set_issued("USD", "GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7").unwrap();
/// ```
///
#[derive(Default, Clone, Debug)]
pub struct Asset<T> {
    asset: T,
}

impl Asset<NativeAsset> {
    /// Creates a new instance of `Asset` representing the native asset (XLM).
    pub fn new() -> Self {
        Asset::default()
    }

    /// Transforms the `Asset` into an `IssuedAsset` with a specified asset code and issuer account ID.
    ///
    /// # Arguments
    ///
    /// * `asset_code` - The asset code string slice, should not exceed 12 characters.
    /// * `issuer_account_id` - The Stellar public key string slice of the asset issuer, should be a valid public key.
    ///
    /// # Returns
    ///
    /// A result that, on success, contains the `Asset<IssuedAsset>` with the specified code and issuer.
    /// On failure, it contains an error message indicating the reason for failure.
    ///
    pub fn set_issued(
        self,
        asset_code: &str,
        issuer_account_id: &str,
    ) -> Result<Asset<IssuedAsset>, String> {
        if asset_code.len() > 12 {
            return Err("asset_code must be 12 characters or less".to_string());
        }

        if let Err(e) = is_public_key(&issuer_account_id) {
            return Err(e.to_string());
        }

        Ok(Asset {
            asset: IssuedAsset(format!("{}:{}", asset_code, issuer_account_id)),
        })
    }
}

impl std::fmt::Display for Asset<NativeAsset> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "native")
    }
}

impl std::fmt::Display for Asset<IssuedAsset> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.asset.0)
    }
}

/// Represents the ordering of records in queries to the Horizon API.
///
/// `Order` is an enumeration used in various requests to specify the desired order of the returned
/// records.
///
/// # Variants
/// * `Asc` - Indicates ascending order.
/// * `Desc` - Indicates descending order.
///
#[derive(PartialEq, Debug)]
pub enum Order {
    Asc,
    Desc,
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Order::Asc => write!(f, "asc"),
            Order::Desc => write!(f, "desc"),
        }
    }
}

pub enum IncludeFailed {
    True,
    False,
}

impl std::fmt::Display for IncludeFailed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            IncludeFailed::True => write!(f, "true"),
            IncludeFailed::False => write!(f, "false"),
        }
    }
}

impl std::fmt::Debug for IncludeFailed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            IncludeFailed::True => write!(f, "true"),
            IncludeFailed::False => write!(f, "false"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_public_key() {
        let result = is_public_key("GAVCBYUQSQA77EOOQMSDDXE6VSWDZRGOZOGMLWGFR6YR4TR243VWBDFO");
        assert!(result.is_ok());
        let result =
            is_public_key("G1234567890123456789012345678901234567890123456789012345678901");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Public key must be 56 characters long");
        let result = is_public_key("BAVCBYUQSQA77EOOQMSDDXE6VSWDZRGOZOGMLWGFR6YR4TR243VWBDFO");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Public key must start with G");
    }

    use stellar_xdr::curr::{LedgerHeader, LedgerHeaderExt, Limits, ReadXdr, StellarValueExt};

    // TODO, add vice versa.
    // https://developers.stellar.org/docs/encyclopedia/xdr#parsing-xdr
    // See if we can use an XDR generator to generate structs for us.
    // Possible solution: https://github.com/stellar/xdrgen
    #[test]
    fn decode_ledger_header() {
        // Decode online at : https://stellar.github.io/xdr-viewer/?type=LedgerHeader&network=public
        let encoded: &[u8] = "AAAAAGPZj1Nu5o0bJ7W4nyOvUxG3Vpok+vFAOtC1K2M7B76ZuZRHr9UdXKbTKiclfOjy72YZFJUkJPVcKT5htvorm1QAAAAAZImGNAAAAAAAAAABAAAAAKgkzRi8nXUGTSmaW1uspDvDqi8yaTgVPYwvm7XLbfAzAAAAQLuRQK8ocAjytwfQelkpwZQa5+jReIO0pbr/9BbUbIffRxQN4Z76J3qDIn5lSJpn0OkYL8ZLPGP0W/S1vlTj5w/fP2GYBKkv20BXGS3EPddI6neK3FK8SYzoBSTAFLgRGXNSJ+05hGEpEjdoewhEaqLJsJbgyYpGLa3aVp8F3SSEAAAAAg3gtrOnZAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABkBfXhAAAAAGQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=".as_bytes();

        let decoded = LedgerHeader::from_xdr_base64(encoded, Limits::none()).unwrap();

        assert_eq!(decoded.ledger_version, 0);
        assert_eq!(
            decoded.previous_ledger_hash.to_string(),
            "63d98f536ee68d1b27b5b89f23af5311b7569a24faf1403ad0b52b633b07be99"
        );
        assert_eq!(decoded.scp_value.upgrades.len(), 0);
        assert_eq!(
            decoded.scp_value.tx_set_hash.to_string(),
            "b99447afd51d5ca6d32a27257ce8f2ef661914952424f55c293e61b6fa2b9b54"
        );
        assert_eq!(decoded.scp_value.close_time.0, 1686734388);
        assert_eq!(
            decoded.tx_set_result_hash.to_string(),
            "df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119"
        );

        match decoded.scp_value.ext {
            StellarValueExt::Signed(signed) => {
                assert_eq!(
                    signed.node_id.0.discriminant().to_string(),
                    "PublicKeyTypeEd25519"
                );
                assert_eq!(signed.node_id.0.name().to_string(), "PublicKeyTypeEd25519");
                // todo check node-id public key
                // todo check signature
            }
            _ => panic!("Expected signed"),
        }

        assert_eq!(
            decoded.bucket_list_hash.to_string(),
            "735227ed398461291237687b08446aa2c9b096e0c98a462dadda569f05dd2484"
        );
        assert_eq!(decoded.ledger_seq, 2);
        assert_eq!(decoded.total_coins, 1000000000000000000);
        assert_eq!(decoded.fee_pool, 0);
        assert_eq!(decoded.inflation_seq, 0);
        assert_eq!(decoded.id_pool, 0);
        assert_eq!(decoded.base_fee, 100);
        assert_eq!(decoded.base_reserve, 100000000);
        assert_eq!(decoded.max_tx_set_size, 100);
        assert_eq!(decoded.ext, LedgerHeaderExt::V0);
        for decoded in decoded.skip_list {
            assert_eq!(
                decoded.to_string(),
                "0000000000000000000000000000000000000000000000000000000000000000"
            );
        }
    }

    #[test]
    fn test_new_native_asset() {
        let native_asset = Asset::<NativeAsset>::new();
        assert_eq!(native_asset.asset, NativeAsset);
    }

    #[test]
    fn test_set_issued_valid() {
        let native_asset = Asset::<NativeAsset>::new();
        let issued_asset = native_asset
            .set_issued(
                "USD",
                "GAVCBYUQSQA77EOOQMSDDXE6VSWDZRGOZOGMLWGFR6YR4TR243VWBDFO",
            )
            .unwrap();
        assert_eq!(
            issued_asset.asset.0,
            "USD:GAVCBYUQSQA77EOOQMSDDXE6VSWDZRGOZOGMLWGFR6YR4TR243VWBDFO"
        );
    }

    #[test]
    fn test_set_issued_invalid_asset_code() {
        let native_asset = Asset::<NativeAsset>::new();
        let result = native_asset.set_issued(
            "LONG_ASSET_CODE",
            "GAVCBYUQSQA77EOOQMSDDXE6VSWDZRGOZOGMLWGFR6YR4TR243VWBDFO",
        );
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "asset_code must be 12 characters or less"
        );
    }

    #[test]
    fn test_set_issued_invalid_public_key() {
        let native_asset = Asset::<NativeAsset>::new();
        let result = native_asset.set_issued("USD", "INVALID_PUBLIC_KEY");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Public key must be 56 characters long");
    }
}
