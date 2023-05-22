# WG Display widget template for Rust


Template repository for creating a [WG Display](https://github.com/eliabieri/wg_display) widget in Rust.  

## 🚀 Getting started

To get started, simply click the `Use this template` button on the top of the repository.  
The API available can be explored [here](https://eliabieri.github.io/wg_display_widget_wit/) or by examining `wg_display_widget_wit/wit`.

## 🛠️ Development

You need to have [Rust](https://www.rust-lang.org/tools/install) installed to develop a widget.  
Next, you need to add the `wasm32-unknown-unknown` target to your Rust installation. This can be done by running the following command:

```bash
rustup target add wasm32-unknown-unknown
```

Lastly, you need to install the `wasm-tools` CLI. This can be done by running the following command:

```bash
cargo install wasm-tools
```

To then build your widget, run the following command:

```bash
# Build widget
cargo build --target wasm32-unknown-unknown --release
# Transform WebAssembly module to WebAssembly component
wasm-tools component new target/wasm32-unknown-unknown/release/widget.wasm -o target/wasm32-unknown-unknown/release/widget.wasm
```

The resulting WebAssembly component can be installed on the WG Display by starting a local web server and supplying the URL of the component to the WG Display Web Dashboard install page.

## 📲 Add your widget to the store

To make your widget available to download in the WG Display Web Dashboard, you need to register it with the main `WG Display` project. This can be done by opening a Pull Request.  
The following steps are needed to register your widget:
1) Fork the [WG Display](https://github.com/eliabieri/wg_display) repository
2) Clone the fork you've just created
3) Add the metadata of your widget to the `widget_store.json` file. You need to supply the following information:
    - `name`: The name of your widget
    - `description`: A short description of your widget
    - `repository`: The URL to the repository of your widget
4) Commit your changes and push them to your fork
5) Open a Pull Request to the main `WG Display` repository
6) Once your Pull Request is merged, your widget will be available to download in the Web Dashboard 🎉