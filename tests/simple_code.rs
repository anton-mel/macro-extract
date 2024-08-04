pub struct Data {
    values: String,
}

impl Data {
    pub fn new(values: &str) -> Data {
        Data {
            values: values.to_string(),
        }
    }

    pub fn print_values(&self) {
        println!("Data values: {}", self.values);
    }
}

fn main() {
    let data = Data::new("Hello, World!");  
    data.print_values();
}
