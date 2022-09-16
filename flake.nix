{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.dream2nix = { url = "github:nix-community/dream2nix"; inputs.nixpkgs.follows = "nixpkgs"; };
  inputs.fenix = { url = "github:nix-community/fenix"; inputs.nixpkgs.follows = "nixpkgs"; };

  outputs = { self, nixpkgs, dream2nix, fenix }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
      channelVersion = "1.62.0";
      toolchain = fenix.packages.x86_64-linux.toolchainOf {
        channel = channelVersion;
        sha256 = "sha256-AoqjoLifz8XrZWP7piauFfWCvhzPMLKxfv57h6Ng1oM=";
      };
    in
    (dream2nix.lib.makeFlakeOutputs {
      systems = [ "x86_64-linux" ];
      config.projectRoot = ./.;
      source = ./.;
      packageOverrides."^.*".set-toolchain.overrideRustToolchain = old: { inherit (toolchain) cargo rustc; };
    })
    // {
      devShells.x86_64-linux.default =
        let
          rust-toolchain = (pkgs.formats.toml { }).generate "rust-toolchain.toml" {
            toolchain = {
              channel = channelVersion;
              components = [ "rustc" "rustfmt" "rust-src" "cargo" "clippy" "rust-docs" ];
            };
          };
        in
        pkgs.mkShell {
          shellHook = "cp --no-preserve=mode ${rust-toolchain} rust-toolchain.toml";
          packages = [
            pkgs.rustup
            fenix.packages.x86_64-linux.rust-analyzer
            pkgs.cmake
            pkgs.pkg-config
            pkgs.fontconfig
          ];
        };
    };
}
