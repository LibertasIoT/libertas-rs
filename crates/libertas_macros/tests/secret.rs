use libertas_macros::{LibertasExport, libertas_export, libertas_secret};

#[libertas_secret]
type Password = String;

#[derive(LibertasExport)]
struct Login {
    #[libertas_secret]
    plain: String,
    named: Password,
    #[libertas_secret]
    optional: Option<String>,
    /// Tokens
    /// ----
    #[libertas_secret]
    tokens: Vec<String>,
}

#[derive(LibertasExport)]
enum Credential {
    Pin {
        #[libertas_secret]
        pin: String,
    },
}

#[libertas_export]
fn check(#[libertas_secret] value: Password) -> Password {
    value
}

#[test]
fn secret_helpers_preserve_strings_for_aliases_fields_variants_and_parameters() {
    let text = "s3cret";
    let login = Login {
        plain: check(text.into()),
        named: text.into(),
        optional: Some(text.into()),
        tokens: vec![text.into()],
    };
    assert_eq!(login.plain, text);
    assert_eq!(login.named, text);
    assert_eq!(login.optional.as_deref(), Some(text));
    assert_eq!(login.tokens, [text]);
    let Credential::Pin { pin } = Credential::Pin { pin: text.into() };
    assert_eq!(pin, text);
}
