use libertas_macros::{LibertasExport, libertas_export, libertas_pattern};

#[allow(dead_code)]
const DIGITS: &str = r"\d+";

#[libertas_pattern($DIGITS)]
type Account = String;

#[derive(LibertasExport)]
struct Values {
    #[libertas_pattern(r#"["']+,) @ {};"#)]
    quoted: String,
    #[libertas_pattern("\\d+")]
    account: Account,
}

#[derive(LibertasExport)]
enum Choice {
    Value {
        #[libertas_pattern($DIGITS)]
        account: Account,
    },
}

#[libertas_export]
fn configure(#[libertas_pattern($DIGITS)] account: Account) -> Values {
    Values {
        quoted: "\"',) @ {};".into(),
        account,
    }
}

#[test]
fn pattern_is_parser_only_metadata_for_aliases_fields_variants_and_parameters() {
    let values = configure("00123".into());
    assert_eq!(values.account, "00123");
    assert_eq!(values.quoted, "\"',) @ {};");
    let Choice::Value { account } = Choice::Value {
        account: "0009".into(),
    };
    assert_eq!(account, "0009");
    // The SDK must not execute regex validation or coerce native String values.
    assert_eq!(configure("not digits".into()).account, "not digits");
}
