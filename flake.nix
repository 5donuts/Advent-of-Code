{
  description = "My Advent of Code puzzle solutions";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
  };

  outputs = { nixpkgs, crane, ... }:
    let
      # https://xeiaso.net/blog/nix-flakes-1-2022-02-21/
      systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
      nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; } );

      rustOverrides = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));
    in {
      devShells = forAllSystems (system:
        let
          pkgs = nixpkgsFor.${system};
        in {
          default = pkgs.mkShell {
            buildInputs = [
              pkgs.clang
              pkgs.llvmPackages_latest.bintools
              pkgs.rustup
            ];

            RUSTC_VERSION = rustOverrides.toolchain.channel;
          };
        });

      packages = forAllSystems (system:
      let
        pkgs = nixpkgsFor.${system};
      in {
        default = (crane.mkLib pkgs).buildPackage {
            src = ./.;
          };
        });
    };
}
