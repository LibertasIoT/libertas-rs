use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, FnArg, Pat, DeriveInput};

#[proc_macro_attribute]
pub fn libertas_export(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    // Iterate through the function arguments (inputs)
    for arg in input.sig.inputs.iter_mut() {
        if let FnArg::Typed(pat_type) = arg {
            // 1. Check the attributes on this specific argument
            // We use 'retain' to keep only the attributes that are NOT ours
            pat_type.attrs.retain(|attr| {
                if attr.path().is_ident("data_exchange_schema") ||
                   attr.path().is_ident("data_exchange_client") {
                    // Get the parameter name for logging
                    if let Pat::Ident(ref id) = *pat_type.pat {
                        println!("Consuming attr on argument: {}", id.ident);
                    }
                    return false; // This "consumes" (removes) the attribute
                }
                true // Keep everything else (like #[allow] or #[deprecated])
            });
        }
    }

    // 2. Return the modified function (now clean of custom attributes)
    TokenStream::from(quote! {
        #input
    })
}

#[proc_macro_derive(LibertasExport, attributes(data_exchange_schema, data_exchange_client))]
pub fn libertas_derive(input: TokenStream) -> TokenStream {
    // Parse the representation of the struct
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // We aren't doing anything with the fields here,
    // just providing a blank implementation of a hypothetical trait.
    let expanded = quote! {
        impl LibertasExport for #name {
        }
    };

    TokenStream::from(expanded)
}
