{
  description = "Solana Toolbox CLI - A toolbox for interacting with Solana programs";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rustfmt" "clippy" ];
        };

        # Native build inputs for Solana development
        nativeBuildInputs = with pkgs; [
          pkg-config
          openssl
          rustToolchain
        ];

        # Build inputs for Solana development
        buildInputs = with pkgs; [
          openssl
          libudev-zero
          libusb1
        ] ++ lib.optionals stdenv.isDarwin [
          darwin.apple_sdk.frameworks.AppKit
          darwin.apple_sdk.frameworks.IOKit
        ];

        # Environment variables for the build
        env = {
          OPENSSL_NO_VENDOR = "1";
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
          OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
        };

      in
      {
        packages = {
          default = self.packages.${system}.solana-toolbox-cli;
          
          solana-toolbox-cli = pkgs.rustPlatform.buildRustPackage {
            pname = "solana-toolbox-cli";
            version = "0.4.2";

            src = ./solana_toolbox_cli;

            cargoLock = {
              lockFile = ./solana_toolbox_cli/Cargo.lock;
            };

            inherit nativeBuildInputs buildInputs;
            inherit env;

            # Skip tests during build as they may require network access
            doCheck = false;

            meta = with pkgs.lib; {
              description = "Toolbox for interacting with Solana programs through CLI";
              homepage = "https://github.com/your-username/solana-toolbox";
              license = licenses.mit;
              maintainers = [ ];
              platforms = platforms.all;
            };
          };
        };

        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;
          inherit env;
          
          shellHook = ''
            echo "Solana Toolbox CLI development environment"
            echo "Available commands:"
            echo "  cargo build --release"
            echo "  cargo test"
            echo "  cargo run -- --help"
          '';
        };

        # Formatter for nix files
        formatter = pkgs.nixpkgs-fmt;
      });
}