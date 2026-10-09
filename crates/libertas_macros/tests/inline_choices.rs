use libertas_macros::{LibertasExport, libertas_inline_choices};
#[derive(LibertasExport)]
#[libertas_inline_choices]
enum Answer {
    Yes,
    No,
    NotSure,
}
#[derive(LibertasExport)]
struct Amount {
    value: i64,
}
#[derive(LibertasExport)]
enum Other {
    Nothing,
    Something(Amount),
}
#[derive(LibertasExport)]
struct Question {
    #[libertas_inline_choices]
    other: Other,
    answer: Answer,
}
#[libertas_inline_choices]
enum StandaloneAnswer {
    Yes,
    No,
}
#[libertas_inline_choices]
type AnswerAlias = Answer;
#[test]
fn inline_choices_preserves_values() {
    let question = Question {
        other: Other::Something(Amount { value: 7 }),
        answer: Answer::NotSure,
    };
    match question.other {
        Other::Something(amount) => assert_eq!(amount.value, 7),
        Other::Nothing => panic!(),
    }
    assert!(matches!(question.answer, Answer::NotSure));
    let alias: AnswerAlias = Answer::Yes;
    assert!(matches!(alias, Answer::Yes));
    let _ = (
        Answer::No,
        Other::Nothing,
        StandaloneAnswer::Yes,
        StandaloneAnswer::No,
    );
}
