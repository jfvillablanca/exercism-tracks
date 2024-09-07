{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    devenv.url = "github:cachix/devenv";
  };

  outputs = {
    self,
    nixpkgs,
    devenv,
    flake-utils,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
      };
    in {
      devShells.default = devenv.lib.mkShell {
        inherit inputs pkgs;
        modules = [
          {
            languages = {
              nix.enable = true;
              rust.enable = true;
              haskell = {
                enable = true;
                languageServer = pkgs.haskell-language-server.override {
                  supportedGhcVersions = ["927"];
                };
              };
            };
            packages = with pkgs; [exercism alejandra] ++ (with pkgs.haskellPackages; [hoogle]);
          }
        ];
      };
    });
}
