{ nixpkgs ? <nixpkgs>, declInput }:

let
  pkgs = import nixpkgs {
    config = {};
  };
in {
  jobsets = pkgs.runCommand "spec.json" {} ''
    cat << EOF
    ${builtins.toXML declInput}
    EOF
    cat > $out <<EOF
    {
      "master": {
        "enabled": 1,
        "hidden": false,
        "description": "set_eq Master branch",
        "nixexprinput": "src",
        "nixexprpath": "default.nix",
        "checkinterval": 60,
        "schedulingshares": 1,
        "enableemail": false,
        "emailoverride": "",
        "keepnr": 10,
        "inputs": {
            "src": { "type": "git", "value": "https://git.huh.gdn/set_eq/ master", "emailresponsible": true },
            "nixpkgs": { "type": "git", "value": "https://github.com/nixos/nixpkgs-channels nixos-18.03", "emailresponsible": false }
      }
    }
    EOF
  '';
}
