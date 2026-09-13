use libertas_macros::LibertasExport;

#[derive(LibertasExport)]
struct OrderInputs {
    #[libertas_calculation_request("Price,Shipping")]
    quantity: u32,
    #[libertas_calculation_request("Price")]
    unit_price: i64,
}

#[test]
fn field_annotations_preserve_native_representation() {
    let order = OrderInputs { quantity: 3, unit_price: 12345 };
    assert_eq!(order.quantity as i64 * order.unit_price, 37035);
}
