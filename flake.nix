{
  description = "Rust + GTK4 + SQLCipher dev environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rust = pkgs.rust-bin.stable.latest.default;

        nativeBuildInputs = with pkgs; [
          pkg-config
          glib
          blueprint-compiler
          librsvg
          makeWrapper
        ];

        buildInputs = with pkgs; [
          rust
          gtk4
          sqlcipher
          glib
          cairo
          gdk-pixbuf
          pango
          atk
          libsoup_3
          webkitgtk_6_0
          libshumate
          libadwaita
          openssl
          blueprint-compiler
          librsvg
          gsettings-desktop-schemas
        ];
      in {
        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;
          shellHook = ''
            echo ${pkgs.librsvg}
          '';
        };
        packages.default = pkgs.rustPlatform.buildRustPackage (finalAttrs: {
          inherit nativeBuildInputs buildInputs;
          pname = "opensignal";
          version = "1.0.0";
          src = ./.;
          cargoHash = "sha256-/0m5q0VIMZikaniYEk52byc+ToJxiwiJefr1kS2Nl5M=";
          buildFeatures = [ "binary" ];
          cargoBuildFlags = [ "--bin" "opensignal" ];
          cargoInstallFlags = [ "--bin" "opensignal" ];
          doCheck = false;
          postInstall = ''
            cp -r $src/icons $out/bin/icons
            wrapProgram $out/bin/opensignal --set LD_LIBRARY_PATH ${pkgs.librsvg}/lib --set GDK_PIXBUF_MODULE_FILE ${pkgs.librsvg}/lib/gdk-pixbuf-2.0/2.10.0/loaders.cache
          '';
        });
      }
    );
}
