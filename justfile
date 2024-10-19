test:
  cargo test --all-features --all-targets && \
  wasm-pack test --firefox --headless
