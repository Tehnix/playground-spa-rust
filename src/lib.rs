mod app;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// This is our entry point to our WASM application, exposed to the JS side. In
/// `main.js` we import the code that `wasm-pack` bundles up, and call this function.
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::App>();
    Ok(())
}
