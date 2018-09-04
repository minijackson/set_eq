{ nixpkgs ? import <nixpkgs>, ... }:

let
  pkgs = nixpkgs {
    config = {};
    overlays = [
      (import ./overlay.nix)
    ];
  };
in pkgs.set_eq
