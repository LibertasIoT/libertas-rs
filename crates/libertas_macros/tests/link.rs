use libertas_macros::{LibertasExport, libertas_link};
#[derive(LibertasExport)]
#[libertas_link("OpenTopic")]
struct Topic {
    id: i64,
}
#[derive(LibertasExport)]
struct Owner {
    #[libertas_link("OpenTopic")]
    topic: Topic,
    other: Topic,
}
#[libertas_link("OpenAnswer")]
enum StandaloneAnswer {
    Yes,
    No,
}
#[libertas_link("OpenAnswer")]
type AnswerAlias = StandaloneAnswer;
#[test]
fn link_preserves_values() {
    let owner = Owner {
        topic: Topic { id: 7 },
        other: Topic { id: 9 },
    };
    assert_eq!(owner.topic.id, 7);
    assert_eq!(owner.other.id, 9);
    let alias: AnswerAlias = StandaloneAnswer::Yes;
    assert!(matches!(alias, StandaloneAnswer::Yes));
    let _ = (StandaloneAnswer::No, AnswerAlias::Yes);
}
