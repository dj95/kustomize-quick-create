{ pkgs ? import <nixpkgs> {} }:
with pkgs;
let
  pinnedPkgs = fetchFromGitHub {
    owner = "NixOS";
    repo = "nixpkgs";
    rev = "639d4f17218568afd6494dbd807bebb2beb9d6b3";
    sha256 = "sha256-pmZoMRkTxEp/EIqUPScTUOOhpPOSLJSL4eahsiP2mxI=";
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
