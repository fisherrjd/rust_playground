{ pkgs ? import
    (fetchTarball {
      name = "jpetrucciani-2024-07-25";
      url = "https://github.com/jpetrucciani/nix/archive/366a7ddbd76187cbf1bb9839228e0cb442b0cee6.tar.gz";
      sha256 = "1jpfhmpyzr554172jp94fxa9wr5r83f7z0cg0l1gvaa30104ai3v";
    })
    { }
}:
let
  name = "rust_playground";


  tools = with pkgs; {
    cli = [
      coreutils
      nixpkgs-fmt
    ];
    rust = [
      cargo
      clang
      rust-analyzer
      rustc
      rustfmt
    ];
    scripts = pkgs.lib.attrsets.attrValues scripts;
  };

  scripts = with pkgs; { };
  paths = pkgs.lib.flatten [ (builtins.attrValues tools) ];
  env = pkgs.buildEnv {
    inherit name paths; buildInputs = paths;
  };
in
(env.overrideAttrs (_: {
  inherit name;
  NIXUP = "0.0.6";
})) // { inherit scripts; }
