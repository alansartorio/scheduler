{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      flake-utils,
      nixpkgs,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        runtimeDependencies = with pkgs; [
        ];
        buildDependencies = with pkgs; [
          sqlite
        ];
      in
      {
        devShells = {
          default = pkgs.mkShell {
            buildInputs = [
              pkgs.cargo
              pkgs.pkg-config
            ]
            ++ buildDependencies
            ++ runtimeDependencies;
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (buildDependencies ++ runtimeDependencies);
          };
        };

      }
    );
}
