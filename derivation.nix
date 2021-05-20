{ stdenv, rustPlatform, dbus, installShellFiles, ... }:

rustPlatform.buildRustPackage rec {
  name = "set_eq-${version}";
  version = "0.1.0";

  nativeBuildInputs = [ installShellFiles ];
  buildInputs = [ dbus ];

  src = ./.;
  cargoSha256 = "0639d4b5f3vxljinm59vh7kfvqqm07iah90gqb10l91xpc9lj2b5";

  postInstall = ''
    installShellCompletion --bash target/release/build/set_eq-*/out/set_eq.bash
    installShellCompletion --fish target/release/build/set_eq-*/out/set_eq.fish
    installShellCompletion --zsh  target/release/build/set_eq-*/out/_set_eq
  '';

  meta = with stdenv.lib; {
    description = "A command-line tool to manipulate PulseAudio's equalizer";
    homepage = https://github.com/minijackson/set_eq;
    platforms = platforms.all;
  };
}
