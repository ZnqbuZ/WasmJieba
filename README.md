<div align="center">

  <h1><code>WasmJieba</code></h1>

<strong>A WASM binding to <a href="https://github.com/messense/jieba-rs">jieba-rs</a>.</strong>

<sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>
</div>

## About

This is a minimal partially usable WASM binding to [jieba-rs].
Both Simplified Chinese and Traditional Chinese are supported.
Thanks to [js-jieba] for the `default.hant` dict file.

Currently, only `cut` and `loadDict` are implemented.

[jieba-rs]: https://github.com/messense/jieba-rs
[js-jieba]: https://www.npmjs.com/package/js-jieba

## 🚴 Usage

See [examples/web] for an example.

[examples/web]: https://github.com/ZnqbuZ/WasmJieba/tree/master/examples/web