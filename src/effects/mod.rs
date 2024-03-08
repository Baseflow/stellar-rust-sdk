pub mod all_effects_request;
pub mod effects_for_account_request;
pub mod effects_for_ledger_request;
pub mod effects_for_liquidity_pools_request;
pub mod response;

static EFFECTS_PATH: &str = "effects";

pub mod prelude {
    pub use super::all_effects_request::*;
    pub use super::effects_for_account_request::*;
    pub use super::effects_for_ledger_request::*;
    pub use super::effects_for_liquidity_pools_request::*;
    pub use super::response::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_test() {
        assert_eq!(EFFECTS_PATH, "effects");
    }
}
