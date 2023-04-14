{
  description = "Nix development environment for Ziggurat";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Latest stable rust without rustfmt.
        stable-rust = pkgs.rust-bin.stable.latest.minimal.override {
          extensions = [ "clippy" "rust-docs" ];
        };
        # Latest nightly rust with rustfmt.
        nightly-rust = pkgs.rust-bin.selectLatestNightlyWith
          (toolchain: toolchain.minimal.override {
            extensions = [ "rustfmt" ];
          });

        localCiScript = pkgs.writeScriptBin "ci-local" ''
          echo "Running cargo check..."
          cargo check --all-targets

          echo "Running cargo fmt check..."
          cargo fmt --all -- --check

          echo "Running cargo clippy..."
          cargo clippy --all-targets -- -D warnings

          echo "Running cargo-sort check..."
          cargo-sort -cw
        '';

        devShell = pkgs.mkShell {
          buildInputs = [ localCiScript ] ++ (with pkgs; [
            stable-rust
            nightly-rust
            cargo-sort
            openssl
            pkg-config
          ]);
        };
      in
      {
        devShells.default = devShell;
      });
}
