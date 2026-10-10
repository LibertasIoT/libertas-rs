use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, Path, Token, parse::Parse, parse::ParseStream};

pub struct Arguments {
    key: Path,
    value: Path,
}

impl Parse for Arguments {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (mut key, mut value) = (None, None);
        while !input.is_empty() {
            let name: syn::Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let slot = match name.to_string().as_str() {
                "key" => &mut key,
                "value" => &mut value,
                _ => return Err(syn::Error::new_spanned(name, "expected key or value")),
            };
            if slot.is_some() {
                return Err(syn::Error::new_spanned(
                    name,
                    "duplicate data-schema argument",
                ));
            }
            *slot = Some(input.parse()?);
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }
        Ok(Self {
            key: key.ok_or_else(|| input.error("libertas_data_schema requires key"))?,
            value: value.ok_or_else(|| input.error("libertas_data_schema requires value"))?,
        })
    }
}

pub fn expand(args: Arguments, mut function: ItemFn) -> TokenStream {
    let Arguments { key, value } = args;
    function.block.stmts.insert(
        0,
        syn::parse_quote!(::libertas::data::__bind_schema::<#key, #value>();),
    );
    quote! {
        const _: () = {
            fn __libertas_require_codecs<K: ::libertas::data::KeyCodec,
                V: ::libertas::AvroEncode + ::libertas::AvroDecode>() {}
            let _ = __libertas_require_codecs::<#key, #value>;
        };
        #function
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn both_named_types_are_required_exactly_once() {
        for valid in [
            "key=Keys,value=Data",
            "value=module::Data,key=module::Keys,",
        ] {
            assert!(syn::parse_str::<Arguments>(valid).is_ok());
        }
        for invalid in [
            "",
            "Data",
            "key=Keys",
            "value=Data",
            "key=Keys,key=Other,value=Data",
            "key=Keys,value=Data,other=Data",
        ] {
            assert!(syn::parse_str::<Arguments>(invalid).is_err(), "{invalid}");
        }
    }
}
