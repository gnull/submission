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
          submission = self.packages.${pkgs.system}.default;
        };
    } // flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        frontend = pkgs.callPackage ./frontend {};
        backend = pkgs.callPackage ./backend {};
        main = pkgs.symlinkJoin {
          name = "submission";
          paths = [ frontend backend ];
          buildInputs = [ pkgs.makeWrapper ];

          postBuild = ''
            wrapProgram $out/bin/submission \
              --add-flags "--static-dir $out/var/www" \
          '';
        };
      in
      {
        packages.default = main;

        # Keep your dev shell for development
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            nodejs_20
            cargo
          ];
        };
      });
}
