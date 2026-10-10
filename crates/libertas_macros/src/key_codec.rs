use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Attribute, Data, DeriveInput, Expr, Fields, Generics, parse_quote};

fn reject_key_attributes(attributes: &[Attribute]) -> syn::Result<()> {
    if let Some(attr) = attributes
        .iter()
        .find(|attr| attr.path().is_ident("libertas_size"))
    {
        return Err(syn::Error::new_spanned(
            attr,
            "#[libertas_size(...)] is only supported on fields by key derives",
        ));
    }
    Ok(())
}

fn byte_bounds(attributes: &[Attribute]) -> syn::Result<Option<TokenStream>> {
    let mut min = None;
    let mut max = None;
    let mut seen = false;
    for attr in attributes
        .iter()
        .filter(|attr| attr.path().is_ident("libertas_size"))
    {
        if seen {
            return Err(syn::Error::new_spanned(attr, "duplicate libertas_size"));
        }
        seen = true;
        attr.parse_nested_meta(|meta| {
            let slot = if meta.path.is_ident("min") {
                &mut min
            } else if meta.path.is_ident("max") {
                &mut max
            } else {
                return Err(meta.error("expected min = N or max = N"));
            };
            if slot.is_some() {
                return Err(meta.error("duplicate size bound"));
            }
            *slot = Some(meta.value()?.parse::<Expr>()?);
            Ok(())
        })?;
    }
    if !seen {
        return Ok(None);
    }
    let max = max.ok_or_else(|| {
        syn::Error::new(
            Span::call_site(),
            "key byte fields require libertas_size(max = N)",
        )
    })?;
    let min = match min {
        Some(min) => quote!(::core::option::Option::Some(#min)),
        None => quote!(::core::option::Option::None),
    };
    Ok(Some(
        quote!(::libertas::data::__key_codec::ByteBounds::new(#min, #max)),
    ))
}

// The compiler resolves native types and constants. The derive composes their
// codec implementations rather than attempting name-based type resolution.
fn fields(
    fields: &Fields,
    accesses: &[TokenStream],
    generics: &mut Generics,
    decode: bool,
) -> syn::Result<(TokenStream, Vec<TokenStream>)> {
    let mut widths = Vec::new();
    let mut encodings = Vec::new();
    for (field, access) in fields.iter().zip(accesses) {
        let ty = &field.ty;
        let (field_width, operation) = match byte_bounds(&field.attrs)? {
            Some(capacity) => {
                generics.make_where_clause().predicates.push(parse_quote!(
                    #ty: ::libertas::data::__key_codec::BoundedBytes
                ));
                (
                    quote!(::libertas::data::__key_codec::bounded_field_width::<#ty>(const { #capacity })),
                    if decode {
                        quote!(::libertas::data::__key_codec::decode_bounded::<#ty>(
                            __libertas_key_field_input, const { #capacity }
                        ))
                    } else {
                        quote!(::libertas::data::__key_codec::encode_bounded(
                            #access, const { #capacity }, __libertas_key_field_output
                        ))
                    },
                )
            }
            None => {
                generics.make_where_clause().predicates.push(if decode {
                    parse_quote!(#ty: ::libertas::data::KeyFieldDecode)
                } else {
                    parse_quote!(#ty: ::libertas::data::KeyFieldCodec)
                });
                (
                    quote!(<#ty as ::libertas::data::KeyFieldCodec>::ENCODED_SIZE),
                    if decode {
                        quote!(<#ty as ::libertas::data::KeyFieldDecode>::decode_field(
                            __libertas_key_field_input
                        ))
                    } else {
                        quote!(<#ty as ::libertas::data::KeyFieldCodec>::write_field(
                            #access, __libertas_key_field_output
                        ))
                    },
                )
            }
        };
        widths.push(field_width.clone());
        encodings.push(if decode {
            quote! {
                let __libertas_key_field_input = ::libertas::data::__key_codec::take(
                    &mut __libertas_key_input, #field_width
                )?;
                let #access = #operation?;
            }
        } else {
            quote! {
                __libertas_key_output.field(#field_width, |__libertas_key_field_output| {
                    #operation
                })?;
            }
        });
    }
    let width = quote!({
        let mut __libertas_key_width = 0usize;
        #(
            __libertas_key_width = ::libertas::data::__key_codec::add_width(
                __libertas_key_width, #widths
            );
        )*
        __libertas_key_width
    });
    Ok((width, encodings))
}

fn validate_variant_count(count: usize, name: &syn::Ident) -> syn::Result<()> {
    if count == 0 {
        return Err(syn::Error::new_spanned(
            name,
            "a key enum must have at least one variant",
        ));
    }
    if count > 65_536 {
        return Err(syn::Error::new_spanned(
            name,
            "a key enum supports at most 65536 variants",
        ));
    }
    Ok(())
}

pub fn derive(input: DeriveInput, root: bool) -> syn::Result<TokenStream> {
    expand(input, root, false)
}

pub fn derive_decode(input: DeriveInput, root: bool) -> syn::Result<TokenStream> {
    expand(input, root, true)
}

fn bindings(fields: &Fields) -> Vec<syn::Ident> {
    fields
        .iter()
        .enumerate()
        .map(|(index, _)| format_ident!("__libertas_key_value_{index}", span = Span::mixed_site()))
        .collect()
}

fn expand(input: DeriveInput, root: bool, decode: bool) -> syn::Result<TokenStream> {
    reject_key_attributes(&input.attrs)?;
    let derive_name = match (root, decode) {
        (true, false) => "LibertasKeyCodec",
        (false, false) => "LibertasKeyFieldCodec",
        (true, true) => "LibertasKeyDecode",
        (false, true) => "LibertasKeyFieldDecode",
    };
    let name = &input.ident;
    let mut generics = input.generics.clone();
    let (width, encodings) = match &input.data {
        Data::Struct(data) if !root => {
            if !matches!(data.fields, Fields::Named(_)) {
                return Err(syn::Error::new_spanned(
                    name,
                    format!("{derive_name} requires a named struct or an enum"),
                ));
            }
            let bindings = bindings(&data.fields);
            let accesses = if decode {
                bindings
                    .iter()
                    .map(|binding| quote!(#binding))
                    .collect::<Vec<_>>()
            } else {
                data.fields
                    .iter()
                    .map(|field| {
                        let name = &field.ident;
                        quote!(&self.#name)
                    })
                    .collect::<Vec<_>>()
            };
            let (width, operations) = fields(&data.fields, &accesses, &mut generics, decode)?;
            let body = if decode {
                let names = data.fields.iter().map(|field| &field.ident);
                quote! {
                    #(#operations)*
                    ::core::result::Result::Ok(Self { #(#names: #bindings),* })
                }
            } else {
                quote!(#(#operations)*)
            };
            (width, body)
        }
        Data::Enum(data) => {
            validate_variant_count(data.variants.len(), name)?;
            let mut widths = Vec::new();
            let mut lengths = Vec::new();
            let mut arms = Vec::new();
            for (index, variant) in data.variants.iter().enumerate() {
                reject_key_attributes(&variant.attrs)?;
                let variant_name = &variant.ident;
                if let Some((_, tag)) = &variant.discriminant {
                    return Err(syn::Error::new_spanned(
                        tag,
                        "key variants use zero-based declaration indexes; remove the explicit discriminant",
                    ));
                }
                let tag = index as u16;
                if matches!(&variant.fields, Fields::Unnamed(fields) if fields.unnamed.len() != 1) {
                    return Err(syn::Error::new_spanned(
                        &variant.fields,
                        "key tuple variants require exactly one payload field",
                    ));
                }
                let bindings = bindings(&variant.fields);
                let accesses = bindings
                    .iter()
                    .map(|binding| quote!(#binding))
                    .collect::<Vec<_>>();
                let (width, writes) = fields(&variant.fields, &accesses, &mut generics, decode)?;
                let (pattern, length_pattern) = match &variant.fields {
                    Fields::Named(fields) => {
                        let names = fields.named.iter().map(|field| &field.ident);
                        (
                            quote!(Self::#variant_name { #(#names: #bindings),* }),
                            quote!(Self::#variant_name { .. }),
                        )
                    }
                    Fields::Unnamed(_) => (
                        quote!(Self::#variant_name(#(#bindings),*)),
                        quote!(Self::#variant_name(..)),
                    ),
                    Fields::Unit => (quote!(Self::#variant_name), quote!(Self::#variant_name)),
                };
                lengths.push(quote! {
                    #length_pattern => const { ::libertas::data::__key_codec::add_width(2, #width) }
                });
                if decode {
                    let check_length = if root {
                        quote! {
                            ::libertas::data::__key_codec::check_decode_len(input, const {
                                ::libertas::data::__key_codec::add_width(2, #width)
                            })?;
                        }
                    } else {
                        quote!()
                    };
                    arms.push(quote! {
                        __libertas_key_tag if __libertas_key_tag == const {
                            let __libertas_key_tag: ::core::primitive::u16 = #tag;
                            __libertas_key_tag
                        } => {
                            #check_length
                            #(#writes)*
                            ::libertas::data::__key_codec::check_padding(__libertas_key_input)?;
                            ::core::result::Result::Ok(#pattern)
                        }
                    });
                } else {
                    arms.push(quote! {
                    #pattern => {
                        let __libertas_key_tag: ::core::primitive::u16 = #tag;
                        __libertas_key_output.write(&__libertas_key_tag.to_be_bytes())?;
                        #(#writes)*
                        __libertas_key_output.fill(0, __libertas_key_output.remaining())?;
                    }
                    });
                }
                widths.push(width);
            }
            let width = if root {
                quote!(match self { #(#lengths),* })
            } else {
                quote!({
                    let mut __libertas_key_max = 0usize;
                    #(
                        let __libertas_key_width = #widths;
                        if __libertas_key_width > __libertas_key_max {
                            __libertas_key_max = __libertas_key_width;
                        }
                    )*
                    ::libertas::data::__key_codec::add_width(2, __libertas_key_max)
                })
            };
            let body = if decode {
                quote! {
                    let __libertas_key_tag = <::core::primitive::u16 as ::libertas::data::KeyFieldDecode>::decode_field(
                        ::libertas::data::__key_codec::take(&mut __libertas_key_input, 2)?
                    )?;
                    match __libertas_key_tag {
                        #(#arms),*,
                        __libertas_key_unknown_tag => ::core::result::Result::Err(
                            ::libertas::data::KeyDecodeError::UnknownDiscriminator { discriminator: __libertas_key_unknown_tag }
                        ),
                    }
                }
            } else {
                quote!(match self { #(#arms),* })
            };
            (width, body)
        }
        _ => {
            return Err(syn::Error::new_spanned(
                name,
                if root {
                    format!("{derive_name} requires a root enum")
                } else {
                    format!("{derive_name} requires a named struct or an enum")
                },
            ));
        }
    };
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    if decode {
        let (trait_name, method, check_length) = if root {
            (
                quote!(::libertas::data::KeyDecode),
                quote!(decode_key),
                quote!(),
            )
        } else {
            (
                quote!(::libertas::data::KeyFieldDecode),
                quote!(decode_field),
                quote! {
                    ::libertas::data::__key_codec::check_decode_len(
                        input, <Self as ::libertas::data::KeyFieldCodec>::ENCODED_SIZE
                    )?;
                },
            )
        };
        Ok(quote! {
            impl #impl_generics #trait_name for #name #type_generics #where_clause {
                fn #method(input: &[::core::primitive::u8]) -> ::core::result::Result<Self, ::libertas::data::KeyDecodeError> {
                    #check_length
                    let mut __libertas_key_input = input;
                    #encodings
                }
            }
        })
    } else if root {
        Ok(quote! {
            impl #impl_generics ::libertas::data::KeyCodec for #name #type_generics #where_clause {
                fn encoded_len(&self) -> ::core::primitive::usize { #width }

                fn write_key(&self, output: &mut ::libertas::data::KeyWriter<'_>) -> ::core::result::Result<(), ::libertas::data::KeyEncodeError> {
                    output.check_len(<Self as ::libertas::data::KeyCodec>::encoded_len(self))?;
                    let __libertas_key_output = output;
                    #encodings
                    ::core::result::Result::Ok(())
                }
            }
        })
    } else {
        Ok(quote! {
            impl #impl_generics ::libertas::data::KeyFieldCodec for #name #type_generics #where_clause {
                const ENCODED_SIZE: ::core::primitive::usize = #width;

                fn write_field(&self, output: &mut ::libertas::data::KeyWriter<'_>) -> ::core::result::Result<(), ::libertas::data::KeyEncodeError> {
                    output.check_len(<Self as ::libertas::data::KeyFieldCodec>::ENCODED_SIZE)?;
                    let __libertas_key_output = output;
                    #encodings
                    ::core::result::Result::Ok(())
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_declarations_with_local_diagnostics() {
        let cases: &[(bool, &str, &str)] = &[
            (true, "struct Key { value: u8 }", "requires a root enum"),
            (false, "struct Field(u8);", "requires a named struct"),
            (
                false,
                "union Field { value: u8 }",
                "requires a named struct",
            ),
            (
                true,
                "enum Key { A = 0 }",
                "remove the explicit discriminant",
            ),
            (
                false,
                "#[repr(u8)] enum Field { A = 0 }",
                "remove the explicit discriminant",
            ),
            (
                true,
                "#[repr(u16)] enum Key { A, B = 42 }",
                "remove the explicit discriminant",
            ),
            (false, "#[repr(u16)] enum Field {}", "at least one variant"),
            (true, "enum Key { A(u8, u8) }", "exactly one payload"),
            (
                false,
                "#[libertas_size(max = 2)] struct Field { value: String }",
                "only supported on fields",
            ),
            (
                true,
                "#[repr(u16)] enum Key { #[libertas_size(max = 2)] A = 0 }",
                "only supported on fields",
            ),
            (
                false,
                "struct Field { #[libertas_size] value: String }",
                "expected attribute arguments",
            ),
            (
                false,
                "struct Field { #[libertas_size()] value: String }",
                "require libertas_size(max = N)",
            ),
            (
                false,
                "struct Field { #[libertas_size(size = 2)] value: String }",
                "expected min = N or max = N",
            ),
            (
                false,
                "struct Field { #[libertas_size(max = 2, max = 3)] value: String }",
                "duplicate size bound",
            ),
            (
                false,
                "struct Field { #[libertas_size(max = 2)] #[libertas_size(max = 3)] value: String }",
                "duplicate libertas_size",
            ),
        ];
        for (root, input, message) in cases {
            for decode in [false, true] {
                let error = expand(syn::parse_str(input).unwrap(), *root, decode).unwrap_err();
                assert!(error.to_string().contains(message), "{input}: {error}");
            }
        }
    }

    #[test]
    fn every_u16_index_is_available_and_the_next_variant_is_rejected() {
        let name = syn::parse_str("Key").unwrap();
        assert!(validate_variant_count(1, &name).is_ok());
        assert!(validate_variant_count(65_536, &name).is_ok());
        assert!(validate_variant_count(65_537, &name).is_err());
        assert!(validate_variant_count(0, &name).is_err());
    }
}
