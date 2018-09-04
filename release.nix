{ nixpkgs ? import <nixpkgs>, ... }:

let
  pkgs = nixpkgs {
    config = {};
  };
in {
  set_eq = pkgs.callPackage ./derivation.nix {};
}
