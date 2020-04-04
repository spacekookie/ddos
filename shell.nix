with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "ddos";
  buildInputs = with pkgs; [
    rustracer rustup clangStdenv cmake
  ];
}
