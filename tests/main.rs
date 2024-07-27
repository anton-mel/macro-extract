

#[allow(dead_code)]
struct MyStruct {
    field: i32,
}

impl MyStruct {
    #[allow(deprecated)]
    #[allow(dead_code)]
    #[deny(warnings)]
    fn my_method(&self) {
        // method implementation
    }
}

#[allow(dead_code)] 
fn standalone_function() {
    // function implementation
}
