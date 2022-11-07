{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  buildInputs = [ cargo rustc rustfmt clippy rust-analyzer nixpkgs-fmt ];
  shellHook = ''
    export RUST_SRC_PATH="${rust.packages.stable.rustPlatform.rustLibSrc}"
  '';
}
