{

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    systems.url = "github:nix-systems/x86_64-linux";
    treefmt = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nix-checks = {
      url = "github:huuff/nix-checks";
    };
    pre-commit = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    utils = {
      url = "github:numtide/flake-utils";
      inputs.systems.follows = "systems";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
      rust-overlay,
      treefmt,
      pre-commit,
      nix-checks,
      ...
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [
          (import rust-overlay)
        ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustPkgs = pkgs.rust-bin.stable.latest.default.override {
          targets = [
            "x86_64-unknown-linux-musl"
            "wasm32-unknown-unknown"
          ];
        };
        treefmt-build = (treefmt.lib.evalModule pkgs (import ./treefmt.nix)).config.build;
        pre-commit-check = pre-commit.lib.${system}.run {
          src = ./.;
          hooks = import ./pre-commit.nix {
            inherit pkgs rustPkgs;
            treefmt = treefmt-build.wrapper;
          };
        };
        inherit (nix-checks.lib.${system}) checks;
      in
      {
        checks = {
          formatting = treefmt-build.check self;
          statix = checks.statix ./.;
          deadnix = checks.deadnix ./.;
          flake-checker = checks.flake-checker ./.;
          #clippy = checks.clippy ./.;
        };

        formatter = treefmt-build.wrapper;

        devShells.default =
          with pkgs;
          mkShell {
            inherit (pre-commit-check) shellHook;
            buildInputs = [

              rustPkgs
              rust-analyzer

              cargo-leptos
              cargo-expand # expand macros
              cargo-generate # required for cargo-leptos
              dart-sass
              binaryen # required for release compilation
              leptosfmt # formatting leptos view! macro
              stylance-cli # bundle sass
              wasm-pack # to test wasm

              # nix
              nil
              nixfmt-rfc-style
            ];
          };
      }
    );
}
