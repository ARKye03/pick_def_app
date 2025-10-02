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

        pick_def_app = pkgs.stdenv.mkDerivation {
          pname = "pick_def_app";
          version = version;
          src = ./.;
          buildInputs = [ pkgs.rust-bin.stable.latest.default ];
          cargoBuildFlags = [ "--release" ];
          installPhase = ''
            mkdir -p $out/bin
            cp target/release/my-rust-app $out/bin/
          '';
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
            ];
          };
        packages.default = pick_def_app;
      }
    );
}
