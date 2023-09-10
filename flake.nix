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
            cargo-watch
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
              ] ++ pkgs.vscode-utils.extensionsFromVscodeMarketplace [
                {
                  name = "vsc-material-theme";
                  publisher = "Equinusocio";
                  version = "33.8.0";
                  sha256 = "sha256-+I4AUwsrElT62XNvmuAC2iBfHfjNYY0bmAqzQvfwUYM=";
                }
                {
                  name = "material-icon-theme";
                  publisher = "PKief";
                  version = "4.28.0";
                  sha256 = "sha256-DO3dcJPk+TMhfb0IJ/eTB7nIKfyCXIiyhZFBpZjJzsM=";
                }
              ];
            })
          ];
        };
      });
}
