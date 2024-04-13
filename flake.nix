{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };
  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      with pkgs;
      {
        devShells.default = mkShell
          rec {
            nativeBuildInputs = [ rustc cargo clippy pkg-config ];
            buildInputs = [
              fontconfig
              wayland
              libxkbcommon
              xorg.libX11
              xorg.libXcursor
            ];
            LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
          };
      });
}
