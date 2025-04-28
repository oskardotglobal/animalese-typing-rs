{
  pkgs ? import <nixpkgs> { },
  fenix ? import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") { },
}:
pkgs.mkShell {
  packages = with pkgs; [
    nixfmt-rfc-style

    mold-wrapped
    llvmPackages.libcxxClang
    pkg-config

    xorg.libX11
    alsa-lib-with-plugins

    fenix.complete.toolchain
  ];

  RUST_BACKTRACE = 1;
  RUST_SRC_PATH = "${fenix.complete.rust-src}/lib/rustlib/src/rust/library";
}
