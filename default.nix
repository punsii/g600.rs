{ pkgs, ... }:
pkgs.rustPlatform.buildRustPackage {
  pname = "brightness";
  version = "1.1.0";

  src = ./.;
  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}
