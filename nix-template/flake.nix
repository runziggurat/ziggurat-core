{
  description = "Nix development environment for Ziggurat";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    ziggurat-core.url = "github:runziggurat/ziggurat-core";
  };

  outputs = { nixpkgs, flake-utils, ziggurat-core, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        # You can define additional CI scripts here, e.g.
        # scripts = {
        #   test-ignored = "cargo test --ignored";
        # } // ziggurat-core.scripts;
        scripts = { } // ziggurat-core.scripts;
      in
      {
        devShells.default = pkgs.mkShell {
          # Enter additional build dependencies here.
          buildInputs = [ ]
            # Contains all the necessities.
            ++ ziggurat-core.buildInputs.${system}
            ++ (ziggurat-core.lib.${system}.mkCiScripts scripts);
        };
      });
}
