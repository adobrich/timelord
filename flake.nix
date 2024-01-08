{
  description = "TODO: add a description";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [rust-overlay.overlays.default];
      };
      libPath = with pkgs;
        lib.makeLibraryPath [
          libGL
          bzip2
          fontconfig
          freetype
          wayland
          libxkbcommon
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
        ];

      nativeBuildInputs = with pkgs; [
        pkg-config
        cmake
        mesa
        makeWrapper
        rust-analyzer
      ];

      buildInputs = with pkgs; [
        fontconfig
        freetype

        vulkan-headers
        vulkan-loader
        libGL

        libxkbcommon
        # WINIT_UNIX_BACKEND=wayland
        wayland

        # WINIT_UNIX_BACKEND=x11
        xorg.libXcursor
        xorg.libXrandr
        xorg.libXi
        xorg.libX11
      ];
    in
      with pkgs; {
        devShell = mkShell {
          inherit buildInputs nativeBuildInputs;

          packages = with pkgs; [
            (rust-bin.stable.latest.default.override {
              extensions = ["rust-src" "rust-analyzer" "clippy"];
            })
            cargo-watch
          ];
          LD_LIBRARY_PATH = "${libPath}";
        };
      });
  # // {
  #   overlay = final: prev: {
  #     inherit (self.packages.${final.system}) timelord;
  #   };
  # };
}
