#![allow(dead_code)]

extern crate serde;
extern crate serde_json;

#[cfg(not(target_arch = "wasm32"))]
mod cpython;

// #[cfg(not(target_arch = "wasm32"))]

// NOTE: This produce LSP error which break it down
// #[cfg(target_arch = "wasm32")]
// mod wasm;

// #[cfg(target_arch = "wasm32")]
// pub use wasm::agent::WasmAgent;
//
// mod agent;
// pub use agent::Agent;
