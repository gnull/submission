{pkgs}:
pkgs.buildNpmPackage {
  pname = "submission-frontend";
  version = "0.1.0";

  src = ./.;

  # This is the hash of your node_modules
  # Set it to an empty string first, then Nix will tell you the correct hash
  npmDepsHash = "sha256-nfT4DOhfpeAx60DqF1JdN15fn2GNfLYctj9W+IufkOg=";

  # The build phase runs your npm build command
  buildPhase = ''
    npm run build
  '';

  # Install phase copies the built files
  installPhase = ''
    mkdir -p $out/var/www
    cp -r static/* $out/var/www
  '';
}
