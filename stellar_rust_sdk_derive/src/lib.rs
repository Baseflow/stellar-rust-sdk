extern crate proc_macro2;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Fields, Field, parse::Nothing};

/// The procedural attribute macro to add pagination functionality to request structs.
///
/// This macro automatically injects pagination-related fields and methods into a struct
/// to facilitate paginated API requests. Specifically, it adds three optional fields
/// and three methods:
///
/// - `cursor`: An `Option<u32>` field that represents the pagination cursor. The cursor
///   is used to track the current position in a paginated dataset. The `set_cursor` method
///   allows setting this field, with a validation that ensures the cursor is greater than
///   or equal to 1.
///
/// - `limit`: An `Option<u8>` field that specifies the maximum number of items to retrieve
///   in a single page. The `set_limit` method allows setting this field, ensuring that the
///   limit is within a valid range (between 1 and 200).
///
/// - `order`: An `Option<Order>` field that defines the sort order of the paginated results.
///   The `set_order` method allows setting this field without additional validation, as the
///   sort order is context-dependent.
///
/// # Usage
///
/// Apply the `#[pagination]` attribute to a struct to automatically add pagination
/// functionality.
///
#[proc_macro_attribute]
pub fn pagination(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemStruct);

    // No arguments should be passed, but if they are, parse them as `Nothing` to prevent misuse. 
    let _ = parse_macro_input!(args as Nothing);

    // Create required fields to be added to the struct.
    let cursor_field: Field = syn::parse_quote! {
        pub cursor: Option<u32>
    };
    let limit_field: Field = syn::parse_quote! {
        pub limit: Option<u8>
    };
    let order_field: Field = syn::parse_quote! {
        pub order: Option<Order>
    };

    // Add the fields to the struct.
    if let Fields::Named(ref mut fields) = input.fields {
        fields.named.push(cursor_field);
        fields.named.push(limit_field);
        fields.named.push(order_field);
    }

    let struct_name = &input.ident;

    // Split the generics into implementation, type, and where clause parts, so that the macro supports generic structs.
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    // Add methods to the struct.
    let expanded = quote! {
        #input
        impl #impl_generics #struct_name #type_generics #where_clause {
            pub fn set_cursor(self, cursor: u32) -> Result<Self, String> {
                // Always accept the cursor since it's non-optional in the setter
                if cursor < 1 {
                    return Err("Cursor must be greater than or equal to 1.".to_string());
                }

                Ok(Self { cursor: Some(cursor), ..self })
            }

            pub fn set_limit(self, limit: u8) -> Result<Self, String> {
                // Validate limit if necessary
                if !(1..=200).contains(&limit) {
                    Err("Limit must be between 1 and 200.".to_string())
                } else {
                    Ok(Self { limit: Some(limit), ..self })
                }
            }

            pub fn set_order(self, order: Order) -> Result<Self, String> {
                // No validation required for setting the order in this context
                Ok(Self { order: Some(order), ..self })
            }
        }
    };
    TokenStream::from(expanded)
}