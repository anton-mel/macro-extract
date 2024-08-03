use crate::inner_module::InnerStruct;

pub mod inner_module {
    pub struct InnerStruct {
        pub field_b: u32,
    }

    impl InnerStruct {
        pub fn new(field_b: u32) -> Self {
            InnerStruct { field_b }
        }

        pub fn method_b(&self) -> u32 {
            self.field_b
        }
    }
}

pub struct OuterStruct {
    pub field_a: String,
}

impl OuterStruct {
    pub fn new(field_a: String) -> Self {
        OuterStruct { field_a }
    }

    pub fn method_a(&self) -> &str {
        &self.field_a
    }
}

pub trait ExampleTrait {
    fn trait_method(&self) -> String;
}

impl ExampleTrait for OuterStruct {
    fn trait_method(&self) -> String {
        format!("Trait method with field_a: {}", self.field_a)
    }
}

impl ExampleTrait for InnerStruct {
    fn trait_method(&self) -> String {
        format!("Trait method with field_b: {}", self.field_b)
    }
}

fn main() {
    let outer = OuterStruct::new("Hello".to_string());
    println!("{}", outer.trait_method());

    let inner = InnerStruct::new(100);
    println!("{}", inner.trait_method());
}
