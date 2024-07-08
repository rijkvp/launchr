{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };
  outputs = { self, nixpkgs, flake-utils, crane, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system; 
          overlays = [ (import rust-overlay) ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain (p: p.rust-bin.stable.latest.default.override {});

        commonArgs = {
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;

          buildInputs = with pkgs; [
            fontconfig
            wayland
            libxkbcommon
            xorg.libX11
            xorg.libXcursor
          ];
          nativeBuildInputs = with pkgs; [ pkg-config ];
        };

        rpath = with pkgs; lib.makeLibraryPath [
         fontconfig
         wayland
         libxkbcommon
         xorg.libX11
         xorg.libXcursor
        ];

        launcher = craneLib.buildPackage (commonArgs // {
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
          postFixup = ''
            patchelf $out/bin/launcher --add-rpath ${rpath}
          '';
        });
      in
      {
        checks = { inherit launcher; };

        packages.default = launcher;

        apps.default = flake-utils.lib.mkApp {
          drv = launcher;
        };

        devShells.default = craneLib.devShell {
          checks = self.checks.${system};

          packages = with pkgs; [
            cargo-flamegraph
            cargo-outdated
            gnuplot
          ];

          LD_LIBRARY_PATH = rpath;
        };
      });
}
