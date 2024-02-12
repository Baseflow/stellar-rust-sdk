pub mod all_effects_request;

pub mod all_effects_response;


static EFFECTS_PATH: &str = "effects";

pub mod prelude {
    pub use super::all_effects_request::*;
    pub use super::all_effects_response::*;
}