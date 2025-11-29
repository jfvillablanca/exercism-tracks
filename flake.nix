{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    devenv.url = "github:cachix/devenv";
    easy-purescript-nix.url = "github:justinwoo/easy-purescript-nix";
  };

  outputs = {
    self,
    nixpkgs,
    devenv,
    flake-utils,
    easy-purescript-nix,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
      };
      easy-ps = easy-purescript-nix.packages.${system};
    in {
      devShells.default = devenv.lib.mkShell {
        inherit inputs pkgs;
        modules = [
          {
            languages = {
              nix.enable = true;
              go.enable = true;
              rust.enable = true;
              haskell = {
                enable = true;
              };
            };
            packages = with pkgs;
              [
                exercism

                # nix
                alejandra

                # go
                gofumpt

                # purescript
                nodejs_22
                esbuild
              ]
              ++ (with pkgs.haskellPackages; [
                hoogle
                hlint
              ])
              ++ (with easy-ps; [
                purs-0_14_7
                spago
                purescript-language-server
                purs-tidy
                purescript-psa
              ]);
          }
        ];
      };
    });
}
