{ ... }:

let
  pkgs = import (fetchTarball "https://github.com/nixos/nixpkgs/archive/21.11.tar.gz") { };
in
pkgs.mkShell {
  buildInputs = [
    pkgs.cargo
    pkgs.rustc
  ];
}
