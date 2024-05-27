mod shared;

use shared::init_and_run;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet() -> String {
    let name = "This is a bevy wasm app!";
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub fn run_bevy_app(canvas_id: &str) {
    init_and_run(Some(canvas_id.to_string()));
}
