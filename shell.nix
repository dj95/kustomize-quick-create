{ pkgs ? import <nixpkgs> {} }:
with pkgs;
let
  pinnedPkgs = fetchFromGitHub {
    owner = "NixOS";
    repo = "nixpkgs";
    rev = "1a411f23ba299db155a5b45d5e145b85a7aafc42";
    sha256 = "sha256-6Va9iVtmmsw4raBc3QKvQT2KT/NGRWlvUlJj46zN8B8=";
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
