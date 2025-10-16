# HTTP GET WASM Module

A WebAssembly module for making HTTP GET requests, built with Rust and Waki.

## Prerequisites

- wasmtime (install with `brew install wasmtime` on macOS, or download from [wasmtime.dev](https://wasmtime.dev/))
- Rust with nightly toolchain

## Quick Start

1. Install wasmtime: `brew install wasmtime`
2. Build the WASM module: `cargo +nightly component build --release`
3. Run a test: `echo '["https://api.restful-api.dev/objects"]' | wasmtime -S http ./target/wasm32-wasip1/release/http-get-wasm.wasm`

## Building

```bash
cargo +nightly component build --release
```

## Usage

### With wasmtime (Simplified Array Format)

```bash
echo '["https://api.restful-api.dev/objects"]' \
  | wasmtime -S http ./target/wasm32-wasip1/release/http-get-wasm.wasm
```

### With headers (Simplified Array Format)

```bash
echo '["https://api.restful-api.dev/objects",{"Accept":"application/json"}]' \
  | wasmtime -S http ./target/wasm32-wasip1/release/http-get-wasm.wasm
```

## Input Format

The module expects a simplified JSON array:
- **Position 0**: `"https://example.com"` - URL to fetch (required, string)
- **Position 1**: `{"Header-Name": "value"}` - Headers object (optional, object)

Example:
```json
[
  "https://api.restful-api.dev/objects",
  {
    "Accept": "application/json",
    "User-Agent": "MyApp/1.0"
  }
]
```

## Output Format

Returns a simplified JSON array:
- **Position 0**: `{"status": 200, "body": "..."}` - HTTP response

Example:
```json
[
  {
    "status": 200,
    "body": "{\"message\": \"Hello World\"}"
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