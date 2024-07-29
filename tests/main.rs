// Attributes applied to the module and various items
#[allow(dead_code)] // Applied to the entire module

// A struct with attributes
#[derive(Debug)] // Rust-provided macro
#[allow(non_snake_case)] // Custom attribute
struct MyStruct {
    #[allow(unused)] // Attribute on field
    field: i32,
}

// Implementation block with nested attributes
impl MyStruct {
    #[deprecated] // Custom attribute
    #[allow(dead_code)]
    #[deny(warnings)]
    fn my_method(&self) {
        // method implementation
    }
}

// Another implementation block for MyStruct with different attributes
impl MyStruct {
    #[allow(unused_variables)] // Custom attribute
    fn another_method(&self) {
        let x = 42;
        #[allow(unused_assignments)] // Custom attribute
        let y = 0;
        // method implementation
    }
}

// A function with various attributes
#[deny(warnings)] // Custom attribute
#[warn(unreachable_code)] // Custom attribute
fn standalone_function() {
    // function implementation
}

// A struct with various attributes and an implementation block
#[cfg(test)] // Conditional compilation attribute
#[allow(dead_code)] // Custom attribute
struct TestStruct {
    value: i32,
}

impl TestStruct {
    #[cfg(test)] // Conditional compilation attribute
    #[allow(unused_variables)] // Custom attribute
    fn test_method(&self) {
        let temp = 0;
        // method implementation
    }
}

// An implementation of a trait for TestStruct with attributes
impl std::fmt::Debug for TestStruct {
    #[allow(unused_variables)] // Custom attribute
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // implementation
        write!(f, "TestStruct {{ value: {} }}", self.value)
    }
}

// A function with various attributes and an implementation block
#[deny(missing_docs)] // Custom attribute
#[allow(dead_code)] // Custom attribute
fn another_function() {
    // function implementation
}

// Example of an empty impl block with attributes
impl std::ops::Add for TestStruct {
    type Output = Self;

    #[allow(clippy::ptr_arg)] // Custom attribute
    fn add(self, rhs: Self) -> Self::Output {
        TestStruct {
            value: self.value + rhs.value,
        }
    }
}
