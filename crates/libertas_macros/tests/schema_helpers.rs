use libertas_macros::{LibertasExport, libertas_data_schema, libertas_export};

#[derive(LibertasExport)]
struct Bounds {
    #[libertas_size(min = 1, max = 8)]
    label: String,
    #[libertas_size(min = 1, max = 4)]
    entries: Vec<u8>,
    #[libertas_number(min = 0, max = 10)]
    count: u32,
}

enum Stored {
    Record(u32),
}

#[libertas_data_schema(Stored)]
fn store() -> Stored {
    Stored::Record(3)
}

#[libertas_export]
fn configure(#[libertas_size(min = 1, max = 8)] label: String) -> String {
    label
}

#[test]
fn supported_schema_helpers_preserve_native_values() {
    let values = Bounds {
        label: configure("text".into()),
        entries: vec![1],
        count: 3,
    };
    assert_eq!(
        (values.label.as_str(), values.entries.len(), values.count),
        ("text", 1, 3)
    );
    let Stored::Record(value) = store();
    assert_eq!(value, 3);
}
