{
  description = "rust-template";

  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs;
    flake-utils.url = github:numtide/flake-utils;

    rust-overlay = {
      url = github:oxalica/rust-overlay;
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };

    cargo2nix = {
      url = github:cargo2nix/cargo2nix;
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
      inputs.rust-overlay.follows = "rust-overlay";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    cargo2nix,
    ...
  } @ inputs:
    flake-utils.lib.eachSystem ["x86_64-linux" "aarch64-linux"] (system: let
      overlays = [
        cargo2nix.overlays.default
        (import rust-overlay)
      ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
      qtMerged = with pkgs;
        qt6.callPackage ({
          env,
          qtbase,
        }:
          env "qt-${qtbase.version}" (with qt6; [
            qtwayland
            libglvnd
            qtdeclarative
          ])) {};
      rustPkgs = pkgs.rustBuilder.makePackageSet {
        packageFun = import ./Cargo.nix;
        rustChannel = "stable";
        rustVersion = "latest";
        packageOverrides = pkgs:
          pkgs.rustBuilder.overrides.all
          ++ [
            (pkgs.rustBuilder.rustLib.makeOverride {
              name = "cxx-qt";
              overrideAttrs = drv: {
                dontWrapQtApps = true;
                propagatedBuildInputs =
                  drv.propagatedBuildInputs
                  or []
                  ++ [
                    pkgs.qt6.qmake
                  ];
              };
            })
            (pkgs.rustBuilder.rustLib.makeOverride {
              name = "cxx-qt-lib";
              overrideAttrs = drv: {
                dontWrapQtApps = true;
                postConfigure = let
                  cxx_src = pkgs.rustBuilder.rustLib.fetchCratesIo {
                    name = "cxx";
                    version = "1.0.122";
                    sha256 = "bb497fad022245b29c2a0351df572e2d67c1046bcef2260ebc022aec81efea82";
                  };
                in ''
                  mkdir -pv include/rust
                  mkdir no
                  cd no
                  tar -xvvf ${cxx_src}
                  cd ..
                  cp -rv no/*/include/* include/rust
                  export CXXFLAGS="$CXXFLAGS -I$PWD/include"
                '';
                buildInputs =
                  drv.buildInputs
                  or []
                  ++ [
                    qtMerged
                  ];
                nativeBuildInputs =
                  drv.nativeBuildInputs
                  or []
                  ++ [
                    qtMerged
                  ];
                propagatedBuildInputs =
                  drv.propagatedBuildInputs
                  or []
                  ++ [
                    qtMerged
                  ];
              };
            })
            (pkgs.rustBuilder.rustLib.makeOverride {
              name = "rachat";
              overrideAttrs = drv: {
                preConfigure = ''
                  export PATH="${qtMerged}/bin:$PATH"
                  export QMAKE="${qtMerged}/bin/qmake"
                '';

                buildInputs =
                  drv.buildInputs
                  or []
                  ++ [
                    qtMerged
                  ];
                nativeBuildInputs =
                  drv.nativeBuildInputs
                  or []
                  ++ [
                    qtMerged
                  ];
                propagatedBuildInputs =
                  drv.propagatedBuildInputs
                  or []
                  ++ [
                    qtMerged
                    pkgs.qt6.wrapQtAppsHook
                  ];
                fixupPhase =
                  drv.fixupPhase
                  or ""
                  + ''
                    rm -rvvf $out
                    mkdir $out
                  '';
              };
            })
          ];
      };
    in rec {
      devShells.default = with pkgs;
        mkShell {
          buildInputs = [
            (rust-bin.nightly.latest.default.override {
              extensions = ["rust-src"];
            })
            cargo2nix.packages.${system}.cargo2nix
            alejandra
            qt6.full
            qtcreator
          ];
        };
      packages = {
        rachat = rustPkgs.workspace.rachat {};
      };
      nixosModules.default = import ./nixos {
        inherit inputs system;
      };
      hydraJobs = packages;
      formatter = pkgs.alejandra;
    });
}
