{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };
  outputs =
    {
      nixpkgs,
      flake-utils,
      crane,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.mkLib pkgs;

        commonArgs = {
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;
          buildInputs = with pkgs; [
            fontconfig
            wayland
            libxkbcommon
            libX11
            libXcursor
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
            libX11
            libXcursor
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
            cargo-edit
            cargo-machete
            cargo-flamegraph
            gnuplot
          ];
          LD_LIBRARY_PATH = rpath;
        };
      }
    );
}
