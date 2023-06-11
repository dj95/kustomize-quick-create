{ pkgs ? import <nixpkgs> {} }:
with pkgs;
let
  pinnedPkgs = fetchFromGitHub {
    owner = "NixOS";
    repo = "nixpkgs";
    rev = "a30520bf8eabf8a5c37889d661e67a2dbcaa59e6";
    sha256 = "sha256-nGWNQVhSw4VSL+S0D0cbrNR9vs9Bq7rlYR+1K5f5j6w=";
  };

  pkgs = import pinnedPkgs {};

  inherit (lib) optional optionals;
  inherit (darwin.apple_sdk.frameworks) Cocoa CoreGraphics Foundation IOKit Kernel OpenGL Security;
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    libiconv
    rustc
  ] ++ optionals stdenv.isDarwin [
    Cocoa
    CoreGraphics
    Foundation
    IOKit
    Kernel
    OpenGL
    Security
    libpng
    zlib
  ];
}
