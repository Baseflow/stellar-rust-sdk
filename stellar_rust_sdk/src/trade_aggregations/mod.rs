// TODO: Documentation
pub mod trade_aggregations_request;

// TODO: Documentation
pub mod response;

// TODO: Documentation
static TRADE_AGGREGATIONS_PATH: &str = "trade_aggregations";

// TODO: Documentation
pub mod prelude {
    pub use super::trade_aggregations_request::*;
    pub use super::response::*;
}

#[cfg(test)]
pub mod test {
    // TODO: Write tests
}