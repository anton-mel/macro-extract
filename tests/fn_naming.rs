#[allow(dead_code)]
struct TestStruct2 {
    value: u8
}

impl std::fmt::Debug for TestStruct2 {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // implementation
        write!(f, "TestStruct2 {{ value: {} }}", self.value)
    }
}
