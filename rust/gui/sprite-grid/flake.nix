{
  description = "Set up a devenv for winit";

  inputs = {
    nixpkgs.url = github:nixos/nixpkgs/nixpkgs-unstable;
    rust-overlay = {
      url = github:oxalica/rust-overlay;
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }:
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
    in with pkgs; rec {
      devShells.${system}.default = mkShell rec {
        buildInputs = [
          # Rust
          rust-bin.stable.latest.default

          # For wayland support
          wayland

          # For x11 support
          xorg.libX11
        ];
        
        LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
      };
    };
}
