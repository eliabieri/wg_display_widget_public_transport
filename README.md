# Swiss Public Transport widget for WG Display

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