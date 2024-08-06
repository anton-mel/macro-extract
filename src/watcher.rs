use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::{fs, path::{Path, PathBuf}, sync::mpsc};
use syn::{parse_file, Attribute, ItemFn, ItemStruct, ItemImpl, ItemEnum, Variant};
use std::collections::HashMap;
use syn::visit::{self, Visit};
use syn::__private::ToTokens;
use std::io::Write;

static PATH: &str = "tests";

fn get_macros_path(src_path: &Path) -> PathBuf {
    src_path.with_extension("macros")
}

/// Handle file changes
fn handle_changes(event: Event) {
    if let Some(path) = event.paths.first() {
        if path.extension().map_or(false, |ext| ext == "rs") {
            let macros_path = get_macros_path(path);
            match event.kind {
                notify::event::EventKind::Create(_) => {
                    // println!("DEBUG: File created: {:?}", path);
                    if !macros_path.exists() {
                        fs::File::create(macros_path).expect("Failed to create file");
                    }
                }
                notify::event::EventKind::Modify(_) => {
                    // println!("DEBUG: File modified: {:?}", path);
                    compile_macros(&path, &macros_path);
                }
                notify::event::EventKind::Remove(_) => {
                    // println!("DEBUG: File removed: {:?}", path);
                    if macros_path.exists() {
                        fs::remove_file(macros_path).expect("Failed to remove file");
                    }
                }
                _ => {}
            }
        }
    }
}

fn compile_macros(input_path: &Path, output_path: &Path) {
    let input_file = std::fs::read_to_string(input_path).expect("Failed to read input file");
    let ast = parse_file(&input_file).expect("Failed to parse file");

    let mut extractor = MacroExtractor { 
        item_macros: HashMap::new(), 
        current_item: None 
    };
    extractor.visit_file(&ast);

    let mut output_file = fs::File::create(output_path).expect("Failed to create output file");

    for (item_name, macros) in extractor.item_macros {
        writeln!(output_file, "{} {{", item_name).unwrap();
        for (macro_name, attrs) in macros {
            let attr_list = attrs.join(", ");
            writeln!(output_file, "   {}: {}", macro_name, attr_list).unwrap();
        }
        writeln!(output_file, "}}\n").unwrap();
    }
}

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
        self.current_item = None;
    }

    fn visit_item_fn(&mut self, item: &'ast ItemFn) {
        let fn_name = format!("fn {}", item.sig.ident.to_string());
        self.current_item = Some(fn_name.clone());
        visit::visit_item_fn(self, item);
        self.current_item = None;
    }

    fn visit_item_impl(&mut self, item: &'ast ItemImpl) {
        for impl_item in &item.items {
            if let syn::ImplItem::Fn(fn_item) = impl_item {
                let fn_name = format!("fn {}", fn_item.sig.ident.to_string());
                self.current_item = Some(fn_name.clone());
                visit::visit_impl_item_fn(self, fn_item);
                self.current_item = None;
            }
        }
        visit::visit_item_impl(self, item);
    }

    fn visit_item_enum(&mut self, item: &'ast ItemEnum) {
        let enum_name = format!("enum {}", item.ident);
        self.current_item = Some(enum_name.clone());
        visit::visit_item_enum(self, item);
        self.current_item = None;
    }

    fn visit_variant(&mut self, var: &'ast Variant) {
        let variant_name = format!("variant {}", var.ident);
        self.current_item = Some(variant_name.clone());
        visit::visit_variant(self, var);
        self.current_item = None;
    }
}

fn trim_macro_contents(macro_str: &str) -> String {
    let start_idx = macro_str.find('(').unwrap_or(0) + 1;
    let end_idx = macro_str.rfind(')').unwrap_or(macro_str.len());
    let contents = &macro_str[start_idx..end_idx];
    contents.trim().replace(", ", ",")
}

pub fn start_watching() -> notify::Result<()> {
    let (tx, rx) = mpsc::channel();

    let mut watcher = RecommendedWatcher::new(
        move |res| {
            match res {
                Ok(event) => tx.send(event).unwrap(),
                Err(e) => eprintln!("watch error: {:?}", e),
            }
        },
        Config::default(),
    )?;

    watcher.watch(Path::new(PATH), RecursiveMode::Recursive)?;

    for event in rx {
        handle_changes(event);
    }

    Ok(())
}
