mod utils;

use std::io::BufReader;
use std::sync::{OnceLock, RwLock};
use log::info;

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(not(target_family = "wasm"))]
type JsValue = String;

#[cfg(target_family = "wasm")]
fn to_value<T: serde::ser::Serialize + ?Sized>(value: &T) -> Result<JsValue, serde_wasm_bindgen::Error> {
    value.serialize(&serde_wasm_bindgen::Serializer::new())
}

#[cfg(not(target_family = "wasm"))]
fn to_value<T: serde::ser::Serialize + ?Sized>(value: &T) -> Result<JsValue, serde_json::Error> {
    serde_json::to_string(value)
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
extern {}

fn instance() -> &'static RwLock<jieba_rs::Jieba> {
    static INSTANCE: OnceLock<RwLock<jieba_rs::Jieba>> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        let new_instance = RwLock::new(
            jieba_rs::Jieba::new());

        info!("jieba instance created.");

        let default_hans_dict = include_str!("dicts/default.hans");
        new_instance.write().unwrap()
            .load_dict(&mut BufReader::new(default_hans_dict.as_bytes())).unwrap();

        info!("default.hans loaded.");

        let default_hant_dict = include_str!("dicts/default.hant");
        new_instance.write().unwrap()
            .load_dict(&mut BufReader::new(default_hant_dict.as_bytes())).unwrap();

        info!("default.hant loaded.");

        new_instance
    })
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[allow(non_snake_case)]
pub fn cut(text: &str, hmm: bool) -> Vec<JsValue> {
    instance().read().unwrap()
        .cut(text, hmm)
        .into_iter()
        .map(JsValue::from)
        .collect()
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[allow(non_snake_case)]
pub fn cutAll(text: &str) -> Vec<JsValue> {
    instance().read().unwrap()
        .cut_all(text)
        .into_iter()
        .map(JsValue::from)
        .collect()
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[allow(non_snake_case)]
pub fn cutForSearch(text: &str, hmm: bool) -> Vec<JsValue> {
    instance().read().unwrap()
        .cut_for_search(text, hmm)
        .into_iter()
        .map(JsValue::from)
        .collect()
}

// wasm_bindgen needs to read this signature.
// I wish patch jieba-rs as little as possible.
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[allow(non_snake_case)]
pub enum TokenizeMode {
    Default,
    Search,
}

// Inelegant, but no better way found.
impl TokenizeMode {
    fn to_jieba(&self) -> jieba_rs::TokenizeMode {
        match self {
            TokenizeMode::Default => jieba_rs::TokenizeMode::Default,
            TokenizeMode::Search => jieba_rs::TokenizeMode::Search,
        }
    }
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[allow(non_snake_case)]
pub fn tokenize(text: &str, mode: Option<TokenizeMode>, hmm: bool) -> Vec<JsValue> {
    instance().read().unwrap()
        .tokenize(text, mode.map(|m| m.to_jieba()).unwrap(), hmm)
        .into_iter()
        .map(|t| to_value(&t).unwrap())
        .collect()
}

// I don't know what this function is for actually.
// Implemented anyway.
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[allow(non_snake_case)]
pub fn tag(sentence: &str, hmm: bool) -> Vec<JsValue> {
    instance().read().unwrap()
        .tag(sentence, hmm)
        .into_iter()
        .map(|t| to_value(&t).unwrap())
        .collect()
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[allow(non_snake_case)]
pub fn suggestFreq(segment: &str) -> usize {
    instance().write().unwrap()
        .suggest_freq(segment)
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[allow(non_snake_case)]
pub fn addWord(word: &str, freq: Option<usize>, tag: Option<String>) -> usize {
    instance().write().unwrap()
        .add_word(word, freq, tag.as_deref())
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[allow(non_snake_case)]
pub fn loadDict(dict: &str) {
    let mut reader = BufReader::new(dict.as_bytes());
    instance().write().unwrap()
        .load_dict(&mut reader).unwrap();
}
