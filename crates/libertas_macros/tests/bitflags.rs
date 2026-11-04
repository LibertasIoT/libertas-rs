use bitflags::bitflags;
use libertas_macros::{LibertasExport, libertas_bitflags, libertas_form_code};

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[libertas_bitflags]
    struct Access: u8 {
        #[libertas_form_code("R")]
        const READ = 1 << 0;
        #[libertas_form_code("W")]
        const WRITE = 1 << 1;
    }
}

#[libertas_bitflags]
struct ManualAccess(u16);

impl ManualAccess {
    #[libertas_form_code("X")]
    const EXECUTE: Self = Self(1 << 2);
}

#[derive(LibertasExport)]
struct Settings {
    #[libertas_bitflags(Access)]
    access: u8,
    #[libertas_bitflags(ManualAccess)]
    manual_access: u16,
}

#[test]
fn bitflags_annotations_do_not_change_rust_values() {
    let settings = Settings {
        access: (Access::READ | Access::WRITE).bits(),
        manual_access: ManualAccess::EXECUTE.0,
    };
    assert_eq!(settings.access, 3);
    assert_eq!(settings.manual_access, 4);
}

#[derive(LibertasExport)]
enum Filing {
    #[libertas_form_code("F")]
    ByAgreement,
}

#[test]
fn form_code_helper_and_attribute_coexist() {
    assert!(matches!(Filing::ByAgreement, Filing::ByAgreement));
}
