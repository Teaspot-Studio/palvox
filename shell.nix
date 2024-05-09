with import ./nix/pkgs.nix {};
let 
  merged-openssl = symlinkJoin { name = "merged-openssl"; paths = [ openssl.out openssl.dev ]; };
in stdenv.mkDerivation rec {
  name = "rust-env";
  env = buildEnv { name = name; paths = buildInputs; };

  buildInputs = [
    rustup
    clang
    cmake
    llvm
    llvmPackages.libclang
    openssl
    cacert
    pkg-config
    xorg.libX11
    xorg.libXrandr
    xorg.libXinerama
    xorg.libXcursor
    xorg.libXi
  ];
  LIBCLANG_PATH="${llvmPackages.libclang}/lib";
  OPENSSL_DIR="${merged-openssl}";
  LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${ lib.makeLibraryPath [
    libGL
    xorg.libX11
    xorg.libXi
  ] }";
}
