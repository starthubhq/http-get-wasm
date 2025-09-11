# HTTP GET WASM Module

A WebAssembly module for making HTTP GET requests, built with Rust and Waki, with Node.js integration.

## Prerequisites

- Node.js 16+ 
- wasmtime (install with `brew install wasmtime`)
- Rust with nightly toolchain

## Building

```bash
cargo +nightly component build --release
```

## Usage

### With wasmtime (original)

```bash
echo '{"params":{"url":"https://httpbin.org/get"}}' \
  | wasmtime -S http ./target/wasm32-wasip1/release/http-get-wasm.wasm
```

### With Node.js (new)

Uses wasmtime to run the compiled WASM module:
```bash
# Basic usage
node index.js '{"params":{"url":"https://httpbin.org/get"}}'

# With headers
node index.js '{"params":{"url":"https://httpbin.org/headers","headers":{"User-Agent":"MyApp/1.0"}}}'

# Run tests
npm test
```

### Programmatic usage in Node.js

```javascript
const { HttpGetRunner } = require('./index.js');

const runner = new HttpGetRunner();
const result = await runner.run({
  params: {
    url: "https://httpbin.org/get",
    headers: {
      "User-Agent": "MyApp/1.0"
    }
  }
});

console.log(result.data); // { status: 200, body: "..." }
```

## Input Parameters

- `url` (string, required): The URL to fetch data from
- `headers` (object, optional): HTTP headers to send with the request

## Output

Returns a JSON object with:
- `status` (number): HTTP status code
- `body` (string): Response body

## Implementation Notes

The Node.js implementation uses wasmtime as a subprocess to run the compiled WASM module:
- **Actual WASM execution** - runs the real Rust-compiled WASM module
- **wasmtime integration** - uses wasmtime for reliable WASM execution
- **Same interface** - maintains the exact same input/output format as direct wasmtime usage
- **Error handling** - proper error handling and logging
- **Dependency checking** - automatically checks for wasmtime availability

This approach gives you the best of both worlds: the performance and capabilities of the WASM module with the convenience of Node.js integration.
