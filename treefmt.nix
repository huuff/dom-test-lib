{ leptosfmt }:
_: {
  projectRootFile = "flake.nix";

  programs = {
    nixfmt.enable = true;
    rustfmt.enable = true;
    taplo.enable = true;
  };

  settings.formatter = {
    leptosfmt = {
      command = "${leptosfmt}/bin/leptosfmt";
      includes = [ "*.rs" ];
    };
  };
}
