cargo +nightly component build --release

echo '{"params":{"url":"https://httpbin.org/get"}}' \
  | wasmtime -S http ./target/
