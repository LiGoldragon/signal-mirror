{
  description = "signal-mirror - ordinary payload-blind Mirror Interface";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      fenix,
      crane,
      ...
    }:
    flake-utils.lib.eachSystem [
      "x86_64-linux"
      "aarch64-linux"
      "aarch64-darwin"
    ] (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        toolchain = fenix.packages.${system}.stable.withComponents [
          "cargo"
          "rustc"
          "rustfmt"
          "clippy"
          "rust-src"
        ];
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
        ethosFilter = path: _type: builtins.match ".*/ethos(/.*)?$" path != null;
        examplesFilter = path: _type: builtins.match ".*/examples(/.*)?$" path != null;
        sourceFilter = path: type:
          type == "directory"
          || craneLib.filterCargoSources path type
          || ethosFilter path type
          || examplesFilter path type;
        src = pkgs.lib.cleanSourceWith {
          src = ./.;
          filter = sourceFilter;
          name = "source";
        };
        commonArgs = {
          inherit src;
          strictDeps = true;
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      in
      {
        packages.default = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
        checks = {
          build = craneLib.cargoBuild (commonArgs // { inherit cargoArtifacts; });
          test = craneLib.cargoTest (commonArgs // { inherit cargoArtifacts; });
          test-round-trip = craneLib.cargoTest (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoTestExtraArgs = "--test round_trip";
            }
          );
          test-canonical = craneLib.cargoTest (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoTestExtraArgs = "--features datom --test canonical";
            }
          );
          doc = craneLib.cargoDoc (
            commonArgs
            // {
              inherit cargoArtifacts;
              RUSTDOCFLAGS = "-D warnings";
              cargoDocExtraArgs = "--all-features";
            }
          );
          fmt = craneLib.cargoFmt { inherit src; };
          clippy = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets --all-features -- -D warnings";
            }
          );
        };
        devShells.default = pkgs.mkShell {
          name = "signal-mirror";
          packages = [ pkgs.jujutsu pkgs.pkg-config toolchain ];
        };
      }
    );
}
