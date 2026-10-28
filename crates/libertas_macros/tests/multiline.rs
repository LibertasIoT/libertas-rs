use libertas_macros::{LibertasExport, libertas_export, libertas_multiline};

#[libertas_multiline]
type Note = String;

#[derive(LibertasExport)]
struct Notes {
    #[libertas_multiline]
    plain: String,
    named: Note,
    #[libertas_multiline]
    optional: Option<String>,
    /// Notes
    /// ----
    #[libertas_multiline]
    entries: Vec<String>,
}

#[derive(LibertasExport)]
enum Entry {
    Value {
        #[libertas_multiline]
        note: String,
    },
}

#[libertas_export]
fn note(#[libertas_multiline] value: Note) -> Note {
    value
}

#[test]
fn multiline_helpers_preserve_strings_for_aliases_fields_variants_and_parameters() {
    let text = "first\nsecond\nthird";
    let notes = Notes {
        plain: note(text.into()),
        named: text.into(),
        optional: Some(text.into()),
        entries: vec![text.into()],
    };
    assert_eq!(notes.plain, text);
    assert_eq!(notes.named, text);
    assert_eq!(notes.optional.as_deref(), Some(text));
    assert_eq!(notes.entries, [text]);
    let Entry::Value { note } = Entry::Value { note: text.into() };
    assert_eq!(note, text);
}
