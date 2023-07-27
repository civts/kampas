{
  description = "Rust env";

  inputs = {
    nixpkgs.url = "github:nixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        devShell = with pkgs; mkShell rec {
          #ENV_VARIABLE_1 = "test";
          buildInputs = [
            pkg-config
            stdenv.cc
            crate2nix
            rustc
            clippy
            rustfmt
            cargo
            rustup
            gcc
            nodejs
            (vscode-with-extensions.override {
              vscode = vscodium;
              vscodeExtensions = with vscode-extensions; [
                jnoortheen.nix-ide
                matklad.rust-analyzer
                vadimcn.vscode-lldb
                tamasfe.even-better-toml
                esbenp.prettier-vscode
                ms-vscode.hexeditor
                svelte.svelte-vscode
                ms-azuretools.vscode-docker
              ];
            })
          ];
        };
      });
}
