extern crate kg_display;
#[macro_use]
extern crate kg_display_derive;


use kg_display::ListDisplay;

#[derive(Debug, Display)]
enum TestEnum {
    #[display(fmt = "struct with fields - field1: {field1}.")]
    Struct {
        field1: String,
        field2: usize,
    },
    #[display("tuple with fields - {_0}, {_2}")]
    Tuple(String, usize, bool),
    #[display("plain")]
    Plain,
    #[display(fmt = "elements: {elems}", elems = "ListDisplay(&_0)")]
    Elems(Vec<String>),
}

#[derive(Debug, Display)]
#[display(fmt = "test struct with fields - field1: {field1}.")]
struct TestStruct {
    field1: String,
    field2: usize,
}

#[test]
fn display_enum_struct_variant() {
    let e = TestEnum::Struct {
        field1: "string1".into(),
        field2: 1024,
    };

    assert_eq!("struct with fields - field1: string1.", format!("{}", e));
}

#[test]
fn display_enum_tuple_variant() {
    let e = TestEnum::Tuple("string1".into(),1024, false);

    assert_eq!("tuple with fields - string1, false", format!("{}", e));
}

#[test]
fn display_enum_plain_variant() {
    let e = TestEnum::Plain;

    assert_eq!("plain", format!("{}", e));
}

#[test]
fn display_enum_custom_param_variant() {
    let e = TestEnum::Elems(vec!["one".into(), "two".into(), "three".into(), "four".into()]);

    assert_eq!("elements: one, two, three, four", format!("{}", e));
}

#[test]
fn display_struct() {
    let e = TestStruct {
        field1: "string1".into(),
        field2: 1024,
    };

    assert_eq!("test struct with fields - field1: string1.", format!("{}", e));
}
