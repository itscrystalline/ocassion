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

      binary = ver: hashes:
        mkDerivation (finalAttrs: {
          pname = "occasion";
          version = ver;

          src = pkgs.fetchurl {
            url = "https://github.com/itscrystalline/occasion/releases/download/v${ver}/occasion-${releaseName}.tar.gz";
            sha256 = hashes.${releaseName};
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
      packages.occasion = binary "0.3.0" {
        linux-x86_64 = "sha256-8WiJdD9AmbF2KVGteMbFwC54oS7XjAOa4r10jFB/1ds=";
        linux-aarch64 = "sha256-5iC6w14VCYD55LwymUZGE1sfo4GaX0JIiJUj/VcqO9c=";
        macos-x86_64 = "sha256-4iqtXKJxBRKTmWWFibCEdatdqDhFtdglUbJ43cI5MwE=";
        macos-aarch64 = "sha256-m8e5xXF5/jRdKtKDkHxxJJvmsgO4X6uQ8ByX7RmTPqQ=";
      };
      packages.default = packages.occasion;
    })
    // flake-utils.lib.eachDefaultSystemPassThrough (system: {
      homeManagerModule = import ./module.nix {inherit (self.packages.${system}) occasion;};
    });
}
