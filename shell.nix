with import <nixpkgs> {};
pkgs.mkShell {
  buildInputs = [ clang ];
}
