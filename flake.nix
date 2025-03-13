{
  description = "rust-template";

  inputs = {
    cargo2nix = {
      url = "github:DarkKirb/cargo2nix/metadata-workaround";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
      inputs.rust-overlay.follows = "rust-overlay";
    };

    flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/1.tar.gz";
    flake-utils.url = "github:numtide/flake-utils";

    nixpkgs.url = "github:NixOS/nixpkgs";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      cargo2nix,
      ...
    }@inputs:
    flake-utils.lib.eachSystem
      [
        "x86_64-linux"
        "aarch64-linux"
      ]
      (
        system:
        let
          overlays = [
            cargo2nix.overlays.default
            (import rust-overlay)
          ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
          qtMerged =
            with pkgs;
            qt6.callPackage (
              {
                env,
                qtbase,
              }:
              env "qt-${qtbase.version}" (
                with qt6;
                [
                  qtwayland
                  libglvnd
                  qtdeclarative
                ]
              )
            ) { };
          rustPkgsNoOverride = pkgs.rustBuilder.makePackageSet {
            packageFun = import ./Cargo.nix;
            rustChannel = "stable";
            rustVersion = "latest";
            packageOverrides = pkgs: pkgs.rustBuilder.overrides.all;
          };
          rustPkgs = pkgs.rustBuilder.makePackageSet {
            packageFun = import ./Cargo.nix;
            rustChannel = "stable";
            rustVersion = "latest";
            packageOverrides =
              pkgs:
              pkgs.rustBuilder.overrides.all
              ++ [
                (pkgs.rustBuilder.rustLib.makeOverride {
                  name = "ruma-macros";
                  overrideAttrs = drv: {
                    postPatch =
                      drv.postPatch or ""
                      + ''
                        substituteInPlace src/api.rs --replace 'manifest_parsed.features.client.is_none()' 'false'
                        substituteInPlace src/api.rs --replace 'manifest_parsed.features.server.is_none()' 'false'
                      '';
                  };
                })
                (pkgs.rustBuilder.rustLib.makeOverride {
                  name = "matrix-sdk-crypto";
                  overrideArgs = args: {
                    dependencies = args.dependencies // {
                      inherit (args.dependencies.ruma.dependencies) ruma_common;
                    };
                  };
                })
                (pkgs.rustBuilder.rustLib.makeOverride {
                  name = "matrix-sdk-base";
                  overrideArgs = args: {
                    dependencies = args.dependencies // {
                      inherit (args.dependencies.ruma.dependencies) ruma_events;
                    };
                  };
                })
                (pkgs.rustBuilder.rustLib.makeOverride {
                  name = "secret-service";
                  overrideArgs = args: {
                    dependencies = args.dependencies // {
                      inherit (args.dependencies.zbus.dependencies) zvariant;
                    };
                  };
                })
                (pkgs.rustBuilder.rustLib.makeOverride {
                  name = "matrix-sdk";
                  overrideArgs = args: {
                    dependencies = args.dependencies // {
                      inherit (args.dependencies.ruma.dependencies) ruma_events;
                    };
                  };
                  overrideAttrs = drv: {
                    postPatch = ''
                      substituteInPlace src/client/futures.rs --replace '#[cfg_vis(target_arch = "wasm32", pub(crate))]' ""
                      substituteInPlace src/encryption/futures.rs --replace '#[cfg_vis(target_arch = "wasm32", pub(crate))]' ""
                    '';
                  };
                })
                (pkgs.rustBuilder.rustLib.makeOverride {
                  name = "cxx";
                  overrideAttrs = drv: {
                    postInstall = ''
                      mkdir -p $out/include
                      cp -rv include $out/include/rust
                    '';
                  };
                })
                (pkgs.rustBuilder.rustLib.makeOverride {
                  name = "cxx-qt";
                  overrideAttrs = drv: {
                    dontWrapQtApps = true;
                    propagatedBuildInputs = drv.propagatedBuildInputs or [ ] ++ [
                      pkgs.qt6.qmake
                    ];
                    postInstall = ''
                      mkdir -p $out/include
                      cp -rv include $out/include/cxx-qt
                      link_src=$(readlink -f $out/build_script_output/cxx-qt-build/target/crates/cxx-qt/include/cxx-qt)
                      rm -f $out/build_script_output/cxx-qt-build/target/crates/cxx-qt/include/cxx-qt
                      cp -rv $link_src $out/build_script_output/cxx-qt-build/target/crates/cxx-qt/include/cxx-qt
                    '';
                  };
                })
                (pkgs.rustBuilder.rustLib.makeOverride {
                  name = "cxx-qt-lib";
                  overrideAttrs = drv: {
                    dontWrapQtApps = true;
                    buildInputs = drv.buildInputs or [ ] ++ [
                      qtMerged
                    ];
                    nativeBuildInputs = drv.nativeBuildInputs or [ ] ++ [
                      qtMerged
                    ];
                    propagatedBuildInputs = drv.propagatedBuildInputs or [ ] ++ [
                      qtMerged
                    ];
                    postInstall = ''
                      link_src=$(readlink -f $out/build_script_output/cxx-qt-build/target/crates/cxx-qt-lib/include/cxx-qt-lib)
                      rm -f $out/build_script_output/cxx-qt-build/target/crates/cxx-qt-lib/include/cxx-qt-lib
                      cp -rv $link_src $out/build_script_output/cxx-qt-build/target/crates/cxx-qt-lib/include/cxx-qt-lib
                      cp -rv src $out
                      sed -i -E "s|/build/cxx-qt-lib-[0-9\.]+|$out|" $out/build_script_output/cxx-qt-build/target/crates/cxx-qt-lib/manifest.json
                    '';
                  };
                })
                (pkgs.rustBuilder.rustLib.makeOverride {
                  name = "rachat-qt";
                  overrideAttrs = drv: {
                    preConfigure = ''
                      export PATH="${qtMerged}/bin:$PATH"
                      export QMAKE="${qtMerged}/bin/qmake"
                    '';

                    buildInputs = drv.buildInputs or [ ] ++ [
                      qtMerged
                    ];
                    nativeBuildInputs = drv.nativeBuildInputs or [ ] ++ [
                      qtMerged
                      pkgs.qt6.wrapQtAppsHook
                    ];
                    propagatedBuildInputs = drv.propagatedBuildInputs or [ ] ++ [
                      qtMerged
                    ];
                    fixupPhase =
                      drv.fixupPhase or ""
                      + ''
                        rm -rvvf $out
                        mkdir $out
                      '';
                  };
                })
              ];
          };
        in
        rec {
          devShells.default =
            with pkgs;
            mkShell {
              buildInputs = [
                (rust-bin.stable.latest.default.override {
                  extensions = [ "rust-src" ];
                })
                cargo2nix.packages.${system}.cargo2nix
                gdb
                sqlx-cli
                cargo-expand
                sqlite
                treefmt
                nixfmt-rfc-style
                qt6.full
                qtcreator
              ];
            };
          packages = (
            pkgs.lib.mapAttrs (_: v: (v { }).overrideAttrs { dontStrip = true; }) rustPkgs.workspace
          );
          nixosModules.default = import ./nixos {
            inherit inputs system;
          };
          checks = pkgs.lib.mapAttrs (_: v: pkgs.rustBuilder.runTests v { }) rustPkgs.workspace;
          hydraJobs = {
            inherit packages checks;
          };
          formatter = pkgs.nixfmt-rfc-style;
        }
      );
}
# Trick renovate into working: "github:NixOS/nixpkgs/nixpkgs-unstable"
