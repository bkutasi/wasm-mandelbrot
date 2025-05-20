# WASM-Mandelbrot

## Description

This repository contains an implementation of the Mandelbrot set using WebAssembly (WASM). 

Try the demo at
 https://kutasi.dev/wasm-mandelbrot

## Features

- Fast and efficient rendering of the Mandelbrot set, accelerated with WebAssembly and Web Workers for parallelism.
- Interactive zoom and pan functionality.
- High-resolution image export.

## Prerequisites

- WebAssembly compatible browser.
- Rust and wasm-pack for building.
- Nightly Rust toolchain: The project is configured to use a specific nightly version of Rust due to WebAssembly threading requirements. See `rust-toolchain.toml`.
- The project uses `wasm-bindgen-rayon` for multi-threading.

## Building

To build the project, run the following command in the root directory of the project:

```bash
wasm-pack build --target web
```

(Note: `wasm-pack` is a separate tool, not a subcommand of `cargo`. Ensure `wasm-pack` is installed and accessible in your system's PATH.)

Ensure your Nightly toolchain is correctly installed and active. The necessary Rust flags and features (like atomics and bulk memory) are specified in `.cargo/config.toml` and the toolchain is defined in `rust-toolchain.toml`. These files are crucial for building the threaded WebAssembly module. `rustup` should automatically use the specified toolchain if `rust-toolchain.toml` is present in the project root.

## Running

To run the project, start a local server in the root directory of the project. You can use Python's built-in HTTP server:

```bash
npx http-server .
```

Then, open your browser and navigate to `localhost:8000`.

## Contributing

Contributions are welcome! Open an issue for any suggestions.

## License

This project is licensed under the GPLv3 License.
