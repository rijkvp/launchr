{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };
  outputs =
    {
      nixpkgs,
      flake-utils,
      crane,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain (p: p.rust-bin.stable.latest.default.override { });

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

          doCheck = false;
        };

        rpath =
          with pkgs;
          lib.makeLibraryPath [
            fontconfig
            wayland
            libxkbcommon
            xorg.libX11
            xorg.libXcursor
          ];

        launchr = craneLib.buildPackage (
          commonArgs
          // {
            cargoArtifacts = craneLib.buildDepsOnly commonArgs;
            postFixup = ''
              patchelf $out/bin/launchr --add-rpath ${rpath}
            '';
          }
        );
      in
      {
        packages.default = launchr;

        apps.default = flake-utils.lib.mkApp {
          drv = launchr;
        };

        devShells.default = craneLib.devShell {

          packages = with pkgs; [
            cargo-flamegraph
            cargo-edit
            gnuplot
          ];

          LD_LIBRARY_PATH = rpath;
        };
      }
    );
}
