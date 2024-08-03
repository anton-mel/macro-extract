use std::fs;
use std::path::Path;
use syn::{parse_file, Item, ItemFn};
use proc_macro2::TokenStream;
use quote::quote;


pub fn process_files(src_path: &Path, skeleton_path: &Path, macros_folder: &Path) {
    let src_content = fs::read_to_string(src_path).expect("Failed to read source file");
    let skeleton_content = fs::read_to_string(skeleton_path).expect("Failed to read skeleton file");

    let src_file = parse_file(&src_content).expect("Failed to parse source file");
    let skeleton_file = parse_file(&skeleton_content).expect("Failed to parse skeleton file");

    for src_item in &src_file.items {
        if let Item::Fn(src_func) = src_item {
            if let Some(skeleton_func) = find_skeleton_function(&skeleton_file, &src_func.sig.ident) {
                let macros = extract_macros_from_skeleton(&skeleton_func);
                let src_func_body = get_function_body(src_func);
                
                for (macro_name, macro_attr) in macros {
                    let macro_file = macros_folder.join(format!("{}.rs", macro_name));
                    let macro_content = fs::read_to_string(&macro_file).expect("Failed to read macro file");
                    let macro_fn: syn::ItemFn = syn::parse_str(&macro_content).expect("Failed to parse macro function");

                    let processed_body = apply_macro_to_body(&macro_fn, &src_func_body, &macro_attr);
                    println!("Processed function body for macro '{}':\n{}", macro_name, processed_body);
                }
            }
        }
    }
}

fn find_skeleton_function(skeleton_file: &syn::File, func_name: &syn::Ident) -> Option<syn::ItemFn> {
    for item in &skeleton_file.items {
        if let Item::Fn(func) = item {
            if &func.sig.ident == func_name {
                return Some(func.clone());
            }
        }
    }
    None
}

fn extract_macros_from_skeleton(_func: &syn::ItemFn) -> Vec<(String, String)> {
    // Dummy implementation; you should parse the function body for macros
    // e.g., extract 'mutates: field_name' and 'calls: function_name'
    vec![
        ("mutates".to_string(), "field_name".to_string()),
        ("calls".to_string(), "function_name".to_string())
    ]
}

fn get_function_body(func: &ItemFn) -> TokenStream {
    let block = &func.block;
    quote! { #block }
}

fn apply_macro_to_body(_macro_fn: &syn::ItemFn, body: &TokenStream, _macro_attr: &str) -> String {
    // Apply the macro to the function body
    let processed = quote! {
        #body
    };
    processed.to_string()
}
