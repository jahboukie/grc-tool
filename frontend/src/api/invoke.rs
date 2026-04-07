use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    fn tauri_invoke(cmd: &str, args: JsValue) -> js_sys::Promise;
}

/// Call a Tauri command with typed arguments and return type.
pub async fn call<A: Serialize, R: DeserializeOwned>(cmd: &str, args: &A) -> Result<R, String> {
    let args_js = serde_wasm_bindgen::to_value(args).map_err(|e| e.to_string())?;
    let promise = tauri_invoke(cmd, args_js);
    let result = JsFuture::from(promise).await.map_err(|e| format!("{:?}", e))?;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

/// Call a Tauri command whose Rust signature expects a named single argument such as `dto` or `request`.
pub async fn call_named<A: Serialize, R: DeserializeOwned>(
    cmd: &str,
    arg_name: &str,
    args: &A,
) -> Result<R, String> {
    let wrapped = serde_json::json!({ arg_name: args });
    let args_js = serde_wasm_bindgen::to_value(&wrapped).map_err(|e| e.to_string())?;
    let promise = tauri_invoke(cmd, args_js);
    let result = JsFuture::from(promise).await.map_err(|e| format!("{:?}", e))?;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

/// Call a Tauri command with no arguments.
pub async fn call_no_args<R: DeserializeOwned>(cmd: &str) -> Result<R, String> {
    let args_js = serde_wasm_bindgen::to_value(&serde_json::json!({})).map_err(|e| e.to_string())?;
    let promise = tauri_invoke(cmd, args_js);
    let result = JsFuture::from(promise).await.map_err(|e| format!("{:?}", e))?;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}
