#[allow(dead_code)]
#[derive(Debug, Clone)]
#[allow(non_snake_case, unused_imports)]
struct ComplexStruct {
    #[cfg(feature = "experimental")]
    #[allow(unused)]
    field: i32,
    #[deny(warnings)]
    #[allow(dead_code)]
    second_field: String,
}
