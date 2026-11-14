{
  description = "Native COSMIC™ Pomodoro timer for deep focus";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = pkgs.rust-bin.stable.latest.default;
        
        nativeBuildInputs = with pkgs; [
          rustVersion
          pkg-config
          cmake
          just
        ];

        buildInputs = with pkgs; [
          libxkbcommon
          fontconfig
          wayland
          alsa-lib
          dbus
          expat
          freetype
        ];
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "tomodorange";
          version = "0.1.2";
          src = ./.;
          cargoHash = "sha256-1sBjtQmD9eWzGBXKea6Wa1pccwtmKTy9WcygG1NdLBY=";
          nativeBuildInputs = [ pkgs.libcosmicAppHook ];
          buildInputs = buildInputs;
          
          postInstall = ''
            install -Dm644 io.github._1dragon_xyz.tomodorange.desktop $out/share/applications/io.github._1dragon_xyz.tomodorange.desktop
            install -Dm644 assets/icon.png $out/share/icons/hicolor/256x256/apps/io.github._1dragon_xyz.tomodorange.png
            install -Dm644 io.github._1dragon_xyz.tomodorange.metainfo.xml $out/share/metainfo/io.github._1dragon_xyz.tomodorange.metainfo.xml
          '';
        };

        devShells.default = pkgs.mkShell {
          buildInputs = nativeBuildInputs ++ buildInputs;
        };
      }
    );
}
