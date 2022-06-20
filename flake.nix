{
  inputs =
    {
      cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
      nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
      rust-overlay.url = "github:oxalica/rust-overlay";
      flake-utils.url = "github:numtide/flake-utils";
    };

  outputs = { cargo2nix, flake-utils, nixpkgs, rust-overlay, ... }:
    with builtins;
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs =
            import nixpkgs
              {
                overlays =
                  [
                    cargo2nix.overlays.default
                    rust-overlay.overlay
                  ];

                inherit system;
              };

          rustChannel = "nightly";
          rustVersion = "2022-05-02";
          rustPkgs =
            pkgs.rustBuilder.makePackageSet
              {
                inherit rustChannel;
                inherit rustVersion;
                packageFun = import ./Cargo.nix;
                packageOverrides =
                  let
                    expat-sys = pkgs.rustBuilder.rustLib.makeOverride {
                      name = "expat-sys";
                      overrideAttrs = drv: {
                        propagatedBuildInputs = drv.propagatedBuildInputs or [ ] ++ [ pkgs.expat ];
                      };
                    };
                    freetype-sys = pkgs.rustBuilder.rustLib.makeOverride {
                      name = "freetype-sys";
                      overrideAttrs = drv: {
                        propagatedBuildInputs = drv.propagatedBuildInputs or [ ] ++ [ pkgs.freetype ];
                      };
                    };
                  in
                  pkgs: pkgs.rustBuilder.overrides.all ++ [ expat-sys freetype-sys ];
              };
        in
        {
          devShell =
            let
              rust-toolchain =
                (pkgs.formats.toml { }).generate "rust-toolchain.toml"
                  {
                    toolchain =
                      {
                        channel = rustChannel + "-" + rustVersion;

                        components =
                          [
                            "rustc"
                            "rust-src"
                            "cargo"
                            "clippy"
                            "rust-docs"
                          ];
                      };
                  };
            in
            rustPkgs.workspaceShell {
              nativeBuildInputs = with pkgs; [ rust-analyzer rustup ];
              shellHook =
                ''
                  cp --no-preserve=mode ${rust-toolchain} rust-toolchain.toml

                  export RUST_SRC_PATH=~/.rustup/toolchains/${rustChannel}-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/
                '';
            };
        }
      );
}
