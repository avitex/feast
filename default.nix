with import <nixpkgs> {};

stdenv.mkDerivation {
    name = "feast-dev";
    buildInputs = [ gcc cargo rustc ];
}
