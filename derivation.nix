{ stdenv, rustPlatform, dbus, pkgconfig, ... }:

rustPlatform.buildRustPackage rec {
  name = "set_eq-${version}";
  version = "0.1.0";

  buildInputs = [ dbus pkgconfig ];

  src = ./.;
  cargoSha256 = "0lknxqr1pfbj6z981rw5ppkklknryyafl5f552aaw4iqhq94slq4";

  meta = with stdenv.lib; {
    description = "A command-line tool to manipulate PulseAudio's equalizer";
    homepage = https://github.com/minijackson/set_eq;
    platforms = platforms.all;
  };
}
