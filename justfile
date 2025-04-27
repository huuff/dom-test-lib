test:
  cargo test --all-features && \
  wasm-pack test --firefox --headless --all-features
