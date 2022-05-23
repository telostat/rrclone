let
  pkgs = import (fetchTarball "https://github.com/nixos/nixpkgs/archive/21.11.tar.gz") { };
  package = (pkgs.lib.modules.importTOML ./Cargo.toml).config.package;
in
pkgs.rustPlatform.buildRustPackage rec {
  pname = package.name;
  version = package.version;

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  meta = with pkgs.lib; {
    description = "Rclone wrapper to use YAML/JSON configuration for backup tasks";
    homepage = "https://github.com/telostat/rrclone";
    license = licenses.bsd3;
  };
}
