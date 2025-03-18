mod utils;

use log::info;
use std::io::{BufReader, Cursor};
use std::sync::{OnceLock, RwLock};

use crate::utils::set_panic_hook;
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;
use zstd::decode_all;

#[cfg(not(target_family = "wasm"))]
type JsValue = String;

#[cfg(target_family = "wasm")]
fn to_value<T: serde::ser::Serialize + ?Sized>(
    value: &T,
) -> Result<JsValue, serde_wasm_bindgen::Error> {
    value.serialize(&serde_wasm_bindgen::Serializer::new())
}

#[cfg(not(target_family = "wasm"))]
fn to_value<T: serde::ser::Serialize + ?Sized>(value: &T) -> Result<JsValue, serde_json::Error> {
    serde_json::to_string(value)
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
unsafe extern "C" {}

static INSTANCE: OnceLock<RwLock<jieba_rs::Jieba>> = OnceLock::new();

const COMPRESSED_DICTS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/dicts.bin"));

fn instance() -> &'static RwLock<jieba_rs::Jieba> {
    INSTANCE.get_or_init(|| {
        set_panic_hook();

        let new_instance = RwLock::new(jieba_rs::Jieba::new());

        info!("jieba instance created.");

        new_instance
            .write()
            .unwrap()
            .load_dict(&mut Cursor::new(decode_all(Cursor::new(COMPRESSED_DICTS)).unwrap()))
            .unwrap();

        new_instance
    })
}

macro_rules! read_instance {
    () => {
        instance().read().unwrap()
    };
}

macro_rules! write_instance {
    () => {
        instance().write().unwrap()
    };
}

macro_rules! to_js_vec {
    ($vec:expr) => {
        $vec.into_iter().map(JsValue::from).collect()
    };
    ($vec:expr, $convert:expr) => {
        $vec.into_iter().map($convert).collect::<Vec<JsValue>>()
    };
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[allow(non_snake_case)]
pub fn cut(text: &str, hmm: bool) -> Vec<JsValue> {
    to_js_vec!(read_instance!().cut(text, hmm))
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[allow(non_snake_case)]
pub fn cutAll(text: &str) -> Vec<JsValue> {
    to_js_vec!(read_instance!().cut_all(text))
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[allow(non_snake_case)]
pub fn cutForSearch(text: &str, hmm: bool) -> Vec<JsValue> {
    to_js_vec!(read_instance!().cut_for_search(text, hmm))
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
    to_js_vec!(
        read_instance!().tokenize(text, mode.map(|m| m.to_jieba()).unwrap(), hmm),
        |t| to_value(&t).unwrap()
    )
}

// I don't know what this function is for actually.
// Implemented anyway.
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[allow(non_snake_case)]
pub fn tag(sentence: &str, hmm: bool) -> Vec<JsValue> {
    to_js_vec!(read_instance!().tag(sentence, hmm), |t| to_value(&t)
        .unwrap())
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[allow(non_snake_case)]
pub fn suggestFreq(segment: &str) -> usize {
    instance().write().unwrap().suggest_freq(segment)
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[allow(non_snake_case)]
pub fn addWord(word: &str, freq: Option<usize>, tag: Option<String>) -> usize {
    write_instance!().add_word(word, freq, tag.as_deref())
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[allow(non_snake_case)]
pub fn loadDict(dict: &str) {
    let mut reader = BufReader::new(dict.as_bytes());
    write_instance!().load_dict(&mut reader).unwrap();
}
