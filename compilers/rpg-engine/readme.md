# WASM Build Guide

## Prerequisites
1. Install Rust
2. Add wasm32 target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
3. Install trunk:
   ```bash
   cargo install trunk
   ```

## Building for WASM
1. Build and serve locally:
   ```bash
   trunk serve
   ```

2. Build for release:
   ```bash
   trunk build --release
   ```

## Notes
- The generated files will be in the `dist` directory
- You can customize the build in `Trunk.toml`
