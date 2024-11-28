# nix-shell csci-courses-dev.nix --command zsh
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
	rustup
	cargo-binutils
	cargo-watch
    tailwindcss
    sqlx-cli
	nodejs_22
	corepack_22
	# on nixos, you need to do some hacky shit to get docker to work in a nix-shell
	# probably just add it to your configuration.nix instead
	# docker_27
  ];
}

