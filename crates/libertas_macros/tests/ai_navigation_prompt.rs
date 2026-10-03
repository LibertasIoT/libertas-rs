use libertas_macros::LibertasExport;

#[derive(LibertasExport)]
#[libertas_ai_navigation_prompt("UNION_NAV")]
enum Protocol {
    #[libertas_request]
    #[libertas_ai_navigation_prompt("PAGE_NAV")]
    Enter,
}

#[test]
fn navigation_guidance_preserves_protocol_values() {
    assert!(matches!(Protocol::Enter, Protocol::Enter));
}
