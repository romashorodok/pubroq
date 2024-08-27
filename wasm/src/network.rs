use js_sys::{Array, Function, Promise};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    pub type WebTransport;

    #[wasm_bindgen(method, getter)]
    fn ready(this: &WebTransport) -> Promise;

    #[wasm_bindgen(structural, method)]
    fn closed(this: &WebTransport) -> Promise;

    #[wasm_bindgen(method)]
    fn close(this: &WebTransport);
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct WebTransportInterface {
    pub(crate) web_transport: WebTransport,
}

async fn run_any_function(
    _s: &mut WebTransportInterface,
    function: Function,
    args: Vec<JsValue>,
) -> JsValue {
    let array_args = Array::new();
    for arg in args {
        array_args.push(&arg);
    }

    let promise = function.apply(&JsValue::NULL, &array_args).unwrap();
    let promise: Promise = promise.dyn_into().unwrap();

    // Await the promise and return the result
    let result = JsFuture::from(promise).await.unwrap();
    result
}

#[wasm_bindgen]
impl WebTransportInterface {
    #[wasm_bindgen(constructor)]
    pub fn new(web_transport: WebTransport) -> Self {
        Self { web_transport }
    }

    // pub fn modify_field(&mut self, value: i32) {
    //     self.field_to_be_modified = value;
    // }
    //
    // pub fn field(&self) -> i32 {
    //     self.field_to_be_modified
    // }

    // #[wasm_bindgen]
    // pub async fn with_callback(&self, function_or_promise: JsValue) -> Result<JsValue, JsValue> {
    //     let mut s = WebTransportInterface::new();
    //     let function = function_or_promise.dyn_into().map_err(|_| {
    //         JsError::new("The provided callback is not a function. Please provide a function.")
    //     })?;
    //
    //     let result = run_any_function(&mut s, function, vec![JsValue::from(1u32)]).await;
    //     let result_number = result.as_f64().unwrap_or(0.0);
    //     let final_result = result_number + 1.0;
    //
    //     Ok(JsValue::from(final_result))
    // }
}
