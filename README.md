<div style="text-align: center">

  <h1><code>WasmJieba</code></h1>

<strong>A WASM binding to <a href="https://github.com/messense/jieba-rs">jieba-rs</a>.</strong>

<sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>
</div>

## About

This is a minimal usable WASM binding to [jieba-rs].
Both Simplified Chinese and Traditional Chinese are supported.
Thanks to [js-jieba] for the `default.hant` dict file.

[jieba-rs]: https://github.com/messense/jieba-rs

[js-jieba]: https://www.npmjs.com/package/js-jieba

## 🎁 Implemented APIs

All APIs from jieba-rs.
Just read [src/lib.rs].
You may also refer to [jieba-rs' documentation].

[src/lib.rs]: https://github.com/ZnqbuZ/WasmJieba/blob/master/src/lib.rs
[jieba-rs' documentation]: https://docs.rs/crate/jieba-rs/latest

## 🛠 Packages

* [wasmjieba] (Dummy package that depends on both targets. Pick the one you need below.)
    * [wasmjieba-nodejs]
    * [wasmjieba-web]

[wasmjieba]: https://www.npmjs.com/package/wasmjieba

[wasmjieba-nodejs]: https://www.npmjs.com/package/wasmjieba-nodejs

[wasmjieba-web]: https://www.npmjs.com/package/wasmjieba-web

## 🚴 Usage

See [examples/web] for an example.

[examples/web]: https://github.com/ZnqbuZ/WasmJieba/tree/master/examples/web