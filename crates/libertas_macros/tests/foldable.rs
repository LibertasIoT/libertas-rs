use libertas_macros::{LibertasExport, libertas_foldable};
#[derive(LibertasExport)]
#[libertas_foldable]
struct Group {
    value: i32,
}
#[derive(LibertasExport)]
#[libertas_foldable]
enum Choice {
    Empty,
    Filled(Group),
}
#[derive(LibertasExport)]
struct Owner {
    #[libertas_foldable]
    choice: Choice,
}
#[test]
fn foldable_preserves_values() {
    let owner = Owner {
        choice: Choice::Filled(Group { value: 3 }),
    };
    match owner.choice {
        Choice::Filled(group) => assert_eq!(group.value, 3),
        _ => panic!(),
    }
    let _ = Choice::Empty;
}
