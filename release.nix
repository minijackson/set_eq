{ nixpkgs ? import <nixpkgs>, ... }:

let
  pkgs = nixpkgs {
    config = {};
  };
{
  set_eq = pkgs.callPackage ./derivation.nix {};
}
