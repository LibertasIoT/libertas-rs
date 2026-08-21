use libertas_macros::{libertas_export, libertas_singleton};

#[libertas_export]
#[libertas_singleton]
fn singleton_task_entry() -> u32 {
    1
}

#[test]
fn singleton_attribute_preserves_the_function() {
    assert_eq!(singleton_task_entry(), 1);
}
