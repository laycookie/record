{
  description = "Front-end for chat backends";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";  # Specify the Nixpkgs version
  };

  outputs = { self, nixpkgs }:
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in
  {
		devShells.${system} = {
			default = pkgs.mkShell.override { stdenv = pkgs.clangStdenv; } {
    		    packages = with pkgs; [
    		      cargo
    		      rustc
    		      rust-analyzer
    		      rustfmt

				  # python3
				  # ninja
				  # clang
				  # clang-tools


				  # vulkan-loader
				  # vulkan-validation-layers
				  # vulkan-tools

				  # libappindicator

				  openssl
				  pkg-config



				  # gtk3
				  # xdotool
				  # libayatana-appindicator
    		    ];
				buildInputs = with pkgs; [
					# libxkbcommon
      			  	# Other dependencies
      			];
				LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
					pkgs.libxkbcommon
					pkgs.wayland

					# pkgs.xorg.libX11
					# pkgs.xorg.libXcursor
					# pkgs.xorg.libXi

					pkgs.vulkan-loader
    			
					# pkgs.freetype
					# pkgs.fontconfig
					# pkgs.libinput
					# pkgs.qt5.full


					# pkgs.libayatana-appindicator
				];

    		    # RUST_BACKTRACE = "full";
				
				# Wayland
    		    # WINIT_UNIX_BACKEND = "wayland";
    		    
				# X11/Xwayland
				# WINIT_UNIX_BACKEND = "x11";
				# WAYLAND_DISPLAY="";
    		};
		};
	};
}
