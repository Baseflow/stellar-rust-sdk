/// Provides the `AllAssetsRequest` struct.
///
/// This module contains the `AllAssetsRequest` struct, which is designed to create requests
/// for querying comprehensive lists of assets from the Horizon server. It facilitates specifying
/// various parameters to tailor the asset query, such as asset issuer, asset code, and pagination options.
///
/// The `AllAssetsRequest` struct is meant to be used in conjunction with the [`HorizonClient`](crate::horizon_client::HorizonClient)
/// to perform the actual API calls and fetch asset data. It adheres to the structure
/// and requirements of the Horizon API for asset queries.
///
pub mod all_assets_request;

/// Provides the `AllAssetsResponse`.
///
/// This module offers a set of structures representing the response received from the Horizon
/// API when querying for all assets. These structures are tailored to convert the JSON response
/// from the Horizon server into structured Rust objects, making it easier to handle and utilize
/// asset data within client applications.
/// For a detailed description of the response structure, refer to the
/// <a href="https://developers.stellar.org/api/horizon/resources/list-all-assets">List All Assets endpoint</a> documentation.
///
/// The provided structures include serialization and deserialization capabilities for dealing
/// with the JSON format of the Horizon server's response. They also leverage the `Getters` derive macro
/// to furnish convenient getter methods for accessing their fields.
///
pub mod response;

/// The base path for all assets endpoints in the Stellar Horizon API.
///
/// This static variable holds the string slice that represents the common base path used in constructing
/// URLs for asset-related queries to the Horizon server. It forms a constant part of the route for all
/// asset-related API endpoints, ensuring uniformity in URL construction across different parts of the SDK.
///
/// # Usage
/// This variable is intended to be used internally by the request-building logic
/// to ensure consistent and accurate path construction for asset-related API calls.
///
static ASSET_PATH: &str = "assets";

/// The `prelude` module of the `assets` module.
///
/// This module is designed as a convenience for users of the Stellar Horizon Rust SDK, facilitating
/// easy and ergonomic imports of commonly used items related to asset data. It re-exports essential
/// structs and traits from the sibling modules in `assets`, streamlining access to these components
/// when utilizing the SDK in client applications.
///
/// By importing from `prelude`, users gain immediate access to the primary functionalities of the
/// asset-related modules without the need for importing each item individually, simplifying code
/// and improving readability.
///
/// # Contents
///
/// The `prelude` includes the following re-exports:
///
/// * From `all_assets_request`: All items (e.g., `AllAssetsRequest`).
/// * From `all_assets_response`: All items (e.g., `AllAssetsResponse`, `Records`, etc.).
///
/// This approach allows for a more concise and focused usage pattern, especially beneficial
/// when dealing with multiple components related to asset data in the Horizon API.
///
/// # Example
/// ```
/// // Import the contents of the assets prelude
/// use stellar_rs::assets::prelude::*;
///
/// // This enables direct use of AllAssetsRequest, AllAssetsResponse, etc.
/// let asset_request = AllAssetsRequest::new();
/// // Further usage...
/// ```
///
pub mod prelude {
    pub use super::all_assets_request::*;
    pub use super::response::*;
}
