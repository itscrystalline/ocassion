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
      mkDerivation = pkgs.stdenvNoCC.mkDerivation;
      buildRustPackage = pkgs.rustPackages.rustPlatform.buildRustPackage;

      releaseName = lib.strings.concatMapStringsSep "-" (n:
        if n == "darwin"
        then "macos"
        else n) (lib.lists.reverseList (lib.strings.splitString "-" system));

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

      binary = ver: hash:
        mkDerivation (finalAttrs: {
          pname = "occasion";
          version = ver;

          src = pkgs.fetchurl {
            url = "https://github.com/itscrystalline/occasion/releases/download/v${ver}/occasion-${releaseName}.tar.gz";
            sha256 = hash;
          };

          dontUnpack = true;
          dontConfigure = true;
          dontBuild = true;
          installPhase = ''
            runHook preInstall

            tar xvzf $src
            mkdir -p $out/bin
            cp occasion $out/bin/

            runHook postInstall
          '';

          meta = {
            description = "A small program to print something / run a command on a specific time/timeframe. ";
            homepage = "https://github.com/itscrystalline/occasion";
            license = lib.licenses.unlicense;
            maintainers = [];
          };
        });
    in rec {
      packages.occasion-latest = package ./. "0.3.0";
      packages.occasion = binary "0.2.0" "sha256-v8cAbAOgDJey+/07AHFVY3quwXmPHow08jbfAQGD5NM=";
      packages.default = packages.occasion;
    })
    // flake-utils.lib.eachDefaultSystemPassThrough (system: {
      homeManagerModule = import ./module.nix {inherit (self.packages.${system}) occasion;};
    });
}
