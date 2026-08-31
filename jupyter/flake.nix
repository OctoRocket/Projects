{
  description = "Evironment for Jupyter notebooks with Haskell";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
  let
    pkgs = import nixpkgs { system = "x86_64-linux"; };
  in {
    devShells."x86_64-linux".default = pkgs.mkShell {
      packages = with pkgs; [
        python314Packages.jupyter
        python314Packages.jupyterlab

        haskellPackages.ghc
        haskellPackages.cabal-install
        haskellPackages.haskell-language-server
        haskellPackages.ihaskell

        pkg-config
      ];
      # AI code of which I'm unsure the purpose of, I don't trust its necessary.
      # shellHook = ''
      #   export PROJECT_ROOT="$PWD"

      #   # Put project-local Jupyter kernels ahead of global kernels.
      #   export JUPYTER_PATH="$PWD/.jupyter:''${JUPYTER_PATH:-}"

      #   mkdir -p "$PWD/.jupyter/kernels"

      #   # Install the kernel once for the current user.
      #   if [ ! -f "$HOME/.local/share/jupyter/kernels/haskell/kernel.json" ]; then
      #     echo "Installing the IHaskell Jupyter kernel..."
      #     ihaskell install \
      #       --ghclib="$(ghc --print-libdir)" \
      #       --prefix="$HOME/.local"
      #   fi

      #   echo
      #   echo "Haskell notebook environment ready"
      #   echo "GHC:     $(ghc --numeric-version)"
      #   echo "IHaskell: $(command -v ihaskell)"
      #   echo
      # '';
    };
  };
}