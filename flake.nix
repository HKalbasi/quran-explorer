# in flake.nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
          };
          nativeBuildInputs = with pkgs; [ dioxus-cli ];
          buildInputs = with pkgs; [ openssl ];
        in
        with pkgs;
        {
          devShells.default = mkShell {
            # 👇 and now we can just inherit them
            inherit buildInputs nativeBuildInputs;
          };
          devShells.android = mkShell
            {
              # 👇 and now we can just inherit them
              inherit buildInputs;
              nativeBuildInputs = nativeBuildInputs ++ [ pkgs.cowsay ];
            };
        }
      );
}
