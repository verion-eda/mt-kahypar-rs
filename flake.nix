{
  description = "mt-kahypar-rs: static rust bindings for mt-kahypar";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      flake-utils,
      nixpkgs,
      rust-overlay,
    }:

    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [
          (import rust-overlay)
          (self: super: {
            rustToolchain = pkgs.symlinkJoin {
              name = "rust-toolchain";
              paths = [
                (super.rust-bin.stable.latest.minimal.override {
                  extensions = [
                    "clippy"
                    "rust-docs"
                  ];
                })
                (super.rust-bin.selectLatestNightlyWith (toolchain: toolchain.rustfmt))
              ];
            };
          })
        ];

        pkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnfree = true;
        };

        common = with pkgs; [
          rustToolchain
          pkg-config
          cargo-deny
          cargo-edit
          cargo-watch
          cargo-machete
          rust-analyzer
          pinact
          cmake
          hwloc
        ];

        libs = map (pkgs.lib.getOutput "lib") (
          with pkgs;
          [
            stdenv.cc.cc
            zlib
            glib
            hwloc
          ]
        );
      in
      {
        devShells.default = pkgs.mkShell {
          packages = common;
          libs = libs;
        };
      }
    );
}
