{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils/4022d587cbbfd70fe950c1e2083a02621806a725";
  };

  outputs = { self, nixpkgs, flake-utils }:

    flake-utils.lib.eachDefaultSystem (system:
      let
        p = import nixpkgs {
          inherit system;
          config = { allowUnfree = true; };
        };
      in
        {
          devShells = rec {
            default = nixpkgs.legacyPackages.${system}.mkShell {
              nativeBuildInputs = [ p.pkg-config ];
              packages = with p; [
                cargo
                clippy
                gnuplot
                iconv
                lldb_18
                rust-analyzer
                rustc
                rustfmt
              ];
            };
          };
        }
    );
}
