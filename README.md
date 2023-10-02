# WASM-Mandelbrot

## Description

This repository contains an implementation of the Mandelbrot set using WebAssembly (WASM). 

Try the demo at
 https://kutasi.dev/wasm-mandelbrot

## Features

- (not so) Fast and efficient rendering of the Mandelbrot set.
- Interactive zoom and pan functionality.
- High-resolution image export.

## Prerequisites

- WebAssembly compatible browser
- Rust and wasm-pack for building

## Building

To build the project, run the following command in the root directory of the project:

```bash
wasm-pack build --target web
```

## Running

To run the project, start a local server in the root directory of the project. You can use Python's built-in HTTP server:

```bash
npx http-server .
```

Then, open your browser and navigate to \`localhost:8000\`.

## Contributing

Contributions are welcome! Please read the contributing guidelines before making any changes.

## License

This project is licensed under the GPLv3 License. See \`LICENSE\` for more information.


