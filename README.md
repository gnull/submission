## Running

Install dependencies:

```sh
# Get the required packages
nix-shell -p rustup -p nodejs_24
rustup toolchain install nightly
rustup default nightly
```

Build frontend:

```sh
# Run Vue.js frontend
cd frontend
npm install
npm run build
```

Build and run backend:

```sh
# Run Rust backend
cargo run
```
