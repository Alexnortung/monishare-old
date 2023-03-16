{ pkgs, ... }:

{
  packages = with pkgs; [
    edgedb
  ];

  languages = {
    rust = {
      enable = true;
    };
  };
}
