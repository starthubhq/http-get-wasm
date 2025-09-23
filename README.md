# HTTP GET WASM Module

A WebAssembly module for making HTTP GET requests, built with Rust and Waki.

## Prerequisites

- wasmtime (install with `brew install wasmtime` on macOS, or download from [wasmtime.dev](https://wasmtime.dev/))
- Rust with nightly toolchain

## Quick Start

1. Install wasmtime: `brew install wasmtime`
2. Build the WASM module: `cargo +nightly component build --release`
3. Run a test: `echo '[{"url":"https://httpbin.org/get"}]' | wasmtime -S http ./target/wasm32-wasip1/release/http-get-wasm.wasm`

## Building

```bash
cargo +nightly component build --release
```

## Usage

### With wasmtime

```bash
echo '[{"url":"https://httpbin.org/get"}]' \
  | wasmtime -S http ./target/wasm32-wasip1/release/http-get-wasm.wasm
```

### With headers

```bash
echo '[{"url":"https://httpbin.org/headers"},{"User-Agent":"MyApp/1.0","X-Custom":"value"}]' \
  | wasmtime -S http ./target/wasm32-wasip1/release/http-get-wasm.wasm
```

## Input Format

The module expects a JSON array with:
- **First element**: Object with `url` field (required)
- **Second element**: Object with HTTP headers (optional)

Example:
```json
[
  {"url": "https://httpbin.org/get"},
  {"User-Agent": "MyApp/1.0", "Authorization": "Bearer token"}
]
```

## Output Format

Returns a JSON array with:
- **First element**: Object with `status` field (HTTP status code)
- **Second element**: Object with `body` field (response body)

Example:
```json
[
  {"status": 200},
  {"body": "{\"message\": \"Hello World\"}"}
]
```

## Implementation Notes

This is a pure WebAssembly module built with Rust and the Waki HTTP client:
- **WASI HTTP support** - uses WASI HTTP interfaces for network requests
- **Rust implementation** - compiled to WebAssembly for cross-platform execution
- **wasmtime execution** - requires wasmtime runtime with HTTP support (`-S http` flag)
- **JSON I/O** - reads JSON from stdin, outputs results to stdout

The module outputs results in the format: `::starthub:state::{json}` for integration with the StarHub platform.