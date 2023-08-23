mod utils;

use std::io::BufReader;
use std::sync::{OnceLock, RwLock};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
}

fn instance() -> &'static RwLock<jieba_rs::Jieba> {
    static INSTANCE: OnceLock<RwLock<jieba_rs::Jieba>> = OnceLock::new();
    INSTANCE.get_or_init(|| RwLock::new(
        jieba_rs::Jieba::new()))
}


#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn cut(text: &str, hmm: bool) -> Vec<JsValue> {
    instance().read().unwrap()
        .cut(text, hmm)
        .into_iter()
        .map(JsValue::from)
        .collect()
}


#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn loadDict(dict: &str) {
    let mut reader = BufReader::new(dict.as_bytes());
    instance().write().unwrap()
        .load_dict(&mut reader).unwrap();
}
