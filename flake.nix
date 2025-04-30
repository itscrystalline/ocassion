{
  description = "Nix flake for occasion";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
      };
      lib = pkgs.lib;
      buildRustPackage = pkgs.rustPackages.rustPlatform.buildRustPackage;
      fetch = {
        rev,
        hash,
      }:
        pkgs.fetchFromGitHub {
          owner = "itscrystalline";
          repo = "occasion";
          rev = rev;
          hash = hash;
        };

      package = source: ver:
        buildRustPackage (finalAttrs: rec {
          pname = "occasion";
          version = ver;

          src = source;

          cargoLock = {
            lockFile = src + "/Cargo.lock";
          };

          meta = {
            description = "A small program to print something / run a command on a specific time/timeframe. ";
            homepage = "https://github.com/itscrystalline/occasion";
            license = lib.licenses.unlicense;
            maintainers = [];
          };
        });
    in rec {
      packages.occasion-latest = package ./. "0.2.0";
      packages.occasion = package (fetch {
        rev = "b58126b4e46f72fb9c64f45aa5b92001bcf1841f";
        hash = "sha256-h5uZ/vht39qPLaSlhfUp20uJgivUsKWOmQLdmj402HU=";
      }) "0.2.0";
      packages.default = packages.occasion;
    })
    // flake-utils.lib.eachDefaultSystemPassThrough (system: {
      homeManagerModule = import ./module.nix {inherit (self.packages.${system}) occasion;};
    });
}
