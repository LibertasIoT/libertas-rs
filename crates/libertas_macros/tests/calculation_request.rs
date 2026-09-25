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
    let order = OrderInputs {
        quantity: 3,
        unit_price: 12345,
    };
    assert_eq!(order.quantity as i64 * order.unit_price, 37035);
}

#[derive(LibertasExport)]
struct CalculatedTotals {
    #[libertas_copy_to("$.items[0].total")]
    total: Option<i64>,
}

#[test]
fn copy_to_preserves_nullable_exact_values() {
    assert_eq!(
        CalculatedTotals {
            total: Some(i64::MAX)
        }
        .total,
        Some(i64::MAX)
    );
    assert_eq!(CalculatedTotals { total: None }.total, None);
}
