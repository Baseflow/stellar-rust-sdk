/// The request module contains the request object
/// which is used to send requests to the server
pub trait Request {
    /// Creates a new request object
    fn new() -> Self;

    /// Gets the relative URL for the request
    /// # Arguments
    /// * `self` - The request object
    /// # Returns
    /// The relative URL for the request
    fn get_path(&self) -> &str;

    /// Gets the query parameters for the request
    /// # Arguments
    /// * `self` - The request object
    /// # Returns
    /// The query parameters for the request
    fn get_query_parameters(&self) -> String;

    /// Validate the request
    /// Returns an error if the request is invalid
    /// Returns Ok(()) if the request is valid
    /// This method is called before the request is sent
    /// to the server
    fn validate(&self) -> Result<(), String>;

    /// Build the URL for the request
    /// # Arguments
    /// * `self` - The request object
    /// * `base_url` - The base URL for the Horizon server
    /// # Returns
    /// The URL for the request
    fn build_url(&self, base_url: &str) -> String;
}

/// is_public_key validates a public key
/// # Arguments
/// * `public_key` - The public key to validate
/// # Returns
/// Ok(()) if the public key is valid
/// Err(String) if the public key is invalid
pub fn is_public_key(public_key: &str) -> Result<(), String> {
    if public_key.len() != 56 {
        return Err("Public key must be 56 characters long".to_string());
    }

    if !public_key.starts_with("G") {
        return Err("Public key must start with G".to_string());
    }

    Ok(())
}

// TODO: All responses are wrapped in a pagination object
// {
//   "_links": {
//     "self": {
//       "href": "https://horizon-testnet.stellar.org/accounts/?cursor=&limit=10&order=asc&sponsor=GAVCBYUQSQA77EOOQMSDDXE6VSWDZRGOZOGMLWGFR6YR4TR243VWBDFO"
//     },
//     "next": {
//       "href": "https://horizon-testnet.stellar.org/accounts/?cursor=&limit=10&order=asc&sponsor=GAVCBYUQSQA77EOOQMSDDXE6VSWDZRGOZOGMLWGFR6YR4TR243VWBDFO"
//     },
//     "prev": {
//       "href": "https://horizon-testnet.stellar.org/accounts/?cursor=&limit=10&order=desc&sponsor=GAVCBYUQSQA77EOOQMSDDXE6VSWDZRGOZOGMLWGFR6YR4TR243VWBDFO"
//     }
//   },
//   "_embedded": {
//     "records": []
//   }
// }
// The records are the actual response type.
// We could deserialize to this Response<T> type

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
}
