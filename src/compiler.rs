#![allow(dead_code)]
#![allow(unreachable_code)]

use std::{collections::HashMap, path::Path};
use syn::{parse_file, Item, ItemFn};
use proc_macro2::TokenStream;
use quote::quote;
use std::fs::read_to_string;

/// Processes both the source and macro skeleton files
pub fn process_files(src_path: &Path, skeleton_path: &Path, _macros_folder: &Path) {
    // Read the source and skeleton files
    let src_content = read_to_string(src_path).expect("Failed to read source file");
    let skeleton_content = read_to_string(skeleton_path).expect("Failed to read skeleton file");

    // Parse the source and skeleton files into ASTs
    let _src_ast = parse_file(&src_content).expect("Failed to parse source file");
    let _skeleton_ast = parse_file(&skeleton_content).expect("Failed to parse skeleton file");

    return;

    // Extract items from the source and skeleton
    let src_items = extract_items(_src_ast);
    let skeleton_items = extract_items(_skeleton_ast);

    // Create a map of function names to their corresponding items for easy lookup
    let mut src_functions = HashMap::new();
    let mut skeleton_functions = HashMap::new();

    for item in src_items {
        if let Item::Fn(func) = item {
            src_functions.insert(func.sig.ident.to_string(), func);
        }
    }

    for item in skeleton_items {
        if let Item::Fn(func) = item {
            skeleton_functions.insert(func.sig.ident.to_string(), func);
        }
    }

    // Process each function in the skeleton file
    for (name, skeleton_fn) in skeleton_functions {
        if let Some(src_fn) = src_functions.get(&name) {
            // Check if there's a macro definition for this function
            if let Some(macro_name) = find_macro_definition(&skeleton_fn) {
                // Load the corresponding macro function from the macros folder
                let macro_path = _macros_folder.join(format!("{}.rs", macro_name));
                let macro_content = read_to_string(&macro_path).expect("Failed to read macro file");
                let macro_ast = parse_file(&macro_content).expect("Failed to parse macro file");

                // Find the macro function in the macro AST
                let macro_fn = macro_ast.items.into_iter().find_map(|item| {
                    if let Item::Fn(func) = item {
                        if func.sig.ident == macro_name {
                            Some(func)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }).expect("Macro function not found");

                // Convert the source function body to a TokenStream
                let src_fn_body = &src_fn.block;
                let src_tokens = quote! { #src_fn_body };

                // Call the macro function with the TokenStream
                let result = call_macro_function(&macro_fn, src_tokens);

                // Handle the result
                match result {
                    Ok(_) => println!("Function `{}` passed the macro assertion.", name),
                    Err(e) => println!("Function `{}` failed the macro assertion: {}", name, e),
                }
            }
        } else {
            println!("Function `{}` found in skeleton but not in source.", name);
        }
    }
}

/// Extracts items from the parsed AST
fn extract_items(parsed: syn::File) -> Vec<Item> {
    parsed.items
}

/// Finds a macro definition in a function
fn find_macro_definition(_func: &ItemFn) -> Option<syn::Ident> {
    // This is a placeholder; you need to implement this function to match your macro definitions
    None
}

/// Calls the macro function with the given TokenStream
fn call_macro_function(_macro_fn: &ItemFn, _tokens: TokenStream) -> Result<(), String> {
    // Placeholder implementation: replace with actual macro call
    // You need to implement this based on how you handle macro function calls
    Ok(())
}
