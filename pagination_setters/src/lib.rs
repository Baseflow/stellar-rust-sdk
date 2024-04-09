extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(PaginationSetters)]
pub fn pagination_setters_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the struct name from the input
    let name = input.ident;

    // Generate the implementation of the PaginationSetters trait
    let expanded = quote! {
        impl PaginationSetters for #name {
            fn set_cursor(mut self, cursor: u32) -> Result<Self, String> {
                if cursor < 1 {
                    return Err("cursor must be greater than or equal to 1".to_string());
                }

                self.cursor = Some(cursor);
                Ok(self)
            }

            fn set_limit(mut self, limit: u8) -> Result<Self, String> {
                if limit < 1 || limit > 200 {
                    return Err("limit must be between 1 and 200".to_string());
                }

                self.limit = Some(limit);
                Ok(self)
            }

            fn set_order(mut self, order: Order) -> Self {
                self.order = Some(order);
                self
            }
        }
    };

    // Convert the generated code into a TokenStream and return it
    TokenStream::from(expanded)
}
