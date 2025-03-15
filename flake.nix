{
  description = "g600 configuration nix flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, treefmt-nix }:

    let
      system = "x86_64-linux";
      treefmtEval = treefmt-nix.lib.evalModule pkgs
        {
          # Used to find the project root
          projectRootFile = "flake.nix";

          programs = {
            nixpkgs-fmt.enable = true;
            rustfmt.enable = true;
          };
        };

      pkgs = import nixpkgs {
        inherit system;
      };

      g600 = import ./default.nix;
    in
    {
      packages.${system}.default = g600;

      devShells.${system} = {
        default = pkgs.mkShell {
          buildInputs = with pkgs; [
            treefmtEval.config.build.wrapper
            rust-analyzer
          ];
        };
      };

      formatter.${system} = treefmtEval.config.build.wrapper;
    };
}
