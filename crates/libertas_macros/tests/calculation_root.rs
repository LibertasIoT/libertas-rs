use libertas_macros::{LibertasExport, libertas_calculation_root, libertas_export};

// Item placement: struct, enum, and type alias, as a standalone attribute.
#[libertas_calculation_root("CalcA,CalcB")]
struct StandaloneStruct(u32);

#[libertas_calculation_root("CalcA")]
enum StandaloneEnum {
    One,
}

#[libertas_calculation_root("CalcA")]
type StandaloneAlias = Vec<u32>;

// Item placement together with the derive: the helper follows the derive.
#[derive(LibertasExport)]
#[libertas_calculation_root("CalcA")]
struct DerivedItem {
    // Field placement: LibertasExport helper.
    #[libertas_calculation_root("CalcA,CalcB")]
    row: u32,
}

// Variant placement: LibertasExport helper.
#[derive(LibertasExport)]
enum Variants {
    #[libertas_calculation_root("CalcA")]
    Edit { value: u32 },
    #[libertas_calculation_root("CalcB")]
    Other(u32),
}

// Function parameter placement, mirroring libertas_calculation_request.
#[libertas_export]
fn configure(#[libertas_calculation_root("CalcA")] row: u32) -> u32 {
    row
}

#[test]
fn calculation_root_preserves_native_representation() {
    assert_eq!(StandaloneStruct(3).0, 3);
    assert!(matches!(StandaloneEnum::One, StandaloneEnum::One));
    let alias: StandaloneAlias = vec![1];
    assert_eq!(alias, [1]);
    assert_eq!(DerivedItem { row: 5 }.row, 5);
    assert!(matches!(Variants::Edit { value: 7 }, Variants::Edit { value: 7 }));
    assert!(matches!(Variants::Other(1), Variants::Other(1)));
    assert_eq!(configure(9), 9);
}
