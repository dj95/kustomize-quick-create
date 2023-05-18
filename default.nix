{ pkgs, ... }:
{
  kustomize-quick-create = pkgs.rustPlatform.buildRustPackage {
    pname = "kustomize-quick-create";
    version = "0.1.0";
    src = ./.;

    cargoLock = {
      lockFile = ./Cargo.lock;
    };

    nativeBuildInputs = [
      pkgs.pkg-config
      pkgs.libiconv
    ];
  };
}
