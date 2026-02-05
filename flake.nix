{
  description = "Fractal - An egui application";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain
            cargo
            rustc
            rustfmt
            clippy
            rust-analyzer

            # System libraries required for egui/eframe
            pkg-config

            # X11 libraries
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr

            # Wayland libraries
            libxkbcommon
            wayland

            # Other dependencies
            libGL
          ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
            libGL
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            libxkbcommon
            wayland
          ]);

          shellHook = ''
            echo "Fractal development environment loaded"
            echo "Run 'cargo run' to start the application"
          '';
        };
      }
    );
}
