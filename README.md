# Swiss Public Transport widget for WG Display

## 🛠️ Development

You need to have [Rust](https://www.rust-lang.org/tools/install) installed to develop a widget.  
Next, you need to add the `wasm32-wasip2` target to your Rust installation. This can be done by running the following command:

```bash
rustup target add wasm32-wasip2
```

To then build your widget, run the following command:

```bash
# Build widget
cargo build --target wasm32-wasip2 --release
```

The resulting WebAssembly component can be installed on the WG Display by starting a local web server and supplying the URL of the component to the WG Display Web Dashboard install page.