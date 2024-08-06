struct TestStruct1 {}

impl TestStruct1 {
    #[deprecated]
    #[allow(dead_code)]
    #[deny(warnings)]
    fn method1(&self) {
        // method implementation
    }
}
