with import (builtins.fetchGit {
  # Descriptive name to make the store path easier to identify
  name = "nixos-unstable-april-2022";
  url = "https://github.com/nixos/nixpkgs/";
  # `git ls-remote https://github.com/nixos/nixpkgs nixos-unstable`
  ref = "refs/heads/nixpkgs-unstable";
  rev = "b80f570a92d04e8ace67ff09c34aa48708a5c88c";
}) {};
  mkShell {
    nativeBuildInputs = [
      bashInteractive
      rust-analyzer
      rustc
      libiconv
      cargo
      rustfmt
      clippy
    ];
  }
