let
  pkgs = (import <nixpkgs>) {};
in
  pkgs.stdenv.mkDerivation {
    name = "janet-native";

    LIBCLANG_PATH="${pkgs.llvmPackages.libclang}/lib";

    buildInputs = with pkgs; [
      cargo
      llvmPackages.libclang
      clang
      which
      rustfmt
    ];
  }