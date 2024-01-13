# Run

```sh
cargo build --target wasm32-wasi && RUST_BACKTRACE=1 RUST_LOG=debug npx swc --config-file=.swcrc src/index.ts
```