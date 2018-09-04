{ nixpkgs ? <nixpkgs>, ... }:

let
  pkgs = import nixpkgs {
    config = {};
  };
in {
  set_eq = pkgs.callPackage ./derivation.nix {};
}
