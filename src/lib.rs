#![allow(non_snake_case)]

mod utils;

use crate::utils::set_panic_hook;
use log::info;
use std::io::{BufReader, Cursor};
use std::sync::{LazyLock, RwLock};
use zstd::Decoder;

cfg_select! {
    target_family = "wasm" => { use wasm_bindgen::prelude::*; }
    _ => { type JsValue = String; }
}

fn to_value<T: serde::ser::Serialize + ?Sized>(value: &T) -> Result<JsValue, JsValue> {
    cfg_select! {
        target_family = "wasm" => value.serialize(&serde_wasm_bindgen::Serializer::new()),
        _ => serde_json::to_string(value)
    }
    .map_err(|e| e.to_string().into())
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
unsafe extern "C" {}

const COMPRESSED_DICTS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/dicts.bin"));

static INSTANCE: LazyLock<RwLock<jieba_rs::Jieba>> = LazyLock::new(|| {
    set_panic_hook();

    let instance = jieba_rs::Jieba::with_dict(&mut BufReader::new(
        Decoder::new(Cursor::new(COMPRESSED_DICTS)).unwrap(),
    )).unwrap();

    info!("instance created.");

    RwLock::new(instance)
});

macro_rules! read {
    () => {
        INSTANCE.read().unwrap()
    };
}

macro_rules! write {
    () => {
        INSTANCE.write().unwrap()
    };
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn cut(text: &str, hmm: bool) -> Result<JsValue, JsValue> {
    to_value(&read!().cut(text, hmm))
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn cutAll(text: &str) -> Result<JsValue, JsValue> {
    to_value(&read!().cut_all(text))
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn cutForSearch(text: &str, hmm: bool) -> Result<JsValue, JsValue> {
    to_value(&read!().cut_for_search(text, hmm))
}

// wasm_bindgen needs to read this signature.
// I wish patch jieba-rs as little as possible.
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
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
pub fn tokenize(text: &str, mode: TokenizeMode, hmm: bool) -> Result<JsValue, JsValue> {
    to_value(&read!().tokenize(text, mode.to_jieba(), hmm))
}

// I don't know what this function is for actually.
// Implemented anyway.
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn tag(sentence: &str, hmm: bool) -> Result<JsValue, JsValue> {
    to_value(&read!().tag(sentence, hmm))
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn suggestFreq(segment: &str) -> usize {
    write!().suggest_freq(segment)
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn addWord(word: &str, freq: Option<usize>, tag: Option<String>) -> usize {
    write!().add_word(word, freq, tag.as_deref())
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn loadDict(dict: &str) {
    write!()
        .load_dict(&mut BufReader::new(dict.as_bytes()))
        .unwrap();
}
