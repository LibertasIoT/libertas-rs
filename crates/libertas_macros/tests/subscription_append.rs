use libertas_macros::LibertasExport;

#[derive(LibertasExport)]
struct MoreEntries {
    #[libertas_append_to("Entries.entries")]
    #[libertas_append_index("$.start_index")]
    added_entries: Vec<Option<String>>,
    start_index: u64,
}

#[test]
fn append_annotations_preserve_payload_and_index() {
    let report = MoreEntries { added_entries: vec![Some("new".into()), None], start_index: u64::MAX };
    assert_eq!(report.added_entries.len(), 2);
    assert_eq!(report.added_entries[1], None);
    assert_eq!(report.start_index, u64::MAX);
}
