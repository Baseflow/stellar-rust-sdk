use crate::models::prelude::Embedded;
use crate::models::Response;

pub struct PathsResponse {
    embedded: Embedded<Path>
}

pub struct Path {
    source_asset_type: String,
    source_asset_code: String,
    source_asset_issuer: String,
    source_amount: String,
    destination_asset_type: String,
    destination_asset_code: String,
    destination_asset_issuer: String,
    destination_amount: String,
    assets: Vec<Asset>,
}

pub struct Asset {
    asset_type: String,
    asset_code: Option<String>,
    asset_issuer: Option<String>,
}

impl Response for PathsResponse {
    fn from_json(json: String) -> Result<Self, String> {
        let embedded = Embedded::from_json(json)?;
        Ok(PathsResponse { embedded })
    }
}