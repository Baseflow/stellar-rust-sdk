pub mod details_request;
pub mod response;

static ORDER_BOOK_PATH: &str = "order_book";

mod prelude {
    pub use super::details_request::*;
    pub use super::response::*;
}