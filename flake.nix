{
  description = "basic rust development evnvironment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    wit-deps.url = "github:bytecodealliance/wit-deps";
  };

  outputs = {nixpkgs, rust-overlay, wit-deps, ...}:
      let 
        system = "x86_64-linux";
        pkgs = import nixpkgs { inherit system; overlays = [ rust-overlay.overlays.default ]; };
      in
    with pkgs; {
      devShells.${system}.default = mkShell {

          packages = [
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-std" "rust-src" "rust-analyzer" ];
              targets = [ "wasm32-wasi" ];
            })
            wit-deps.packages.${system}.wit-deps
            wabt
          ];
          
          nativeBuildInputs = [ ];
          
          buildInputs = [ ];
        };

      formatter.x86_64-linux = legacyPackages.${system}.nixpkgs-fmt;
    };
}

