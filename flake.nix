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
            wrapGAppsHook4
          ];

          buildInputs = with pkgs; [
            gtk4
            libadwaita
            glib
            cairo
            pango
            gdk-pixbuf
            graphene
          ];
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
              blueprint-compiler
              gtk4
              libadwaita
              glib
            ];
          };
        packages.default = pick_def_app;
      }
    );
}
