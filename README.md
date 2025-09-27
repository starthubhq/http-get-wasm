# HTTP GET WASM Module

A WebAssembly module for making HTTP GET requests, built with Rust and Waki.

## Prerequisites

- wasmtime (install with `brew install wasmtime` on macOS, or download from [wasmtime.dev](https://wasmtime.dev/))
- Rust with nightly toolchain

## Quick Start

1. Install wasmtime: `brew install wasmtime`
2. Build the WASM module: `cargo +nightly component build --release`
3. Run a test: `echo '[{"url":"https://httpbin.org/json"}]' | wasmtime -S http ./target/wasm32-wasip1/release/http-get-wasm.wasm`

## Building

```bash
cargo +nightly component build --release
```

## Usage

### With wasmtime (Array Format)

```bash
echo '[{"name":"url","value":"https://httpbin.org/json"}]' \
  | wasmtime -S http ./target/wasm32-wasip1/release/http-get-wasm.wasm
```

### With headers (Array Format)

```bash
echo '[{"name":"url","value":"https://httpbin.org/headers"},{"name":"headers","value":{"User-Agent":"MyApp/1.0","X-Custom":"value"}}]' \
  | wasmtime -S http ./target/wasm32-wasip1/release/http-get-wasm.wasm
```

## Input Format

The module expects a JSON array with input objects:
- **Position 0**: `{"name": "url", "value": "..."}` - URL to fetch (required)
- **Position 1**: `{"name": "headers", "value": {...}}` - Headers object (optional)

Example:
```json
[
  {
    "name": "url",
    "value": "https://httpbin.org/get"
  },
  {
    "name": "headers", 
    "value": {
      "User-Agent": "MyApp/1.0",
      "Authorization": "Bearer token"
    }
  }
]
```

## Output Format

Returns a JSON array with output objects:
- **Position 0**: `{"name": "response", "value": {...}}` - HTTP response

Example:
```json
[
  {
    "name": "response",
    "value": {
      "status": 200,
      "body": "{\"message\": \"Hello World\"}"
    }
  }
]
```

## Implementation Notes

This is a pure WebAssembly module built with Rust and the Waki HTTP client:
- **WASI HTTP support** - uses WASI HTTP interfaces for network requests
- **Rust implementation** - compiled to WebAssembly for cross-platform execution
- **wasmtime execution** - requires wasmtime runtime with HTTP support (`-S http` flag)
- **JSON I/O** - reads JSON from stdin, outputs results to stdout

The module outputs results in the format: `::starthub:state::{json}` for integration with the StarHub platform.