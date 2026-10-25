use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::ext::IdentExt;
use syn::{DeriveInput, ExprPath, FnArg, ItemFn, Pat, Path, PathArguments, parse_macro_input};

mod data_schema;
mod key_codec;

/// Derives `libertas::data::KeyCodec` for a root key enum.
///
/// Uses the zero-based variant index as a big-endian `u16` tag.
/// Supports up to 65,536 variants; explicit discriminants are rejected.
/// Published root variants are append-only. No `repr` annotation is required.
/// Encodes the selected tag and fields without padding to other root variants.
/// String and byte-vector fields require `#[libertas_size(max = N)]`.
/// Option fields add a one-byte presence flag and retain the full payload slot.
/// Optional String/byte-vector fields use the same byte-capacity annotation.
/// This derive generates encoding only; it imposes no decoding requirement.
#[proc_macro_derive(LibertasKeyCodec, attributes(libertas_size, libertas_pattern))]
pub fn libertas_key_codec_derive(input: TokenStream) -> TokenStream {
    key_codec::derive(parse_macro_input!(input as DeriveInput), true)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Derives `libertas::data::KeyFieldCodec` for a fixed-width key component.
///
/// Named structs concatenate their fields. Enums encode their zero-based
/// variant index as big-endian `u16` and pad to the largest variant payload.
/// String and byte-vector fields require `#[libertas_size(max = N)]`.
/// Option fields add a one-byte presence flag; None has an all-zero payload slot.
/// Published layouts must remain unchanged, including nested enum variants.
#[proc_macro_derive(LibertasKeyFieldCodec, attributes(libertas_size, libertas_pattern))]
pub fn libertas_key_field_codec_derive(input: TokenStream) -> TokenStream {
    key_codec::derive(parse_macro_input!(input as DeriveInput), false)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Derives `libertas::data::KeyDecode` alongside `LibertasKeyCodec`.
///
/// Decodes an exact root key, rejecting unknown discriminants, extra or missing
/// bytes, and noncanonical field encodings. Components must support decoding.
#[proc_macro_derive(LibertasKeyDecode, attributes(libertas_size, libertas_pattern))]
pub fn libertas_key_decode_derive(input: TokenStream) -> TokenStream {
    key_codec::derive_decode(parse_macro_input!(input as DeriveInput), true)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Derives `libertas::data::KeyFieldDecode` alongside `LibertasKeyFieldCodec`.
///
/// Uses the same fixed-width layout and validates canonical union and bounded
/// byte padding. String fields also require valid UTF-8.
#[proc_macro_derive(LibertasKeyFieldDecode, attributes(libertas_size, libertas_pattern))]
pub fn libertas_key_field_decode_derive(input: TokenStream) -> TokenStream {
    key_codec::derive_decode(parse_macro_input!(input as DeriveInput), false)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

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
                // Parameter documentation is consumed by schema extraction. Rust
                // does not permit doc attributes on compiled function parameters.
                if attr.path().is_ident("doc")
                    || attr.path().is_ident("libertas_workflow_request")
                    || attr.path().is_ident("libertas_prev_request")
                    || attr.path().is_ident("libertas_calculation_request")
                    || attr.path().is_ident("libertas_calculation_root")
                    || attr.path().is_ident("libertas_copy_from")
                    || attr.path().is_ident("libertas_constrained_by")
                    || attr.path().is_ident("libertas_action_control")
                    || attr.path().is_ident("libertas_ai_navigation_prompt")
                    || attr.path().is_ident("libertas_default_request")
                    || attr.path().is_ident("libertas_copy_to")
                    || attr.path().is_ident("libertas_append_to")
                    || attr.path().is_ident("libertas_append_index")
                    || attr.path().is_ident("libertas_validation_rules")
                    || attr.path().is_ident("libertas_range_min")
                    || attr.path().is_ident("libertas_range_max")
                    || attr.path().is_ident("libertas_enum_source")
                    || attr.path().is_ident("libertas_key")
                    || attr.path().is_ident("libertas_bitflags")
                    || attr.path().is_ident("libertas_date_only")
                    || attr.path().is_ident("libertas_format")
                    || attr.path().is_ident("libertas_pattern")
                    || attr.path().is_ident("libertas_money")
                    || attr.path().is_ident("libertas_formatted_text")
                    || attr.path().is_ident("libertas_mime_types")
                    || attr.path().is_ident("libertas_chart")
                    || attr.path().is_ident("libertas_chart_channel")
                    || attr.path().is_ident("libertas_chart_scale")
                    || attr.path().is_ident("libertas_chart_guide")
                    || attr.path().is_ident("libertas_chart3d")
                    || attr.path().is_ident("libertas_chart3d_channel")
                    || attr.path().is_ident("libertas_chart3d_scale")
                    || attr.path().is_ident("libertas_chart3d_guide")
                    || attr.path().is_ident("libertas_chart3d_view")
                    || attr.path().is_ident("libertas_physical_unit")
                    || attr.path().is_ident("libertas_endpoint_schema")
                    || attr.path().is_ident("libertas_endpoint_base_objects")
                    || attr.path().is_ident("libertas_endpoint_server")
                    || attr.path().is_ident("libertas_permissions")
                    || attr.path().is_ident("libertas_access_host")
                    || attr.path().is_ident("libertas_data_schema")
                    || attr.path().is_ident("libertas_default")
                    || attr.path().is_ident("libertas_fixed")
                    || attr.path().is_ident("libertas_device_type")
                    || attr.path().is_ident("libertas_foldable")
                    || attr.path().is_ident("libertas_inline_choices")
                    || attr.path().is_ident("libertas_link")
                    || attr.path().is_ident("libertas_link_only")
                    || attr.path().is_ident("libertas_protocol_variant")
                    || attr.path().is_ident("libertas_request_confirmation")
                    || attr.path().is_ident("libertas_llm_doc_trans")
                    || attr.path().is_ident("libertas_llm_doc_field")
                    || attr.path().is_ident("libertas_cache_identity")
                    || attr.path().is_ident("libertas_suppress_label")
                    || attr.path().is_ident("libertas_ui_include_description")
                    || attr.path().is_ident("libertas_read_only")
                    || attr.path().is_ident("libertas_hidden")
                    || attr.path().is_ident("libertas_exclude_ui")
                    || attr.path().is_ident("libertas_llm_tool_call")
                    || attr.path().is_ident("libertas_number")
                    || attr.path().is_ident("libertas_size")
                    || attr.path().is_ident("libertas_time_interval")
                    || attr.path().is_ident("libertas_enum")
                    || attr.path().is_ident("libertas_unordered")
                    || attr.path().is_ident("libertas_ask_on_add")
                    || attr.path().is_ident("libertas_unique")
                    || attr.path().is_ident("libertas_virtual_device_type")
                    || attr.path().is_ident("content")
                {
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
    // Valid authoring declares one package catalog in leading crate docs:
    // //! #[libertas_string_resources(APP_STRINGS)]
    // Function/type/variant/field placement is invalid. This compiler helper
    // only consumes an item attribute; it does not approve schema placement.
    // The source parser resolves the constant and preserves misplaced metadata
    // so Studio's shared validator can reject it with its source location.
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

/// Declares the comma-delimited DNS hostnames a Libertas function task needs
/// to access. The schema parser extracts the value, and Libertas Studio checks
/// that every hostname has a localized reason in package StringResources.
#[proc_macro_attribute]
pub fn libertas_access_host(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Marks a Libertas function as allowing at most one task on a Hub.
///
/// This is parser-only metadata. The Hub enforces the constraint when a task
/// is created, when its function changes, or when its App version changes.
/// Enables explicit status-returning Data API operations for this task function.
#[proc_macro_attribute]
pub fn libertas_data_error_status(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn libertas_singleton(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn libertas_data_schema(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as data_schema::Arguments);
    let input = parse_macro_input!(item as ItemFn);
    data_schema::expand(args, input).into()
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

/// Suppresses a value row label; on a List of structs, suppresses member names and positions.
/// Presentation metadata only; Studio owns the placement rule.
#[proc_macro_attribute]
pub fn libertas_suppress_label(_attr: TokenStream, item: TokenStream) -> TokenStream {
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

/// Parser-only 3D chart mark/composition metadata. Studio validates it.
#[proc_macro_attribute]
pub fn libertas_chart3d(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Parser-only 3D chart channel metadata. Studio validates it.
#[proc_macro_attribute]
pub fn libertas_chart3d_channel(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Parser-only 3D chart scale metadata. Studio validates it.
#[proc_macro_attribute]
pub fn libertas_chart3d_scale(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Parser-only 3D chart guide metadata. Studio validates it.
#[proc_macro_attribute]
pub fn libertas_chart3d_guide(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Parser-only 3D camera/projection metadata. Studio validates it.
#[proc_macro_attribute]
pub fn libertas_chart3d_view(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Parser-only portable physical-unit metadata. The schema parser validates it.
#[proc_macro_attribute]
pub fn libertas_physical_unit(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Exact i64 money in fixed minor units, with a currency code and scale.
/// Studio validates the metadata; this macro leaves wire values unchanged.
#[proc_macro_attribute]
pub fn libertas_money(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Parser-only Unicode LDML display-format metadata.
/// Studio validates the pattern and its effective host type. This standalone
/// form supports reusable aliases; fields/parameters use the helper paths.
#[proc_macro_attribute]
pub fn libertas_format(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Parser-only calculation root declaration: a comma-delimited list of
/// calculation names on the root node (a struct, enum, type alias, field,
/// enum variant or List member type). On fields and variants it is a
/// `LibertasExport` helper; this standalone form keeps it valid Rust on items.
/// Studio owns placement and rules; this macro leaves the native value unchanged.
#[proc_macro_attribute]
pub fn libertas_calculation_root(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Parser-only numeric type and bounds metadata. On a field it is a
/// `LibertasExport` helper; this standalone form keeps it valid Rust on a
/// numeric type alias. The value is unchanged; Studio owns placement.
#[proc_macro_attribute]
pub fn libertas_number(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Parser-only String, Bytes or List size bounds. On a field it is a
/// `LibertasExport` helper; this standalone form keeps it valid Rust on a type
/// alias. The value is unchanged; Studio owns placement.
#[proc_macro_attribute]
pub fn libertas_size(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Parser-only portable whole-String pattern metadata. Studio owns effective
/// placement and regex validation; this macro leaves the native value unchanged.
#[proc_macro_attribute]
pub fn libertas_pattern(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Requests that clients display the localized long description inline with
/// the short display name. This parser-only metadata does not alter the Rust
/// value or its wire encoding.
#[proc_macro_attribute]
pub fn libertas_ui_include_description(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Allows manual folding of a struct or union in generated clients.
#[proc_macro_attribute]
pub fn libertas_foldable(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Asks clients to render an Enumeration's or Union's permitted choices inline
/// as radio controls. Presentation metadata only; Studio owns the placement rule.
#[proc_macro_attribute]
pub fn libertas_inline_choices(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Names the one Request a tap on this field or named type declaration opens.
/// Parser-only metadata: this macro does not alter the Rust value or its wire
/// encoding, and Studio owns target resolution and placement.
#[proc_macro_attribute]
pub fn libertas_link(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Marks a protocol Request variant as offered only through declared links,
/// never in Actions, menus, landings or automatic singleton selection.
/// Parser-only metadata: this macro does not alter the Rust value or its wire
/// encoding, and Studio owns placement and the link-target rules.
#[proc_macro_attribute]
pub fn libertas_link_only(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Marks a protocol Request variant as critical: before a client sends it on
/// a person's choice or submission, it asks the person to confirm, and the AI
/// assistant asks the user before it calls the generated tool.
/// Parser-only metadata: this macro does not alter the Rust value or its wire
/// encoding, and Studio owns placement and the conflict rules.
#[proc_macro_attribute]
pub fn libertas_request_confirmation(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Declares pure Libertas data validation expressions on a function, named
/// type, field, parameter, or other schema data-type node. Studio owns parsing
/// and semantic validation; this macro deliberately leaves Rust code intact.
#[proc_macro_attribute]
pub fn libertas_validation_rules(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Declares a Data Validation expression whose finite numeric result is the
/// dynamic inclusive minimum for a Number value.
#[proc_macro_attribute]
pub fn libertas_range_min(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Declares a Data Validation expression whose finite numeric result is the
/// dynamic inclusive maximum for a Number value.
#[proc_macro_attribute]
pub fn libertas_range_max(_attr: TokenStream, item: TokenStream) -> TokenStream {
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
///
/// Retired helpers are not accepted merely because the compiler sees this derive.
/// Use `libertas_size` for String/List/Bytes sizes and `libertas_cache_identity`
/// for supported draft identity metadata.
///
/// ```compile_fail
/// use libertas_macros::LibertasExport;
/// #[derive(LibertasExport)]
/// struct Invalid { #[libertas_string(max_length = 4)] value: String }
/// ```
///
/// ```compile_fail
/// use libertas_macros::LibertasExport;
/// #[derive(LibertasExport)]
/// struct Invalid { #[libertas_array(sizeMax = 4)] value: Vec<u32> }
/// ```
///
/// ```compile_fail
/// use libertas_macros::LibertasExport;
/// #[derive(LibertasExport)]
/// struct Invalid { #[libertas_cacheable] value: String }
/// ```
///
/// Retired parameter helpers are not silently consumed by the function wrapper.
///
/// ```compile_fail
/// use libertas_macros::libertas_export;
/// #[libertas_export]
/// fn invalid(#[libertas_string(max_length = 4)] value: String) {}
/// ```
///
/// ```compile_fail
/// use libertas_macros::libertas_export;
/// #[libertas_export]
/// fn invalid(#[libertas_array(sizeMax = 4)] values: Vec<u32>) {}
/// ```
///
/// Retired presentation attributes are rejected on fields, standalone aliases,
/// and function parameters. Author `promote`, `flatten`, `label`, and `title`
/// rules in the App's `display.json` instead.
///
/// ```compile_fail
/// use libertas_macros::LibertasExport;
/// #[derive(LibertasExport)]
/// struct Invalid { #[libertas_ui_header] value: String }
/// ```
///
/// ```compile_fail
/// use libertas_macros::LibertasExport;
/// #[derive(LibertasExport)]
/// struct Invalid { #[libertas_ui_name(amount)] label: String, amount: i64 }
/// ```
///
/// ```compile_fail
/// #[libertas_macros::libertas_ui_header]
/// type Invalid = Vec<u32>;
/// ```
///
/// ```compile_fail
/// #[libertas_macros::libertas_ui_name(amount)]
/// type Invalid = Vec<u32>;
/// ```
///
/// ```compile_fail
/// use libertas_macros::libertas_export;
/// #[libertas_export]
/// fn invalid(#[libertas_ui_header] value: String) {}
/// ```
///
/// ```compile_fail
/// use libertas_macros::libertas_export;
/// #[libertas_export]
/// fn invalid(#[libertas_ui_name(amount)] label: String, amount: i64) {}
/// ```
#[proc_macro_derive(LibertasExport, attributes(
    libertas_calculation_request,
    libertas_calculation_root,
    libertas_workflow_request,
    libertas_prev_request,
    libertas_copy_from,
    libertas_constrained_by,
    libertas_action_control,
    libertas_ai_navigation_prompt,
    libertas_default_request,
    libertas_copy_to,
    libertas_append_to,
    libertas_append_index,
    libertas_validation_rules,
    libertas_range_min,
    libertas_range_max,
    libertas_enum_source,
    // Marks the List member field whose value an EnumSource host stores.
    libertas_key,
    libertas_bitflags,
    libertas_date_only,
    libertas_format,
    libertas_pattern,
    libertas_money,
    libertas_formatted_text,
    libertas_mime_types,
    libertas_endpoint_schema,
    libertas_endpoint_server,
    libertas_endpoint_base_objects,
    libertas_permissions,
    libertas_access_host,
    libertas_foreign_type,
    libertas_request,
    // Restricts which end-user endpoint access levels may invoke this request;
    // the schema parser canonicalizes "Read"/"Write" names to a bitmap.
    libertas_access_privilege,
    libertas_response,
    libertas_error,
    libertas_error_field_path,
    libertas_output_only,
    libertas_subscription_request,
    libertas_subscription_data,
    libertas_default,
    // On an Endpoint client this carries an editable peer-process constraint;
    // it remains parser-only metadata and does not fix the encoded Endpoint ID.
    libertas_fixed,
    libertas_device_type,
    libertas_virtual_device_type,
    libertas_suppress_label,
    libertas_foldable,
    // Renders an Enumeration's or Union's permitted choices inline.
    libertas_inline_choices,
    // Names the Request a tap on this field or named type declaration opens.
    libertas_link,
    // Offers a Request variant only through declared links, not in Actions.
    libertas_link_only,
    // Asks the person to confirm before a client sends this Request variant.
    libertas_request_confirmation,
    libertas_llm_doc_trans,
    libertas_llm_doc_field,
    // Requests that clients render the long description beside the short
    // display name. This is presentation metadata only.
    libertas_ui_include_description,
    libertas_read_only,
    libertas_hidden,
    // Omits a server Endpoint or protocol variant from client-generated UI;
    // runtime protocol behavior and wire encoding remain unchanged.
    libertas_exclude_ui,
    libertas_llm_tool_call,
    libertas_number,
    libertas_size,
    libertas_time_interval,
    libertas_enum,
    libertas_unordered,
    libertas_ask_on_add,
    libertas_unique,
    libertas_next_request,
    libertas_next_response,
    // Associates an ordinary enum member with one endpoint protocol variant.
    libertas_protocol_variant,
    libertas_cache_identity,
    libertas_protocol_conformance,
    libertas_chart,
    libertas_chart_channel,
    libertas_chart_scale,
    libertas_chart_guide,
    libertas_chart3d,
    libertas_chart3d_channel,
    libertas_chart3d_scale,
    libertas_chart3d_guide,
    libertas_chart3d_view,
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
        }
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
        }
        _ => panic!("LibertasAvroEncode only supports structs and enums"),
    };

    let expanded = quote! {
        impl #impl_generics ::libertas::NotBytesEncode for #name #ty_generics #where_clause {}

        impl #impl_generics ::libertas::AvroEncode for #name #ty_generics #where_clause {
            fn avro_encode(&self, buffer: &mut (impl ::libertas::ByteBuffer + ?::core::marker::Sized)) {
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
        }
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
        }
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
