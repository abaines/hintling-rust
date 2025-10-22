# hintling-rust
Client-only party word-guessing game where players giving hints without saying the word. Built for mobile web browser (no app stores). For learning Rust programming language.

## Minimal Yew+Trunk Starter

This is a minimal starter project using Yew (a Rust framework for building web apps) and Trunk (a build tool for Rust WebAssembly applications).

![Yew+Trunk Starter Screenshot](https://github.com/user-attachments/assets/318866a0-1837-4990-af6a-e099df152b1e)

### Prerequisites

1. Install Rust: https://rustup.rs/
2. Add WebAssembly target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
3. Install Trunk:
   ```bash
   cargo install trunk
   ```

### Development

To run the development server with hot-reload:

```bash
trunk serve
```

This will start a local server at `http://127.0.0.1:8080` (or another port if 8080 is busy).

### Building for Production

To build the project for production:

```bash
trunk build --release
```

The output will be in the `dist/` directory.

### Project Structure

- `src/main.rs` - Main application code with Yew components
- `index.html` - HTML template for Trunk
- `Cargo.toml` - Rust project dependencies and configuration

