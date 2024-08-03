// TODO: Expand the implementation

use syn::{
    Item, ItemFn, ItemMod, ItemStruct, 
    ItemImpl, ImplItem, ImplItemFn};
    use proc_macro2::TokenStream;
use std::path::Path;
use std::io::Write;
use prettyplease;
use quote::quote;
use std::fs;


pub fn create_skeleton(path: &Path) {
    let content = fs::read_to_string(path).expect("Failed to read file");
    let parsed_file = syn::parse_file(&content).expect("Failed to parse file");
    let mut output = TokenStream::new();

    for item in &parsed_file.items {
        match item {
            Item::Fn(func) => {
                let func_skeleton = extract_function_skeleton(func);
                output.extend(func_skeleton);
            }
            Item::Struct(s) => {
                let struct_skeleton = extract_struct_skeleton(s);
                output.extend(struct_skeleton);
            }
            Item::Impl(impl_block) => {
                let impl_skeleton = extract_impl_skeleton(impl_block);
                output.extend(impl_skeleton);
            }
            Item::Mod(module) => {
                let mod_skeleton = extract_mod_skeleton(module);
                output.extend(mod_skeleton);
            }
            _ => {}
        }
    }

    // Format the TokenStream to a pretty-printed string
    let formatted_output = prettyplease::unparse(&syn::parse2::<syn::File>(quote! { #output }).expect("Failed to parse TokenStream"));

    // Create the .macros file path
    let mut macros_path = path.to_path_buf();
    macros_path.set_extension("macros");

    // Write the skeleton to the .macros file
    let mut file = fs::File::create(&macros_path).expect("Failed to create file");
    file.write_all(formatted_output.as_bytes()).expect("Failed to write to file");
}



// Extract function skeleton
fn extract_function_skeleton(func: &ItemFn) -> TokenStream {
    let sig = &func.sig;
    let fn_name = &sig.ident;
    let inputs = &sig.inputs;

    // Construct the function signature without the body
    let skeleton = quote! {
        fn #fn_name(#inputs) {}
    };

    skeleton
}

// Extract struct skeleton
fn extract_struct_skeleton(struct_item: &ItemStruct) -> TokenStream {
    let struct_name = &struct_item.ident;
    
    // Construct the struct without fields
    let skeleton = quote! {
        pub struct #struct_name {}
    };

    skeleton
}

// Extract impl block skeleton
fn extract_impl_skeleton(impl_block: &ItemImpl) -> TokenStream {
    let impl_target = &impl_block.self_ty;
    let mut impl_items: Vec<TokenStream> = Vec::new();

    for item in &impl_block.items {
        match item {
            ImplItem::Fn(func) => {
                let func_skeleton = extract_impl_function_skeleton(func);
                impl_items.push(func_skeleton);
            }
            _ => {}
        }
    }

    let impl_skeleton = quote! {
        impl #impl_target {
            #(#impl_items)*
        }
    };

    impl_skeleton
}

// Extract function skeleton within impl block
fn extract_impl_function_skeleton(func: &ImplItemFn) -> TokenStream {
    let sig = &func.sig;
    let fn_name = &sig.ident;
    let inputs = &sig.inputs;

    // Construct the function signature without the body
    let skeleton = quote! {
        fn #fn_name(#inputs) {}
    };

    skeleton
}

// Extract module skeleton
fn extract_mod_skeleton(module: &ItemMod) -> TokenStream {
    let mut mod_items: Vec<TokenStream> = Vec::new();

    for item in &module.content.clone().unwrap_or_default().1 {
        match item {
            Item::Fn(func) => {
                let func_skeleton = extract_function_skeleton(func);
                mod_items.push(func_skeleton);
            }
            Item::Mod(sub_mod) => {
                let sub_mod_skeleton = extract_mod_skeleton(sub_mod);
                mod_items.push(sub_mod_skeleton);
            }
            Item::Struct(s) => {
                let struct_skeleton = extract_struct_skeleton(s);
                mod_items.push(struct_skeleton);
            }
            Item::Impl(impl_block) => {
                let impl_skeleton = extract_impl_skeleton(impl_block);
                mod_items.push(impl_skeleton);
            }
            _ => {}
        }
    }

    let mod_skeleton = quote! {
        mod #module.ident {
            #(#mod_items)*
        }
    };

    mod_skeleton
}
