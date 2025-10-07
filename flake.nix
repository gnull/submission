{
  description = "Vue.js frontend application";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        frontend = pkgs.callPackage ./frontend {};
        backend = pkgs.callPackage ./backend {};
      in
      {
        inherit frontend backend;
        packages.default = pkgs.writeShellScriptBin "submission" ''
          ${backend}/bin/submission --static-dir "${frontend}/var/www" "$@"
        '';

        # Keep your dev shell for development
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            nodejs_20
            cargo
          ];
        };
      });
}
