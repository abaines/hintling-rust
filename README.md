# hintling-rust
Client-only party word-guessing game where players give hints without saying the word.
Built for mobile web browsers (no app stores).
For learning the Rust programming language.

## Minimal Yew+Trunk Starter

This is a minimal starter project using Yew (a Rust framework for building web apps)
and Trunk (a build tool for Rust WebAssembly applications).

### Development environment: GitHub Codespaces / Dev Container (Ubuntu 24.04)

If you see `bash: rustup: command not found` inside your Codespace, Rust is not installed in the container.
Use the steps below to install Rust and the required tooling inside a GitHub Codespace or any Ubuntu 24.04 dev container.

1. Update packages and install build prerequisites:

```bash
sudo apt update
sudo apt install -y build-essential curl pkg-config libssl-dev
```

2. Install rustup (the official Rust toolchain installer) and add Cargo to your shell environment.

For better security, download the installer, verify its checksum, then run it.
The example below uses SHA256.
Replace the `EXPECTED_SHA256` value with the checksum published on the official Rust website before running.

```bash
# download the installer into /tmp
curl -fsSLo /tmp/sh.rustup.rs https://sh.rustup.rs

sha256sum -c sh.rustup.rs.sha256

# if the checksum matches, run the installer
sh /tmp/sh.rustup.rs -y

# enable cargo in the current shell
source $HOME/.cargo/env
```

If you open a fresh terminal in Codespaces the environment will usually be loaded automatically.
If not, re-run the `source` line or reload the shell.

3. Add the WebAssembly target required by Yew/Trunk:

```bash
rustup target add wasm32-unknown-unknown
```

4. Install Trunk (the build/serve tool for Yew WASM apps):

```bash
cargo install trunk
```

(Optional) Install `wasm-opt` from the binary releases (recommended for smaller release bundles).
On Ubuntu you can install `binaryen` or download `wasm-opt` from the Binaryen project.

### Development (hot-reload)

Run the dev server (hot reload):

```bash
trunk serve
```

Trunk will serve the app using `index.html` and the wasm build from `src/main.rs`.
By default it listens on `http://127.0.0.1:8080`.

### Building for Production

Build an optimized production bundle:

```bash
trunk build --release
```

The output will be written to the `dist/` directory.

### Quick "Try it" (Codespaces)

1. Open the Codespace terminal.
2. Run the install steps above (rustup + trunk).
3. Start the dev server:

```bash
trunk serve --open
```

This will start the server and open a browser preview in Codespaces (if available).

### Project Structure

- `src/main.rs` - Main application code with Yew components
- `index.html` - HTML template for Trunk
- `Cargo.toml` - Rust project dependencies and configuration

