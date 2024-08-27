use notify_debouncer_full::new_debouncer;
use notify_debouncer_full::notify::{RecursiveMode, Watcher};

use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

use std::io::{BufRead, BufReader};
use std::thread;

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
        let wasm_workspace = workspace_root.join("wasm");
        let workspace_path = Path::new(&wasm_workspace);
        let src_path = workspace_path.join("src");
        println!(
            "Workspace root: {:?} | watch dir: {:?} ",
            workspace_path, src_path
        );

        let mut wasm_pack = Command::new("wasm-pack");
        wasm_pack
            .arg("build")
            .arg("--target=web")
            .arg("--out-dir=../client/pkg");
        wasm_pack.current_dir(workspace_path);

        let (tx, rx) = std::sync::mpsc::channel();
        let mut debouncer = new_debouncer(Duration::from_secs(1), None, tx)?;
        debouncer
            .watcher()
            .watch(&src_path, RecursiveMode::Recursive)?;

        for result in rx {
            match result {
                Ok(_) => {
                    let mut cmd = wasm_pack
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()?;

                    let stdout = cmd.stdout.take().expect("Failed to capture stdout");
                    let stderr = cmd.stderr.take().expect("Failed to capture stderr");

                    let stdout_thread = thread::spawn(move || {
                        let reader = BufReader::new(stdout);
                        for line in reader.lines() {
                            println!("wasm_pack | {}", line.unwrap());
                        }
                    });

                    let stderr_thread = thread::spawn(move || {
                        let reader = BufReader::new(stderr);
                        for line in reader.lines() {
                            eprintln!("wasm_pack | {}", line.unwrap());
                        }
                    });

                    let status = cmd.wait()?;
                    println!("Build finished with status: {}", status);

                    stdout_thread.join().expect("Failed to join stdout thread");
                    stderr_thread.join().expect("Failed to join stderr thread");
                }
                Err(errors) => errors.iter().for_each(|error| println!("{error:?}")),
            }

            println!();
        }

        Ok(())
    } else {
        Err("Workspace root not found".into())
    }
}
