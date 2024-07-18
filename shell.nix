let
	pkgs = import <nixpkgs> {};
in
pkgs.mkShell {
	packages = with pkgs; [
		cargo
		rustc
		rust-analyzer
		rustfmt
		clippy
		
		openssl

		pkg-config
		libxkbcommon
		wayland

		binutils
		freetype
		fontconfig

		gio-sharp
		glib
		gtk4
	];

	LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
		pkgs.libxkbcommon
		pkgs.wayland
		pkgs.vulkan-loader

		pkgs.binutils
		pkgs.freetype
		pkgs.fontconfig

		pkgs.gio-sharp
		pkgs.glib
		pkgs.gtk4
	];

	env = {
		RUST_BACKTRACE = "full";
		WINIT_UNIX_BACKEND="wayland";
	}; 
}
