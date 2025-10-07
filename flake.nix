{
  description = "Vue.js frontend application";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    {
      nixosModules.default = {config, lib, pkgs, ...}:
        import ./nix/service.nix {
          inherit config lib pkgs;
          backend = self.packages.${pkgs.system}.backend;
          frontend = self.packages.${pkgs.system}.frontend;
        };
    } // flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        frontend = pkgs.callPackage ./frontend {};
        backend = pkgs.callPackage ./backend {};
      in
      {
        packages.frontend = frontend;
        packages.backend = backend;
        packages.default = pkgs.writeShellScriptBin "submission" ''
          ${backend}/bin/submission --static-dir "${frontend}" "$@"
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
