pub mod all_assets_request;
pub mod all_assets_response;

static ASSET_PATH: &str = "assets";

pub mod prelude {
    pub use super::all_assets_request::*;
    pub use super::all_assets_response::*;
}
