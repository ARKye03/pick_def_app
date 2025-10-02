{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        version = builtins.replaceStrings [ "\n" ] [ "" ] (builtins.readFile ./version);

        pkg-dependencies = with pkgs; [
          pkg-config
          libadwaita
          gtk4
          blueprint-compiler
        ];

        pick_def_app = pkgs.rustPlatform.buildRustPackage {
          pname = "pick_def_app";
          version = version;
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
            blueprint-compiler
          ];

          buildInputs = pkg-dependencies;
        };
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              openssl
              pkg-config
              rust-bin.stable.latest.default
            ]
            ++ pkg-dependencies;
          };
        packages.default = pick_def_app;
      }
    );
}
