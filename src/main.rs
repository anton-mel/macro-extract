use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use syn::{parse_file, Attribute, ItemFn, ItemStruct, ItemImpl};
use syn::visit::{self, Visit};
use syn::__private::ToTokens;
use std::collections::HashMap;


struct MacroExtractor {
    item_macros: HashMap<String, HashMap<String, Vec<String>>>,
    current_item: Option<String>,
}

impl<'ast> Visit<'ast> for MacroExtractor {
    fn visit_attribute(&mut self, attr: &'ast Attribute) {
        let macro_name = if let Some(last_segment) = attr.path().segments.last() {
            last_segment.ident.to_string()
        } else {
            "unknown".to_string()
        };

        let macro_str = attr.to_token_stream().to_string();
        let trimmed_contents = trim_macro_contents(&macro_str);

        if let Some(ref item_name) = self.current_item {
            self.item_macros
                .entry(item_name.clone())
                .or_insert_with(HashMap::new)
                .entry(macro_name)
                .or_insert_with(Vec::new)
                .push(trimmed_contents);
        }
        visit::visit_attribute(self, attr);
    }

    fn visit_item_struct(&mut self, item: &'ast ItemStruct) {
        let struct_name = format!("struct {}", item.ident);
        self.current_item = Some(struct_name.clone());
        visit::visit_item_struct(self, item);
        self.current_item = None; // Reset after processing
    }

    fn visit_item_fn(&mut self, item: &'ast ItemFn) {
        let fn_name = format!("fn {}", item.sig.ident.to_string());
        self.current_item = Some(fn_name.clone());
        visit::visit_item_fn(self, item);
        self.current_item = None; // Reset after processing
    }

    fn visit_item_impl(&mut self, item: &'ast ItemImpl) {
        // Process items inside the impl block
        for impl_item in &item.items {
            if let syn::ImplItem::Fn(fn_item) = impl_item {
                let fn_name = format!("fn {}", fn_item.sig.ident.to_string());
                self.current_item = Some(fn_name.clone());
                visit::visit_impl_item_fn(self, fn_item);
                self.current_item = None; // Reset after processing each impl method
            }
        }
        visit::visit_item_impl(self, item); // Visit the impl block itself if necessary
    }
}

fn trim_macro_contents(macro_str: &str) -> String {
    let start_idx = macro_str.find('(').unwrap_or(0) + 1;
    let end_idx = macro_str.rfind(')').unwrap_or(macro_str.len());
    let contents = &macro_str[start_idx..end_idx];
    contents.trim().replace(", ", ",")
}

fn main() -> std::io::Result<()> {
    let input_path = std::env::args().nth(1).expect("No input file path provided");
    let input_file = std::fs::read_to_string(&input_path).expect("Failed to read input file");

    // Parse the input file into a syntax tree
    let ast = parse_file(&input_file).expect("Failed to parse file");

    // Collect attributes
    let mut extractor = MacroExtractor { 
        item_macros: HashMap::new(), 
        current_item: None 
    };
    extractor.visit_file(&ast);

    // Write the extracted macros to a file
    let output_path = Path::new(&input_path).with_extension("macros");
    let mut output_file = File::create(output_path)?;

    for (item_name, macros) in extractor.item_macros {
        writeln!(output_file, "{} {{", item_name)?;
        for (macro_name, attrs) in macros {
            let attr_list = attrs.join(", ");
            writeln!(output_file, "   {}: {}", macro_name, attr_list)?;
        }
        writeln!(output_file, "}}\n")?;
    }

    Ok(())
}
