use libertas_macros::{LibertasExport, libertas_number, libertas_size};

#[libertas_number(min = 1900, max = 2100)]
type Year = u32;

#[libertas_size(max = 64)]
type Name = String;

#[derive(LibertasExport)]
struct Record {
    #[libertas_number(min = 2000, max = 2099)]
    year: Year,
    #[libertas_size(min = 1)]
    name: Name,
}

#[test]
fn bound_aliases_and_bound_fields_compile() {
    let record = Record { year: 2026, name: "x".into() };
    assert_eq!((record.year, record.name.len()), (2026, 1));
}
