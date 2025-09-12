# HTTP GET WASM Module

A WebAssembly module for making HTTP GET requests, built with Rust and Waki.

## Prerequisites

- wasmtime (install with `brew install wasmtime` on macOS, or download from [wasmtime.dev](https://wasmtime.dev/))
- Rust with nightly toolchain

## Quick Start

1. Install wasmtime: `brew install wasmtime`
2. Build the WASM module: `cargo +nightly component build --release`
3. Run a test: `echo '{"params":{"url":"https://httpbin.org/get"}}' | wasmtime -S http ./target/wasm32-wasip1/release/http-get-wasm.wasm`

## Building

```bash
cargo +nightly component build --release
```

## Usage

### With wasmtime

```bash
echo '{"params":{"url":"https://httpbin.org/get"}}' \
  | wasmtime -S http ./target/wasm32-wasip1/release/http-get-wasm.wasm
```

### With headers

```bash
echo '{"params":{"url":"https://httpbin.org/headers","headers":{"User-Agent":"MyApp/1.0"}}}' \
  | wasmtime -S http ./target/wasm32-wasip1/release/http-get-wasm.wasm
```

## Input Parameters

- `url` (string, required): The URL to fetch data from
- `headers` (object, optional): HTTP headers to send with the request

## Output

Returns a JSON object with:
- `status` (number): HTTP status code
- `body` (string): Response body

## Implementation Notes

This is a pure WebAssembly module built with Rust and the Waki HTTP client:
- **WASI HTTP support** - uses WASI HTTP interfaces for network requests
- **Rust implementation** - compiled to WebAssembly for cross-platform execution
- **wasmtime execution** - requires wasmtime runtime with HTTP support (`-S http` flag)
- **JSON I/O** - reads JSON from stdin, outputs results to stdout

The module outputs results in the format: `::starthub:state::{json}` for integration with the StarHub platform.