## Running

```sh
# Get the required packages
nix-shell -p rustup -p nodejs_24
rustup toolchain install nightly
rustup default nightly
```

```sh
# Run Rust backend
cargo run

# Run Vue.js frontend
cd frontend
npm install
npm run run dev
```
