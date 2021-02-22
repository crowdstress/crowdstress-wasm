#[path = "app.rs"]
mod app;
#[path = "utils/behaviour.rs"]
mod behaviour;
#[path = "config.rs"]
mod config;
#[path = "utils/human.rs"]
mod human;
#[path = "utils/physics.rs"]
mod physics;

#[macro_use]
extern crate serde_derive;
extern crate js_sys;
extern crate serde_json;
extern crate wasm_bindgen;

use crate::app::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn tick(app_object: JsValue) -> JsValue {
    let mut app: App = app_object.into_serde().unwrap();
    app.tick()
}
