# Development Environment
# You can activate it through:
#   - New CLI: `nix develop --file shell.nix default`
#   - Old CLI: `nix-shell -A default`
{
  pkgs ? (import ./nix/nixpkgs.nix),
}:
{
  default = pkgs.mkShell {
    nativeBuildInputs = let
      tex-env = pkgs.texlive.combine {
        inherit (pkgs.texlive) scheme-full latex-bin latexmk;
      };
    in [
      # This project uses Nixtamal for input pinning
      pkgs.nixtamal

      # C / C++
      pkgs.gcc

      # Python
      pkgs.python3

      # Rust
      pkgs.rustc
      pkgs.rustfmt

      # Zig
      pkgs.zig

      # LaTeX Documents
      # tex-env
      # pkgs.python3Packages.pygments # For the minted package
    ];
  };
}
