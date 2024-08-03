use syn::{
    Block, Item, ItemFn, ItemImpl, ItemMod, Stmt, FnArg, Pat, PatIdent, Type, TypePath, Expr, ExprCall, 
    Member, ExprField, ExprPath, ExprAssign, ExprBinary, ExprBlock, ExprIf, ExprWhile, ExprForLoop, 
    punctuated::Punctuated, token::Comma, ImplItemFn,
};
use quote::quote;
use proc_macro2::TokenStream;
use proc_macro2::TokenStream as ProcTokenStream;
use std::collections::HashSet;

pub fn create_skeleton(input: &syn::File) -> ProcTokenStream {
    let mut output = TokenStream::new();

    for item in &input.items {
        match item {
            Item::Fn(func) => {
                let func_skeleton = extract_function_skeleton(func);
                output.extend(func_skeleton);
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

    output
}

fn extract_function_skeleton(func: &ItemFn) -> ProcTokenStream {
    let sig = &func.sig;
    let fn_name = &sig.ident;
    let inputs = &sig.inputs;

    // Construct the function signature without the body
    let skeleton = quote! {
        fn #fn_name(#inputs) {}
    };

    skeleton
}

fn extract_impl_skeleton(impl_block: &ImplItemFn) -> ProcTokenStream {
    let impl_items: Vec<ProcTokenStream> = impl_block.items.iter()
        .filter_map(|item| {
            if let syn::ImplItem::Fn(func) = item {
                let func_skeleton = extract_function_skeleton(func);
                Some(func_skeleton)
            } else {
                None
            }
        })
        .collect();

    let impl_skeleton = quote! {
        impl #impl_block.self_ty {
            #(#impl_items)*
        }
    };

    impl_skeleton
}

fn extract_mod_skeleton(module: &ItemMod) -> ProcTokenStream {
    let mut mod_items: Vec<ProcTokenStream> = Vec::new();

    for item in &module.content.clone().unwrap_or_default().1 {
        match item {
            Item::Fn(func) => {
                let func_skeleton = extract_function_skeleton(func);
                mod_items.push(func_skeleton);
            }
            Item::Impl(impl_block) => {
                let impl_skeleton = extract_impl_skeleton(impl_block);
                mod_items.push(impl_skeleton);
            }
            Item::Mod(sub_mod) => {
                let sub_mod_skeleton = extract_mod_skeleton(sub_mod);
                mod_items.push(sub_mod_skeleton);
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
