{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    # Based on: https://crane.dev/examples/quick-start-simple.html
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, flake-utils, crane, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        craneLib = crane.mkLib pkgs;

        commonArgs = rec {
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;

          buildInputs = with pkgs; [
            fontconfig
            wayland
            libxkbcommon
            xorg.libX11
            xorg.libXcursor
          ];
          nativeBuildInputs = with pkgs; [ clippy pkg-config ];
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        };

        launcher = craneLib.buildPackage (commonArgs // {
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        });
      in
      with pkgs;
      {
        checks = { inherit launcher; };

        packages.default = launcher;

        apps.default = flake-utils.lib.mkApp {
          drv = launcher;
        };

        devShells.default = craneLib.devShell {
          checks = self.checks.${system};

          packages = [
            cargo-flamegraph
          ];
        };
      });
}
