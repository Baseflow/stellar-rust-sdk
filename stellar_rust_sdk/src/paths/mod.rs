mod response;
mod find_payment_paths_request;
mod list_strict_receive_payment_paths_request;
mod list_strict_send_payment_paths_request;

pub(crate) static PATHS_STRICT_RECEIVE_PAYMENT_PATH: &str = "paths";

#[derive(Default, Clone, Debug)]
pub enum AssetType {
    #[default]
    Native,
    CreditAlphanum4(Asset),
    CreditAlphanum12(Asset),
}

#[derive(Clone, Debug)]
struct Asset {
    asset_code: String,
    issuer_account_id: String,
}

#[derive(Default, Clone, Debug)]
pub enum IssuedOrNative {
    #[default]
    Native,
    Issued(Asset),
}