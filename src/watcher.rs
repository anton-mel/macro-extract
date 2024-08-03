/// This file handles change events for the currently accessed
/// file. It automatically reruns a compiler for the macro
/// assertions if the .macro file is provided.

use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::{fs, path::{Path, PathBuf}, sync::mpsc};
use crate::skeleton::create_skeleton;
use crate::compiler::process_files;


static PATH: &str = "tests";
static MACROS_FOLDER: &str = "macros";

fn get_macros_path(src_path: &Path) -> PathBuf {
    // Replace ".rs" extension with ".macros"
    let macros_path = src_path.with_extension("macros");
    macros_path
}

/// Handle file changes
fn handle_changes(event: Event) {
    // Check if the event pertains to a file
    if let Some(path) = event.paths.first() {
        // Skip non-Rust files
        if path.extension().map_or(false, |ext| ext == "rs") {
            let macros_path = get_macros_path(path);
            match event.kind {
                notify::event::EventKind::Create(_) => {
                    println!("DEBUG: File created: {:?}", path);
                    if !macros_path.exists() {
                        fs::File::create(macros_path).expect("Failed to create file");
                    }
                }
                notify::event::EventKind::Modify(_) => {
                    println!("DEBUG: File modified: {:?}", path);
                    if !macros_path.exists() {
                        fs::File::create(macros_path).expect("Failed to create file");
                        create_skeleton(path);
                    } else {
                        let macros_folder = Path::new(MACROS_FOLDER);
                        process_files(path, &macros_path, macros_folder);
                    }
                }
                notify::event::EventKind::Remove(_) => {
                    println!("DEBUG: File removed: {:?}", path);
                    if macros_path.exists() {
                        fs::remove_file(macros_path).expect("Failed to remove file");
                    }
                }
                _ => {}
            }
        }
    }
}

/// Encapsulate Rust::notify initialization
pub fn start_watching() -> notify::Result<()> {
    // Create a channel to receive file change events
    let (tx, rx) = mpsc::channel();

    // Create a watcher
    let mut watcher = RecommendedWatcher::new(
        move |res| {
            match res {
                Ok(event) => tx.send(event).unwrap(),
                Err(e) => eprintln!("watch error: {:?}", e),
            }
        },
        Config::default(),
    )?;

    // Watch the src directory recursively
    watcher.watch(Path::new(PATH), RecursiveMode::Recursive)?;

    // Handle events
    for event in rx {
        handle_changes(event);
    }

    Ok(())
}

