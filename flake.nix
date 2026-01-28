{
  description = "Monster Group Image Generation with diffusion-rs";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };
      rustToolchain = pkgs.rust-bin.stable.latest.default;
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustToolchain
          pkg-config
          cmake
          git
        ];

        shellHook = ''
          echo "🎨 Monster Image Generation Ready"
          echo "=================================="
          echo ""
          echo "Build: cargo build --release --example monster_gen"
          echo "Run: cargo run --release --example monster_gen"
          echo ""
        '';
      };
    };
}
