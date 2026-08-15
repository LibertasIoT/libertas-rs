use libertas_macros::{libertas_export, LibertasExport};

#[derive(LibertasExport)]
struct Dates {
    #[libertas_date_only]
    day: u32,
}

#[libertas_export]
fn configure(#[libertas_date_only] day: u32) -> u32 {
    day
}

#[test]
fn date_only_is_accepted_as_parser_only_metadata() {
    let dates = Dates {
        day: configure(20260815),
    };
    assert_eq!(dates.day, 20260815);
}
