{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
  in {
    devShells."${system}".default = pkgs.mkShell {
      buildInputs = with pkgs; [
        cargo
        rustc
        rustfmt
        clippy
        rust-analyzer
        glib
      ];

      nativeBuildInputs = with pkgs; [
        pkg-config
      ];

      env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    };

    packages."${system}".default = pkgs.rustPlatform.buildRustPackage {
      name = "taksnotes";
      src = ./.;
      buildInputs = with pkgs; [
        glib
      ];
      nativeBuildInputs = with pkgs; [
        pkg-config
      ];

      cargoLock.lockFile = ./Cargo.lock;
    };
  };
}
