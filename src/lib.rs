mod utils;

use std::io::BufReader;
use std::sync::{OnceLock, RwLock};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {}

fn instance() -> &'static RwLock<jieba_rs::Jieba> {
    static INSTANCE: OnceLock<RwLock<jieba_rs::Jieba>> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        let new_instance = RwLock::new(
            jieba_rs::Jieba::new());

        let default_hans_dict = include_str!("dicts/default.hans.dict.txt");
        new_instance.write().unwrap()
            .load_dict(&mut BufReader::new(default_hans_dict.as_bytes())).unwrap();

        let default_hant_dict = include_str!("dicts/default.hant.dict.txt");
        new_instance.write().unwrap()
            .load_dict(&mut BufReader::new(default_hant_dict.as_bytes())).unwrap();

        new_instance
    })
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
pub fn cutAll(text: &str) -> Vec<JsValue> {
    instance().read().unwrap()
        .cut_all(text)
        .into_iter()
        .map(JsValue::from)
        .collect()
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn cutForSearch(text: &str, hmm: bool) -> Vec<JsValue> {
    instance().read().unwrap()
        .cut_for_search(text, hmm)
        .into_iter()
        .map(JsValue::from)
        .collect()
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn suggestFreq(segment: &str) -> usize {
    instance().write().unwrap()
        .suggest_freq(segment)
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn addWord(word: &str, freq: Option<usize>, tag: Option<String>) -> usize {
    instance().write().unwrap()
        .add_word(word, freq, tag.as_deref())
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn loadDict(dict: &str) {
    let mut reader = BufReader::new(dict.as_bytes());
    instance().write().unwrap()
        .load_dict(&mut reader).unwrap();
}
