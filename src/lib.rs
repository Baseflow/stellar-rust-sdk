mod assets;
mod accounts;
mod ledgers;
mod claimable_balances;
mod horizon_client;
mod models;
mod xdr;

/// The asset type
/// Native - The native asset
/// Issued - An issued asset
/// [AccountsRequest](struct.AccountsRequest.html)
pub enum AssetType {
    Native,
    Issued,
}

impl std::fmt::Display for AssetType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AssetType::Native => write!(f, "native"),
            AssetType::Issued => write!(f, "issued"),
        }
    }
}

/// The order of the records
/// Asc - Ascending order
/// Desc - Descending order
/// [AccountsRequest](struct.AccountsRequest.html)
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