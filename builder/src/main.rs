use notify_debouncer_full::new_debouncer;
use notify_debouncer_full::notify::{RecursiveMode, Watcher};

use std::env;
use std::path::{Path, PathBuf};
use std::time::Duration;

fn find_workspace_root(mut current_dir: PathBuf) -> Option<PathBuf> {
    loop {
        if current_dir.join("Cargo.toml").exists() {
            let content = std::fs::read_to_string(current_dir.join("Cargo.toml")).ok()?;
            if content.contains("[workspace]") {
                return Some(current_dir);
            }
        }

        if !current_dir.pop() {
            return None;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let crate_path = PathBuf::from(crate_dir);

    if let Some(workspace_root) = find_workspace_root(crate_path) {
        let workspace_path = Path::new(&workspace_root);
        let src_path = workspace_path.join("src");
        println!(
            "Workspace root: {:?} | watch dir: {:?} ",
            workspace_path, src_path
        );

        let (tx, rx) = std::sync::mpsc::channel();
        let mut debouncer = new_debouncer(Duration::from_secs(1), None, tx)?;
        debouncer
            .watcher()
            .watch(&src_path, RecursiveMode::Recursive)?;

        for result in rx {
            match result {
                Ok(events) => events.iter().for_each(|event| println!("{event:?}")),
                Err(errors) => errors.iter().for_each(|error| println!("{error:?}")),
            }
            println!();
        }

        Ok(())
    } else {
        Err("Workspace root not found".into())
    }
}
