use libertas_macros::LibertasExport;

#[derive(LibertasExport)]
struct Editor {
    #[libertas_hidden]
    allowed: Vec<i32>,
    #[libertas_constrained_by("$.allowed")]
    choice: i32,
}

#[test]
fn constrained_by_preserves_native_values() {
    // Helpers consume source metadata; Studio, not rustc, validates schema hosts.
    let value = Editor { allowed: vec![0, 2], choice: 2 };
    assert!(value.allowed.contains(&value.choice));
}
