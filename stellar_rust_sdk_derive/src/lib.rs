extern crate proc_macro2;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Pagination, attributes(Pagination))]
pub fn pagination_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let expanded = quote! {
        impl #impl_generics Paginatable for #struct_name #type_generics #where_clause {
            fn set_cursor(self, cursor: u32) -> Result<Self, String> {
                // Always accept the cursor since it's non-optional in the setter
                if cursor < 1 {
                    return Err("Cursor must be greater than or equal to 1.".to_string());
                }

                Ok(Self { cursor: Some(cursor), ..self })
            }

            fn set_limit(self, limit: u8) -> Result<Self, String> {
                // Validate limit if necessary
                if !(1..=200).contains(&limit) {
                    Err("Limit must be between 1 and 200.".to_string())
                } else {
                    Ok(Self { limit: Some(limit), ..self })
                }
            }

            fn set_order(self, order: Order) -> Result<Self, String> {
                // No validation required for setting the order in this context
                Ok(Self { order: Some(order), ..self })
            }
        }
    };
    TokenStream::from(expanded)
}
