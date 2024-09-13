/// Contains the details of a non-native asset.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct AssetData {
    pub asset_code: String,
    pub asset_issuer: String,
}

/// Represents 1 of the asset types which can be specified in a request.
#[derive(Default, Clone, Debug)]
pub enum IssuedOrNative {
    #[default]
    Native,
    Issued(AssetData),
}

/// Represents 1 of the asset types which can be specified in a request.
/// This type has more specific options to choose from.
#[derive(Default, Clone, PartialEq, Debug)]
pub enum AssetType {
    /// A native asset_type type. It holds no value.
    #[default]
    Native,
    /// An alphanumeric 4 asset_type type. It holds an Asset struct with asset code and asset issuer.
    Alphanumeric4(AssetData),
    /// An alphanumeric 12 asset_type type. It holds an Asset struct with asset code and asset issuer.
    Alphanumeric12(AssetData),
}
