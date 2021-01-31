{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
    in rec {
      # `nix build`
      packages.dymoprint = naersk-lib.buildPackage {
        pname = "dymoprint";
        root = ./.;
        overrideMain = _: {
          IM_CONVERT = "${pkgs.imagemagick}/bin/convert";
        };
      };
      defaultPackage = packages.dymoprint;
    });
}
