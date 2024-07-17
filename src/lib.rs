use js_sys::Object;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::js_sys;
pub enum AnimeValue {
    String(String),
    Number(f64),
}

pub fn hashmap_to_anime_options(options: HashMap<&str, AnimeValue>) -> JsValue {
    let obj = Object::new();
    for (key, value) in options {
        let js_value = match value {
            AnimeValue::String(s) => JsValue::from_str(&s),
            AnimeValue::Number(n) => JsValue::from_f64(n),
        };
        js_sys::Reflect::set(&obj, &JsValue::from_str(key), &js_value).unwrap();
    }
    obj.into()
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    pub fn anime(options: JsValue);
}
