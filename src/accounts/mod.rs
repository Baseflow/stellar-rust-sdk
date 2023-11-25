pub mod accounts_request;
pub mod accounts_response;
pub mod single_account_request;
pub mod single_account_response;

static ACCOUNTS_PATH: &str = "accounts";

pub mod prelude {
    pub use super::accounts_request::*;
    pub use super::accounts_response::*;
    pub use super::single_account_request::*;
    pub use super::single_account_response::*;
}
