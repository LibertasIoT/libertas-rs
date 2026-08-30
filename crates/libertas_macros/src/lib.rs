use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::ext::IdentExt;
use syn::{DeriveInput, ExprPath, FnArg, ItemFn, Pat, Path, PathArguments, parse_macro_input};

fn variant_index_const_ident(variant: &syn::Ident) -> syn::Ident {
    let variant = variant.unraw();
    format_ident!("__VARIANT_INDEX_{}", variant, span = variant.span())
}

/// Generates declaration-order indexes for every variant of an enum.
///
/// The generated associated constants are intended to be referenced through
/// [`variant_index!`]. Unit, tuple, and struct variants are all supported, and
/// the enum's generic parameters and `where` clause are preserved.
///
/// # Stability
///
/// This macro supports two distinct compatibility scenarios for protocol and
/// data Union types in the Libertas type system.
///
/// ## Append-only version updates
///
/// A new version of a published Union may add variants only after all existing
/// variants. It must not remove, reorder, or insert variants before an existing
/// variant, because doing so would change an established declaration index.
/// Libertas development tools validate and enforce this version-evolution rule.
/// Declaration indexes therefore remain stable across compatible, append-only
/// versions.
///
/// ## Runtime client-server protocol compatibility
///
/// Protocol compatibility is asymmetric. An endpoint client maintains the
/// server's protocol declaration, but the server does not have the client's
/// protocol declaration. Client/server matching occurs during configuration,
/// but that match validates only the mandatory protocol requirements. It does
/// not establish that the server supports every optional payload type the
/// client may use later.
///
/// Before sending a request that uses an optional payload type, the client must
/// therefore check the active server's protocol declaration at runtime. The
/// server cannot perform this check on the client's behalf because it does not
/// have the client's declaration. The client should use [`variant_index!`]
/// instead of a hard-coded ordinal for this check. When the payload type is
/// reached through nested Union variants, the client must check every variant
/// along that declaration path before sending the request.
///
/// ```
/// use libertas_macros::{variant_index, VariantIndex};
///
/// struct Connect;
/// struct Data;
///
/// #[derive(VariantIndex)]
/// enum Message {
///     Connect(Connect),
///     Data(Data),
/// }
///
/// const DATA_INDEX: usize = variant_index!(Message::Data);
/// assert_eq!(DATA_INDEX, 1);
/// ```
#[proc_macro_derive(VariantIndex)]
pub fn variant_index_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let data = match &input.data {
        syn::Data::Enum(data) => data,
        _ => {
            return syn::Error::new_spanned(name, "VariantIndex can only be derived for an enum")
                .to_compile_error()
                .into();
        }
    };
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let constants = data.variants.iter().enumerate().map(|(index, variant)| {
        let const_name = variant_index_const_ident(&variant.ident);
        quote! {
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            pub const #const_name: ::core::primitive::usize = #index;
        }
    });

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #(#constants)*
        }
    }
    .into()
}

/// Expands an enum variant path to its zero-based declaration index.
///
/// The enum must derive [`VariantIndex`]. The expansion is an associated
/// constant expression, so it can initialize constants and index constant
/// tables without constructing the enum variant or its payload.
///
/// Qualified paths and generic enums are supported:
///
/// ```
/// use libertas_macros::{variant_index, VariantIndex};
///
/// #[derive(VariantIndex)]
/// enum Message<T> {
///     Empty,
///     Data(T),
/// }
///
/// const DATA_INDEX: usize = variant_index!(Message::<String>::Data);
/// const TABLE: [u32; 2] = [10, 20];
/// const DATA_VALUE: u32 = TABLE[variant_index!(Message::<String>::Data)];
///
/// assert_eq!(DATA_INDEX, 1);
/// assert_eq!(DATA_VALUE, 20);
/// ```
#[proc_macro]
pub fn variant_index(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ExprPath);
    if input.qself.is_some() {
        return syn::Error::new_spanned(
            input,
            "variant_index! expects an enum variant path such as Message::Data",
        )
        .to_compile_error()
        .into();
    }

    let leading_colon = input.path.leading_colon;
    let mut segments = input.path.segments.into_iter().collect::<Vec<_>>();
    let Some(variant) = segments.pop() else {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "variant_index! expects an enum variant path such as Message::Data",
        )
        .to_compile_error()
        .into();
    };
    if segments.is_empty() || !matches!(&variant.arguments, PathArguments::None) {
        return syn::Error::new_spanned(
            variant,
            "variant_index! expects an enum variant path such as Message::Data",
        )
        .to_compile_error()
        .into();
    }

    let const_name = variant_index_const_ident(&variant.ident);
    let enum_path = Path {
        leading_colon,
        segments: segments.into_iter().collect(),
    };

    quote! {
        #enum_path::#const_name
    }
    .into()
}

/// This macro is used on Libertas public functions.
/// 
#[proc_macro_attribute]
pub fn libertas_export(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    // Iterate through the function arguments (inputs)
    for arg in input.sig.inputs.iter_mut() {
        if let FnArg::Typed(pat_type) = arg {
            // 1. Check the attributes on this specific argument
            // We use 'retain' to keep only the attributes that are NOT ours
            pat_type.attrs.retain(|attr| {
                if attr.path().is_ident("libertas_copy_from") ||
                   attr.path().is_ident("libertas_enum_source") ||
                   attr.path().is_ident("libertas_bitflags") ||
                   attr.path().is_ident("libertas_date_only") ||
                   attr.path().is_ident("libertas_format") ||
                   attr.path().is_ident("libertas_formatted_text") ||
                   attr.path().is_ident("libertas_chart") ||
                   attr.path().is_ident("libertas_chart_channel") ||
                   attr.path().is_ident("libertas_chart_scale") ||
                   attr.path().is_ident("libertas_chart_guide") ||
                   attr.path().is_ident("libertas_physical_unit") ||
                   attr.path().is_ident("libertas_endpoint_schema") ||
                   attr.path().is_ident("libertas_endpoint_base_objects") || 
                   attr.path().is_ident("libertas_endpoint_server") || 
                   attr.path().is_ident("libertas_permissions") ||
                   attr.path().is_ident("libertas_data_schema") || 
                   attr.path().is_ident("libertas_default") || 
                   attr.path().is_ident("libertas_fixed") ||
                   attr.path().is_ident("libertas_device_type") || 
                   attr.path().is_ident("libertas_ui_header") || 
                   attr.path().is_ident("libertas_read_only") || 
                   attr.path().is_ident("libertas_hidden") || 
                   attr.path().is_ident("libertas_exclude_ui") ||
                   attr.path().is_ident("libertas_number") || 
                   attr.path().is_ident("libertas_size") ||
                   attr.path().is_ident("libertas_time_interval") || 
                   attr.path().is_ident("libertas_string") || 
                   attr.path().is_ident("libertas_enum") || 
                   attr.path().is_ident("libertas_array") || 
                   attr.path().is_ident("libertas_unordered") || 
                   attr.path().is_ident("libertas_unique") || 
                   attr.path().is_ident("libertas_virtual_device_type") ||
                   attr.path().is_ident("content") {
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

#[proc_macro_attribute]
pub fn libertas_string_resources(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parser-only metadata is valid on an exported function and on a reusable
    // named schema type. The source parser validates the exact placement
    // and resolves the referenced constant; the Rust compiler only needs the
    // annotation consumed without rewriting the item.
    item
}

/// Declares the compile-time permission list required by a Libertas function.
#[proc_macro_attribute]
pub fn libertas_permissions(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    TokenStream::from(quote! {
        #input
    })
}

/// Marks a Libertas function as allowing at most one task on a Hub.
///
/// This is parser-only metadata. The Hub enforces the constraint when a task
/// is created, when its function changes, or when its App version changes.
#[proc_macro_attribute]
pub fn libertas_singleton(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn libertas_data_schema(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    // Return the modified function (now clean of custom attributes)
    TokenStream::from(quote! {
        #input
    })
}

/// Declares the direct official App type for a published compatible type.
///
/// Structs and enums may also use this spelling as a `LibertasExport` helper
/// attribute. The standalone attribute form keeps compatible type aliases valid
/// Rust source; placement and FQN validation belong to the schema parser.
#[proc_macro_attribute]
pub fn libertas_foreign_type(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Parser-only chart mark/composition metadata. The schema parser validates it.
#[proc_macro_attribute]
pub fn libertas_chart(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Parser-only chart channel metadata. The schema parser validates it.
#[proc_macro_attribute]
pub fn libertas_chart_channel(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Parser-only chart scale metadata. The schema parser validates it.
#[proc_macro_attribute]
pub fn libertas_chart_scale(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Parser-only chart guide metadata. The schema parser validates it.
#[proc_macro_attribute]
pub fn libertas_chart_guide(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Parser-only portable physical-unit metadata. The schema parser validates it.
#[proc_macro_attribute]
pub fn libertas_physical_unit(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Parser-only Unicode LDML display-format metadata.
///
/// The schema parser validates the pattern and its effective host type. This
/// standalone form keeps reusable numeric and temporal type aliases valid Rust
/// source; fields and exported function parameters use the derive/export
/// helper paths below.
#[proc_macro_attribute]
pub fn libertas_format(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Marks either a bitflag catalog declaration or an integer field that uses it.
///
/// The schema parser fabricates a localized Enumeration from a catalog and
/// projects that catalog onto integer fields. This macro intentionally leaves
/// Rust runtime representation and wire encoding unchanged. It also works on a
/// struct emitted by the popular `bitflags!` macro.
#[proc_macro_attribute]
pub fn libertas_bitflags(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Enables parser-only attributes on published structs and enums.
///
/// `libertas_foreign_type` is accepted by Rust through this derive because it
/// declares compatibility for the published named type itself. The schema
/// parser rejects it on fields, variants, and references.
#[proc_macro_derive(LibertasExport, attributes(
    libertas_copy_from,
    libertas_enum_source,
    libertas_bitflags,
    libertas_date_only,
    libertas_format,
    libertas_formatted_text,
    libertas_endpoint_schema, 
    libertas_endpoint_server, 
    libertas_endpoint_base_objects,
    libertas_permissions,
    libertas_foreign_type,
    libertas_request,
    // Restricts which end-user endpoint access levels may invoke this request;
    // the schema parser canonicalizes "Read"/"Write" names to a bitmap.
    libertas_access_privilege,
    libertas_response, 
    libertas_error,
    libertas_subscription_request, 
    libertas_subscription_data,
    libertas_default,
    // On an Endpoint client this carries an editable peer-process constraint;
    // it remains parser-only metadata and does not fix the encoded Endpoint ID.
    libertas_fixed,
    libertas_device_type,
    libertas_virtual_device_type,
    libertas_ui_header,
    libertas_read_only,
    libertas_hidden,
    // Omits a server Endpoint or protocol variant from client-generated UI;
    // runtime protocol behavior and wire encoding remain unchanged.
    libertas_exclude_ui,
    libertas_number,
    libertas_size,
    libertas_time_interval,
    libertas_string,
    libertas_enum,
    libertas_array,
    libertas_unordered,
    libertas_unique,
    libertas_next_request,
    libertas_next_response,
    libertas_cacheable,
    libertas_protocol_conformance,
    libertas_chart,
    libertas_chart_channel,
    libertas_chart_scale,
    libertas_chart_guide,
    libertas_physical_unit,
    ))]
pub fn libertas_derive(input: TokenStream) -> TokenStream {
    // 1. Parse the input tokens into a syntax tree
    // We parse it even if we do nothing to ensure the code is valid Rust
    let _input = parse_macro_input!(input as DeriveInput);

    // 2. Return an empty TokenStream
    // This "does nothing" to the final compiled output
    TokenStream::new()
}

#[proc_macro_derive(LibertasAvroEncode)]
pub fn libertas_avro_encode_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

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
        impl #impl_generics ::libertas::NotBytesEncode for #name #ty_generics #where_clause {}

        impl #impl_generics ::libertas::AvroEncode for #name #ty_generics #where_clause {
            fn avro_encode(&self, buffer: &mut alloc::vec::Vec<u8>) {
                #encode_logic
            }
        }

        impl #impl_generics #name #ty_generics #where_clause {
            pub fn to_avro(&self) -> alloc::vec::Vec<u8> {
                let mut buffer = alloc::vec::Vec::new();
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
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

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
        impl #impl_generics ::libertas::NotBytesDecode for #name #ty_generics #where_clause {}

        impl #impl_generics ::libertas::AvroDecode for #name #ty_generics #where_clause {
            fn avro_decode(buffer: &[u8], offset: &mut usize) -> core::result::Result<Self, &'static str> {
                #decode_logic
            }
        }

        impl #impl_generics #name #ty_generics #where_clause {
            pub fn from_avro(buffer: &[u8]) -> core::result::Result<Self, &'static str> {
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
