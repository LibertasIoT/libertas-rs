use libertas_macros::{LibertasExport, libertas_dynamic, libertas_export};

#[derive(LibertasExport)]
#[libertas_dynamic(r#"Editable = [status] == "draft";"#)]
struct Item {
    status: &'static str,
    #[libertas_dynamic("Editable = false;")]
    amount: u32,
}

#[libertas_dynamic("Editable = true;")]
type Amount = u32;

#[libertas_export]
fn configure(#[libertas_dynamic("Editable = true;")] amount: Amount) -> u32 {
    amount
}

#[test]
fn dynamic_metadata_preserves_native_types_and_values() {
    let item = Item { status: "draft", amount: 42 };
    assert_eq!(item.status, "draft");
    assert_eq!(configure(item.amount), 42);
}
