# macro-extract

`macro-extract` is a helper Rust verification tool that extracts and formats macro attributes applied to functions, structs, and implementations.

**Example Input:**

```rust
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
```

## Output

**Example Output:**

```
fn standalone_function {
   allow: dead_code
}

impl my_method {
   allow: deprecated, dead_code
   deny: warnings
}

struct MyStruct {
   allow: dead_code
}
```

## Installation

To install `macro-extract`, you can use Cargo to build it from source.

```sh
cargo build --release
```

## Usage

Run `macro-extract` with the path to your Rust source file as an argument.

```sh
./target/release/macro-extract path/to/your_file.rs
```

The output will be saved in the same directory as the input file, with a `.macros` extension.

## License

`macro-extract` is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.
