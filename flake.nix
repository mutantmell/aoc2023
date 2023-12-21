{
  description = "Example Rust development environment for Zero to Nix";

  # Flake inputs
  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixos-unstable;
    rust-overlay.url = github:oxalica/rust-overlay;
  };

  outputs = {
    self, nixpkgs, rust-overlay,
  }: let
    overlays = [
      (import rust-overlay)
      (self: super: {
        rustToolchain = super.rust-bin.stable.latest.default;
      })
    ];

    allSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
    forAllSystems = f: nixpkgs.lib.genAttrs allSystems (system: f {
      pkgs = import nixpkgs { inherit overlays system; };
    });
  in {
    devShells = forAllSystems ({ pkgs }: {
      default = pkgs.mkShell {
        packages = [
          pkgs.rustToolchain
        ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
          pkgs.libiconv
        ];
      };
    });
  };
}
