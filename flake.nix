{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        inherit (pkgs) lib;

        rust-toolchain = pkgs.rust-bin.stable.latest.default;
        rust-dev-toolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rust-toolchain;

        cssFilter = path: _type: builtins.match ".*css$" path != null;
        cssOrCargo = path: type: (cssFilter path type) || (craneLib.filterCargoSources path type);
        src = lib.cleanSourceWith {
          src = craneLib.path ./.;
          filter = cssOrCargo;
        };

        buildInputs = [
          pkgs.bintools
        ] ++ lib.optionals pkgs.stdenv.isDarwin [
          # Additional darwin specific inputs can be set here
          pkgs.libiconv
        ];

        cargoArtifacts = craneLib.buildDepsOnly {
          inherit src buildInputs;
        };

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        nucleoid-contributors-gen = craneLib.buildPackage {
          inherit cargoArtifacts src buildInputs;
        };
      in
    {
      checks = {
        # Build the crate as part of `nix flake check` for convenience
        inherit nucleoid-contributors-gen;

        # Run clippy (and deny all warnings) on the crate source,
        # again, resuing the dependency artifacts from above.
        #
        # Note that this is done as a separate derivation so that
        # we can block the CI if there are issues here, but not
        # prevent downstream consumers from building our crate by itself.
        nucleoid-contributors-gen-clippy = craneLib.cargoClippy {
          inherit cargoArtifacts src buildInputs;
          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        };

        # Check formatting
        nucleoid-contributors-gen-fmt = craneLib.cargoFmt {
          inherit src;
        };
      };

      packages.default = pkgs.stdenv.mkDerivation rec {
        pname = "nucleoid-contributors";
        version = "devel";
        src = ./.;
        nativeBuildInputs = [ nucleoid-contributors-gen ];
        buildPhase = "nucleoid-contributors";
        installPhase = "cp -r build $out";
      };
      packages.nucleoid-contributors-gen = nucleoid-contributors-gen;

      devShells.default = pkgs.mkShell {
        inputsFrom = builtins.attrValues self.checks;

        # Extra inputs can be added here
        nativeBuildInputs = with pkgs; [
          rust-dev-toolchain
        ];
      };
    });
}
