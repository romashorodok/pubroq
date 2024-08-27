use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct WasmAgent {}

#[wasm_bindgen]
impl WasmAgent {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmAgent {
        WasmAgent {}
    }

    #[wasm_bindgen]
    pub fn ufrag(&self) -> String {
        protocol::Agent::ufrag(self)
    }
}

impl protocol::Agent for WasmAgent {
    fn ufrag(&self) -> String {
        "test".into()
    }
}
