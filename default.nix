{ pkgs, ... }:
pkgs.rustPlatform.buildRustPackage {
  pname = "g600";
  version = "0.0.1";

  src = ./.;
  cargoLock = {
    lockFile = ./Cargo.lock;
  };
  buildInputs = with pkgs; [
    xdotool
  ];
}
