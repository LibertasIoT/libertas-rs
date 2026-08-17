use libertas_macros::{VariantIndex, variant_index};

struct TypeA;
struct TypeB;
struct TypeC;

#[allow(dead_code)]
#[derive(VariantIndex)]
enum T {
    A(TypeA),
    B(TypeB),
    C(TypeC),
}

const I: usize = variant_index!(T::B);
const TABLE: [u32; 3] = [10, 20, 30];
const B_VALUE: u32 = TABLE[variant_index!(T::B)];

#[test]
fn returns_the_declaration_index_without_constructing_the_payload() {
    assert_eq!(I, 1);
    assert_eq!(B_VALUE, 20);
}

#[allow(dead_code)]
#[derive(VariantIndex)]
enum EveryVariantShape {
    Unit,
    Tuple(u32),
    Struct { value: u32 },
}

#[test]
fn supports_every_variant_shape() {
    const UNIT: usize = variant_index!(EveryVariantShape::Unit);
    const TUPLE: usize = variant_index!(EveryVariantShape::Tuple);
    const STRUCT: usize = variant_index!(EveryVariantShape::Struct);

    assert_eq!([UNIT, TUPLE, STRUCT], [0, 1, 2]);
}

mod qualified {
    use libertas_macros::VariantIndex;

    #[allow(dead_code)]
    #[derive(VariantIndex)]
    pub enum Message {
        Connect,
        Data(u32),
        Disconnect,
    }
}

#[test]
fn supports_qualified_and_absolute_paths() {
    const QUALIFIED: usize = variant_index!(qualified::Message::Data);
    const ABSOLUTE: usize = variant_index!(crate::qualified::Message::Disconnect);

    assert_eq!(QUALIFIED, 1);
    assert_eq!(ABSOLUTE, 2);
}

#[allow(dead_code)]
#[derive(VariantIndex)]
enum GenericMessage<T>
where
    T: Clone,
{
    Empty,
    Data(T),
}

#[test]
fn supports_generic_enum_paths_and_where_clauses() {
    const EMPTY: usize = variant_index!(GenericMessage::<String>::Empty);
    const DATA: usize = variant_index!(GenericMessage::<String>::Data);

    assert_eq!(EMPTY, 0);
    assert_eq!(DATA, 1);
}
