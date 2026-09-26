use libertas_macros::LibertasExport;
#[derive(LibertasExport)]
struct Row {
    id: i64,
}
#[derive(LibertasExport)]
enum Protocol {
    Hub {
        #[libertas_link("OpenRow")]
        rows: Vec<Row>,
    },
    #[libertas_link_only]
    OpenRow,
    Other(Row),
}
#[test]
fn link_only_preserves_values() {
    let hub = Protocol::Hub {
        rows: vec![Row { id: 7 }],
    };
    assert!(matches!(&hub, Protocol::Hub { rows } if rows[0].id == 7));
    assert!(matches!(Protocol::OpenRow, Protocol::OpenRow));
    assert!(matches!(Protocol::Other(Row { id: 9 }), Protocol::Other(Row { id: 9 })));
}
