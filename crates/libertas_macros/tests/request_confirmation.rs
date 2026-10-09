use libertas_macros::LibertasExport;
#[derive(LibertasExport)]
struct Row {
    id: i64,
}
#[derive(LibertasExport)]
enum Protocol {
    State {
        rows: Vec<Row>,
    },
    #[libertas_request_confirmation]
    EraseAll,
    #[libertas_request_confirmation]
    Remove(Row),
}
#[test]
fn request_confirmation_preserves_values() {
    let state = Protocol::State {
        rows: vec![Row { id: 7 }],
    };
    assert!(matches!(&state, Protocol::State { rows } if rows[0].id == 7));
    assert!(matches!(Protocol::EraseAll, Protocol::EraseAll));
    assert!(matches!(
        Protocol::Remove(Row { id: 9 }),
        Protocol::Remove(Row { id: 9 })
    ));
}
