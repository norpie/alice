let
  pkgs = import <nixpkgs> {};
in
  pkgs.mkShell {
    buildInputs = with pkgs; [
      at-spi2-atk
      atkmm
      cairo
      gdk-pixbuf
      glib
      gobject-introspection
      gobject-introspection.dev
      gtk3
      harfbuzz
      librsvg
      clang
      libclang
      libsoup_3
      pango
      webkitgtk_4_1
      webkitgtk_4_1.dev
      pkg-config
    ];

    OPENSSL_NO_VENDOR = 1;
    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [openssl libclang]);
    OPENSSL_LIB_DIR = "${pkgs.lib.getLib pkgs.openssl}/lib";
    OPENSSL_DIR = "${pkgs.openssl.dev}";
    PKG_CONFIG_PATH = with pkgs; "\
            ${glib.dev}/lib/pkgconfig:\
            ${libsoup_3.dev}/lib/pkgconfig:\
            ${webkitgtk_4_1.dev}/lib/pkgconfig:\
            ${at-spi2-atk.dev}/lib/pkgconfig:\
            ${gtk3.dev}/lib/pkgconfig:\
            ${gdk-pixbuf.dev}/lib/pkgconfig:\
            ${cairo.dev}/lib/pkgconfig:\
            ${pango.dev}/lib/pkgconfig:\
            ${harfbuzz.dev}/lib/pkgconfig";
    XDG_DATA_DIRS = let
      base = pkgs.lib.concatMapStringsSep ":" (x: "${x}/share") [
        pkgs.adwaita-icon-theme
        pkgs.shared-mime-info
      ];
      gsettings_schema = pkgs.lib.concatMapStringsSep ":" (x: "${x}/share/gsettings-schemas/${x.name}") [
        pkgs.glib
        pkgs.gsettings-desktop-schemas
        pkgs.gtk3
      ];
    in "${base}:${gsettings_schema}";
  }
