## Installing

This web app is packaged as a Nixos system config.
To install it and enable the systemd service, import this flake as `submission` and then add the following to your config.

```nix
modules [ submission.nixosModules.default ];

services.submission-web = {
  enable = true;
  host = "0.0.0.0";
  port = 8080;
  # Optionally, set the storage path:
  # dataDir = "/some/path";
};
```

## Running

### Run with Nix

This web service is packages with Nix flakes, and you can both build and run it with a single command.
The following command just runs the service listening on port 8080, storing sqlite DB and files in the current directory.

```
nix run
```

You can pass it extra commandline options like so.

```
nix run .#default -- --host 0.0.0.0 --port 80
```

It also accepts `--database` and `--uploads` options to choose where to store data.
See `--help` for more.

### Manually

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
cd backend
cargo run -- --static-dir ../frontend/static --host 0.0.0.0 --port 0000
```
