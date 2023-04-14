{
  description = "Nix development environment for Ziggurat";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem
      (system:
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

          buildInputs = with pkgs; [
            stable-rust
            nightly-rust
            cargo-sort
            openssl
            pkg-config
          ];
        in
        {
          inherit buildInputs;

          lib = import ./lib.nix { inherit pkgs; };

          devShells.default = pkgs.mkShell {
            buildInputs = self.buildInputs.${system}
            ++ (self.lib.${system}.mkCiScripts self.scripts);
          };
        }) // {
      scripts = {
        check = "cargo check --all-targets";
        fmt = "cargo fmt --all -- --check";
        clippy = "cargo clippy --all-targets -- -D warnings";
        sort = "cargo-sort --check --workspace";
      };
    };
}
