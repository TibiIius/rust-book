{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  buildInputs = [ cargo rustc rustfmt rustPackages.clippy rust-analyzer ];
  RUST_SRC_PATH = "${rust.packages.stable.rustPlatform.rustLibSrc}";
}
