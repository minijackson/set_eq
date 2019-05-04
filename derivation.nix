{ stdenv, rustPlatform, dbus, pkgconfig, ... }:

rustPlatform.buildRustPackage rec {
  name = "set_eq-${version}";
  version = "0.1.0";

  buildInputs = [ dbus pkgconfig ];

  src = ./.;
  cargoSha256 = "0qc2fapqsri244sm85ykpdq2icwvg99fdb9nqpf0fmhzh8m9plhy";

  preFixup = ''
    mkdir -p "$out/share/"{bash-completion/completions,fish/vendor_completions.d,zsh/site-functions}
    cp target/release/build/set_eq-*/out/set_eq.bash "$out/share/bash-completion/completions/"
    cp target/release/build/set_eq-*/out/set_eq.fish "$out/share/fish/vendor_completions.d/"
    cp target/release/build/set_eq-*/out/_set_eq     "$out/share/zsh/site-functions/"
  '';

  checkPhase = ''
    runHook preCheck

    echo "Running cargo test --release"
    cargo test --release

    runHook postCheck
  '';

  meta = with stdenv.lib; {
    description = "A command-line tool to manipulate PulseAudio's equalizer";
    homepage = https://github.com/minijackson/set_eq;
    platforms = platforms.all;
  };
}
