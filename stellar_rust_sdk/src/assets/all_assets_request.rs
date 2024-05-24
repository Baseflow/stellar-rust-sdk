use crate::{models::*, BuildQueryParametersExt, Paginatable};
use stellar_rust_sdk_derive::Pagination;

/// Represents a request for listing all assets in the Stellar Horizon API.
///
/// This structure allows for specifying various parameters to filter and paginate the list of all assets
/// known to the Stellar network.
/// More details can be found in the Horizon API documentation on
/// <a href="https://developers.stellar.org/api/horizon/resources/list-all-assets">Assets</a>.
///
/// # Usage
///
/// To use `AllAssetsRequest`, create an instance and set any desired filters. Then pass it to
/// `HorizonClient::get_all_assets` to execute the query.
///
/// # Example
/// ```
/// # use stellar_rs::assets::prelude::{AllAssetsRequest, AllAssetsResponse};
/// # use stellar_rs::models::*;
/// # use stellar_rs::horizon_client::HorizonClient;
/// # use stellar_rust_sdk_derive::Pagination;
/// # use stellar_rs::Paginatable;
/// #
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let base_url = "https://horizon-testnet.stellar.org".to_string();
/// # let horizon_client = HorizonClient::new(base_url)
/// #    .expect("Failed to create Horizon Client");
/// #
/// let request = AllAssetsRequest::default()
///     .set_asset_code("USD")?
///     .set_asset_issuer("GAXLYH...")?
///     .set_limit(20)?
///     .set_order(Order::Desc);
///
/// let response = horizon_client.get_all_assets(&request.unwrap()).await;
/// # Ok({})
/// # }
///
/// ```
///
#[derive(Default, Pagination)]
pub struct AllAssetsRequest {
    /// The code of the asset to filter by. This is typically the identifier
    ///   assigned to custom assets on the Stellar network.
    asset_code: Option<String>,

    /// The Stellar address of the issuer for the asset you want to filter by.
    ///   It is relevant for assets that are custom issued on the Stellar network.
    asset_issuer: Option<String>,

    /// A pointer to a specific location in a collection of responses, derived from the
    ///   `paging_token` value of a record. Used for pagination control in the API response.
    cursor: Option<u32>,

    /// Specifies the maximum number of records to be returned in a single response.
    ///   The range for this parameter is from 1 to 200. The default value is set to 10.
    limit: Option<u8>,

    /// Determines the [`Order`] of the records in the response. Valid options are [`Order::Asc`] (ascending)
    ///   and [`Order::Desc`] (descending). If not specified, it defaults to ascending.
    order: Option<Order>,
}

impl Request for AllAssetsRequest {
    fn get_query_parameters(&self) -> String {
        vec![
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
            self.asset_code
                .as_ref()
                .map(|ac| format!("asset_code={}", ac)),
            self.asset_issuer
                .as_ref()
                .map(|ac| format!("asset_issuer={}", ac)),
        ]
        .build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}{}",
            base_url,
            super::ASSET_PATH,
            self.get_query_parameters()
        )
    }
}

impl AllAssetsRequest {
    /// Creates a new `AllAssetsRequest` with default parameters.
    pub fn new() -> AllAssetsRequest {
        AllAssetsRequest::default()
    }

    /// Sets the asset code filter for the `AllAssetsRequest`.
    ///
    /// This method specifies the code of the asset to filter by in the assets query. The asset code
    /// refers to the identifier assigned to assets on the Stellar network.
    ///
    /// # Arguments
    /// * `asset_code` - A string slice representing the asset code. The asset code must be 12 characters
    ///   or fewer in length. It typically corresponds to custom asset identifiers on the Stellar network.
    ///
    pub fn set_asset_code(self, asset_code: &str) -> Result<AllAssetsRequest, String> {
        if asset_code.len() > 12 {
            return Err("asset_code must be 12 characters or less".to_string());
        }

        Ok(AllAssetsRequest {
            asset_code: Some(asset_code.to_string()),
            ..self
        })
    }

    /// Sets the asset issuer filter for the `AllAssetsRequest`.
    ///
    /// This method specifies the Stellar address of the issuer to filter by in the assets query.
    ///
    /// # Arguments
    /// * `asset_issuer` - A string slice representing the Stellar address of the asset issuer.
    ///   The address must be exactly 56 characters long, conforming to the standard Stellar public
    ///   key format.
    ///
    pub fn set_asset_issuer(self, asset_issuer: &str) -> Result<AllAssetsRequest, String> {
        if asset_issuer.len() != 56 {
            return Err("asset_issuer must be 56 characters".to_string());
        }

        Ok(AllAssetsRequest {
            asset_issuer: Some(asset_issuer.to_string()),
            ..self
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_asset_code_valid() {
        let request = AllAssetsRequest::new().set_asset_code("XLM").unwrap();
        assert_eq!(request.asset_code.unwrap(), "XLM");
    }

    #[test]
    fn test_set_asset_code_invalid() {
        let request = AllAssetsRequest::new().set_asset_code("XLM123456789012");
        assert_eq!(
            request.err().unwrap(),
            "asset_code must be 12 characters or less"
        );
    }

    #[test]
    fn test_set_asset_issuer_valid() {
        let request = AllAssetsRequest::new()
            .set_asset_issuer("Baseflow_TechnologyInnovationAndSoftwareDevelopment_2023")
            .unwrap();
        assert_eq!(
            request.asset_issuer.unwrap(),
            "Baseflow_TechnologyInnovationAndSoftwareDevelopment_2023"
        );
    }

    #[test]
    fn test_set_asset_issuer_invalid() {
        let request = AllAssetsRequest::new()
            .set_asset_issuer("BaseflowSoftwareDevelopmentPowerhouse_InnovativeSolutions2023");
        assert_eq!(
            request.err().unwrap(),
            "asset_issuer must be 56 characters".to_string()
        );
    }

    #[test]
    fn test_set_cursor_valid() {
        let request = AllAssetsRequest::new().set_cursor(12345).unwrap();
        assert_eq!(request.cursor.unwrap(), 12345);
    }

    #[test]
    fn test_set_cursor_invalid() {
        let request = AllAssetsRequest::new().set_cursor(0);
        assert_eq!(
            request.err().unwrap(),
            "Cursor must be greater than or equal to 1.".to_string()
        );
    }

    #[test]
    fn test_set_limit_valid() {
        let request = AllAssetsRequest::new().set_limit(20).unwrap();
        assert_eq!(request.limit.unwrap(), 20);
    }

    #[test]
    fn test_set_limit_invalid_low() {
        let request = AllAssetsRequest::new().set_limit(0);
        assert_eq!(
            request.err().unwrap(),
            "Limit must be between 1 and 200.".to_string()
        );
    }

    #[test]
    fn test_set_limit_invalid_high() {
        let request = AllAssetsRequest::new().set_limit(201);
        assert_eq!(
            request.err().unwrap(),
            "Limit must be between 1 and 200.".to_string()
        );
    }
}
