{ pkgs, lib }:
pkgs.rustPlatform.buildRustPackage {
  pname = "submission-backend";
  version = "0.1.0";

  src = ./.;

  # This is the hash of your Cargo.lock dependencies
  # Set it to an empty string first, then Nix will tell you the correct hash
  cargoHash = "sha256-z7IzNJ2xP077oytfbJDpklvm9FHxIo/gDsKx9pBGMo0=";

  # Optional: if you need additional build inputs
  nativeBuildInputs = with pkgs; [
    pkg-config
  ];

  buildInputs = with pkgs; [
    # openssl
    # Add other runtime dependencies here
  ];

  # Optional: set custom build flags
  # cargoTestFlags = [];
  # cargoBuildFlags = [];

  # Optional: skip tests if they require network or are problematic
  # doCheck = false;

  meta = with lib; {
    description = "My Rust backend server";
    homepage = "https://github.com/gnull/submission";
    license = licenses.mit;
    maintainers = [ ];
  };
}
