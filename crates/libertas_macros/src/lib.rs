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
                if attr.path().is_ident("agent_tool_schema") ||
                   attr.path().is_ident("agent_tool_server") || 
                   attr.path().is_ident("tag") ||
                   attr.path().is_ident("content") || 
                   attr.path().is_ident("untagged") {
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

#[proc_macro_derive(LibertasExport, attributes(agent_tool_schema, agent_tool_server))]
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

#[proc_macro_derive(LibertasAvroEncode)]
pub fn libertas_avro_encode_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let encode_logic = match &input.data {
        syn::Data::Struct(data) => {
            if let syn::Fields::Named(fields) = &data.fields {
                let field_encodes = fields.named.iter().map(|f| {
                    let field_name = &f.ident;
                    quote! {
                        ::libertas::AvroEncode::avro_encode(&self.#field_name, buffer);
                    }
                });
                quote! {
                    #(#field_encodes)*
                }
            } else {
                panic!("LibertasAvroEncode only supports structs with named fields");
            }
        },
        syn::Data::Enum(data) => {
            let has_discriminant = data.variants.iter().any(|v| v.discriminant.is_some());
            if has_discriminant {
                for v in &data.variants {
                    if !v.fields.is_empty() {
                        panic!("LibertasAvroEncode: enums with discriminants cannot have fields");
                    }
                }
                let variants = data.variants.iter().map(|v| {
                    let variant_name = &v.ident;
                    quote! {
                        Self::#variant_name => (Self::#variant_name as i32),
                    }
                });
                quote! {
                    let val = match self {
                        #(#variants)*
                    };
                    ::libertas::AvroEncode::avro_encode(&val, buffer);
                }
            } else {
                let variants = data.variants.iter().enumerate().map(|(i, v)| {
                    let variant_name = &v.ident;
                    let index = i as i32;
                    match &v.fields {
                        syn::Fields::Unit => {
                            quote! {
                                Self::#variant_name => ::libertas::AvroEncode::avro_encode(&#index, buffer),
                            }
                        },
                        syn::Fields::Unnamed(fields) => {
                            let field_names: std::vec::Vec<_> = (0..fields.unnamed.len())
                                .map(|idx| syn::Ident::new(&format!("f{}", idx), proc_macro2::Span::call_site()))
                                .collect();
                            let field_encodes = field_names.iter().map(|f| quote! { ::libertas::AvroEncode::avro_encode(#f, buffer); });
                            quote! {
                                Self::#variant_name(#(#field_names),*) => {
                                    ::libertas::AvroEncode::avro_encode(&#index, buffer);
                                    #(#field_encodes)*
                                }
                            }
                        },
                        syn::Fields::Named(fields) => {
                            let field_names: std::vec::Vec<_> = fields.named.iter().map(|f| f.ident.as_ref().unwrap()).collect();
                            let field_encodes = field_names.iter().map(|f| quote! { ::libertas::AvroEncode::avro_encode(#f, buffer); });
                            quote! {
                                Self::#variant_name { #(#field_names),* } => {
                                    ::libertas::AvroEncode::avro_encode(&#index, buffer);
                                    #(#field_encodes)*
                                }
                            }
                        }
                    }
                });
                quote! {
                    match self {
                        #(#variants)*
                    }
                }
            }
        },
        _ => panic!("LibertasAvroEncode only supports structs and enums"),
    };

    let expanded = quote! {
        impl ::libertas::NotBytesEncode for #name {}

        impl ::libertas::AvroEncode for #name {
            fn avro_encode(&self, buffer: &mut std::vec::Vec<u8>) {
                #encode_logic
            }
        }

        impl #name {
            pub fn to_avro(&self) -> std::vec::Vec<u8> {
                let mut buffer = std::vec::Vec::new();
                ::libertas::AvroEncode::avro_encode(self, &mut buffer);
                buffer
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(LibertasAvroDecode)]
pub fn libertas_avro_decode_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let decode_logic = match &input.data {
        syn::Data::Struct(data) => {
            if let syn::Fields::Named(fields) = &data.fields {
                let field_decodes = fields.named.iter().map(|f| {
                    let field_name = &f.ident;
                    let ty = &f.ty;
                    quote! {
                        #field_name: <#ty as ::libertas::AvroDecode>::avro_decode(buffer, offset)?
                    }
                });
                quote! {
                    Ok(Self {
                        #(#field_decodes),*
                    })
                }
            } else {
                panic!("LibertasAvroDecode only supports structs with named fields");
            }
        },
        syn::Data::Enum(data) => {
            let has_discriminant = data.variants.iter().any(|v| v.discriminant.is_some());
            if has_discriminant {
                for v in &data.variants {
                    if !v.fields.is_empty() {
                        panic!("LibertasAvroDecode: enums with discriminants cannot have fields");
                    }
                }
                let variants = data.variants.iter().map(|v| {
                    let variant_name = &v.ident;
                    quote! {
                        x if x == (Self::#variant_name as i32) => Ok(Self::#variant_name),
                    }
                });
                quote! {
                    let val = <i32 as ::libertas::AvroDecode>::avro_decode(buffer, offset)?;
                    match val {
                        #(#variants)*
                        _ => Err("Invalid enum integer value"),
                    }
                }
            } else {
                let variants = data.variants.iter().enumerate().map(|(i, v)| {
                    let variant_name = &v.ident;
                    let index = i as i32;
                    match &v.fields {
                        syn::Fields::Unit => {
                            quote! {
                                #index => Ok(Self::#variant_name),
                            }
                        },
                        syn::Fields::Unnamed(fields) => {
                            let field_decodes = fields.unnamed.iter().map(|f| {
                                let ty = &f.ty;
                                quote! { <#ty as ::libertas::AvroDecode>::avro_decode(buffer, offset)? }
                            });
                            quote! {
                                #index => Ok(Self::#variant_name(#(#field_decodes),*)),
                            }
                        },
                        syn::Fields::Named(fields) => {
                            let field_decodes = fields.named.iter().map(|f| {
                                let field_name = &f.ident;
                                let ty = &f.ty;
                                quote! { #field_name: <#ty as ::libertas::AvroDecode>::avro_decode(buffer, offset)? }
                            });
                            quote! {
                                #index => Ok(Self::#variant_name { #(#field_decodes),* }),
                            }
                        }
                    }
                });
                quote! {
                    let index = <i32 as ::libertas::AvroDecode>::avro_decode(buffer, offset)?;
                    match index {
                        #(#variants)*
                        _ => Err("Invalid enum variant index"),
                    }
                }
            }
        },
        _ => panic!("LibertasAvroDecode only supports structs and enums"),
    };

    let expanded = quote! {
        impl ::libertas::NotBytesDecode for #name {}

        impl ::libertas::AvroDecode for #name {
            fn avro_decode(buffer: &[u8], offset: &mut usize) -> std::result::Result<Self, &'static str> {
                #decode_logic
            }
        }

        impl #name {
            pub fn from_avro(buffer: &[u8]) -> std::result::Result<Self, &'static str> {
                let mut offset = 0;
                let result = <Self as ::libertas::AvroDecode>::avro_decode(buffer, &mut offset)?;
                if offset != buffer.len() {
                    return Err("Trailing bytes after decoding");
                }
                Ok(result)
            }
        }
    };
    TokenStream::from(expanded)
}
