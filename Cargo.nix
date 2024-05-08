# This file was @generated by cargo2nix 0.11.0.
# It is not intended to be manually edited.
args @ {
  release ? true,
  rootFeatures ? [
    "rachat/default"
  ],
  rustPackages,
  buildRustPackages,
  hostPlatform,
  hostPlatformCpu ? null,
  hostPlatformFeatures ? [],
  target ? null,
  codegenOpts ? null,
  profileOpts ? null,
  cargoUnstableFlags ? null,
  rustcLinkFlags ? null,
  rustcBuildFlags ? null,
  mkRustCrate,
  rustLib,
  lib,
  workspaceSrc,
  ignoreLockHash,
}: let
  nixifiedLockHash = "e77f104ffe78d11bb83b617c18587a0b1e381157d2a0f6edad16104b047a2648";
  workspaceSrc =
    if args.workspaceSrc == null
    then ./.
    else args.workspaceSrc;
  currentLockHash = builtins.hashFile "sha256" (workspaceSrc + /Cargo.lock);
  lockHashIgnored =
    if ignoreLockHash
    then builtins.trace "Ignoring lock hash" ignoreLockHash
    else ignoreLockHash;
in
  if !lockHashIgnored && (nixifiedLockHash != currentLockHash)
  then throw "Cargo.nix ${nixifiedLockHash} is out of sync with Cargo.lock ${currentLockHash}"
  else let
    inherit (rustLib) fetchCratesIo fetchCrateLocal fetchCrateGit fetchCrateAlternativeRegistry expandFeatures decideProfile genDrvsByProfile;
    profilesByName = {
    };
    rootFeatures' = expandFeatures rootFeatures;
    overridableMkRustCrate = f: let
      drvs = genDrvsByProfile profilesByName ({
        profile,
        profileName,
      }:
        mkRustCrate ({inherit release profile hostPlatformCpu hostPlatformFeatures target profileOpts codegenOpts cargoUnstableFlags rustcLinkFlags rustcBuildFlags;} // (f profileName)));
    in
      {
        compileMode ? null,
        profileName ? decideProfile compileMode release,
      }: let
        drv = drvs.${profileName};
      in
        if compileMode == null
        then drv
        else drv.override {inherit compileMode;};
  in {
    cargo2nixVersion = "0.11.0";
    workspace = {
      rachat = rustPackages.unknown.rachat."0.1.0";
    };
    "registry+https://github.com/rust-lang/crates.io-index".cc."1.0.97" = overridableMkRustCrate (profileName: rec {
      name = "cc";
      version = "1.0.97";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "099a5357d84c4c61eb35fc8eafa9a79a902c2f76911e5747ced4e032edd8d9b4";
      };
      features = builtins.concatLists [
        ["jobserver"]
        ["libc"]
        ["once_cell"]
        ["parallel"]
      ];
      dependencies = {
        jobserver = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".jobserver."0.1.31" {inherit profileName;}).out;
        ${
          if hostPlatform.isUnix
          then "libc"
          else null
        } =
          (rustPackages."registry+https://github.com/rust-lang/crates.io-index".libc."0.2.154" {inherit profileName;}).out;
        once_cell = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".once_cell."1.19.0" {inherit profileName;}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".clang-format."0.3.0" = overridableMkRustCrate (profileName: rec {
      name = "clang-format";
      version = "0.3.0";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "696283b40e1a39d208ee614b92e5f6521d16962edeb47c48372585ec92419943";
      };
      dependencies = {
        thiserror = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".thiserror."1.0.60" {inherit profileName;}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".codespan-reporting."0.11.1" = overridableMkRustCrate (profileName: rec {
      name = "codespan-reporting";
      version = "0.11.1";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "3538270d33cc669650c4b093848450d380def10c331d38c768e34cac80576e6e";
      };
      dependencies = {
        termcolor = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".termcolor."1.4.1" {inherit profileName;}).out;
        unicode_width = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".unicode-width."0.1.12" {inherit profileName;}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".convert_case."0.6.0" = overridableMkRustCrate (profileName: rec {
      name = "convert_case";
      version = "0.6.0";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "ec182b0ca2f35d8fc196cf3404988fd8b8c739a4d270ff118a398feb0cbec1ca";
      };
      dependencies = {
        unicode_segmentation = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".unicode-segmentation."1.11.0" {inherit profileName;}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".cxx."1.0.122" = overridableMkRustCrate (profileName: rec {
      name = "cxx";
      version = "1.0.122";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "bb497fad022245b29c2a0351df572e2d67c1046bcef2260ebc022aec81efea82";
      };
      features = builtins.concatLists [
        ["alloc"]
        ["default"]
        ["std"]
      ];
      dependencies = {
        cxxbridge_macro = (buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".cxxbridge-macro."1.0.122" {profileName = "__noProfile";}).out;
        link_cplusplus = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".link-cplusplus."1.0.9" {inherit profileName;}).out;
      };
      buildDependencies = {
        cc = (buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".cc."1.0.97" {profileName = "__noProfile";}).out;
        cxxbridge_flags = (buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".cxxbridge-flags."1.0.122" {profileName = "__noProfile";}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".cxx-build."1.0.122" = overridableMkRustCrate (profileName: rec {
      name = "cxx-build";
      version = "1.0.122";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "9327c7f9fbd6329a200a5d4aa6f674c60ab256525ff0084b52a889d4e4c60cee";
      };
      features = builtins.concatLists [
        ["parallel"]
      ];
      dependencies = {
        cc = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".cc."1.0.97" {inherit profileName;}).out;
        codespan_reporting = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".codespan-reporting."0.11.1" {inherit profileName;}).out;
        once_cell = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".once_cell."1.19.0" {inherit profileName;}).out;
        proc_macro2 = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.82" {inherit profileName;}).out;
        quote = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".quote."1.0.36" {inherit profileName;}).out;
        scratch = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".scratch."1.0.7" {inherit profileName;}).out;
        syn = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".syn."2.0.61" {inherit profileName;}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".cxx-gen."0.7.122" = overridableMkRustCrate (profileName: rec {
      name = "cxx-gen";
      version = "0.7.122";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "a476ac5d29b1957ad93669eef9a030e9fc139103f9bb1ff9f15504c3f21236b0";
      };
      dependencies = {
        codespan_reporting = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".codespan-reporting."0.11.1" {inherit profileName;}).out;
        proc_macro2 = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.82" {inherit profileName;}).out;
        quote = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".quote."1.0.36" {inherit profileName;}).out;
        syn = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".syn."2.0.61" {inherit profileName;}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".cxx-qt."0.6.1" = overridableMkRustCrate (profileName: rec {
      name = "cxx-qt";
      version = "0.6.1";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "08aa6cda7588b6d17c563b0d2fadc060d4204d04908c0f359ae288857091218d";
      };
      dependencies = {
        cxx = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".cxx."1.0.122" {inherit profileName;}).out;
        cxx_qt_macro = (buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".cxx-qt-macro."0.6.1" {profileName = "__noProfile";}).out;
        static_assertions = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".static_assertions."1.1.0" {inherit profileName;}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".cxx-qt-build."0.6.1" = overridableMkRustCrate (profileName: rec {
      name = "cxx-qt-build";
      version = "0.6.1";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "9e097b99f49792922a72a8ca35d9391762e48e63363d6998255be1f2ca1edf69";
      };
      features = builtins.concatLists [
        ["default"]
        ["link_qt_object_files"]
        ["qt_gui"]
        ["qt_qml"]
      ];
      dependencies = {
        cc = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".cc."1.0.97" {inherit profileName;}).out;
        codespan_reporting = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".codespan-reporting."0.11.1" {inherit profileName;}).out;
        convert_case = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".convert_case."0.6.0" {inherit profileName;}).out;
        cxx_gen = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".cxx-gen."0.7.122" {inherit profileName;}).out;
        cxx_qt_gen = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".cxx-qt-gen."0.6.1" {inherit profileName;}).out;
        cxx_qt_lib_headers = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".cxx-qt-lib-headers."0.6.1" {inherit profileName;}).out;
        proc_macro2 = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.82" {inherit profileName;}).out;
        qt_build_utils = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".qt-build-utils."0.6.1" {inherit profileName;}).out;
        quote = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".quote."1.0.36" {inherit profileName;}).out;
        version_check = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".version_check."0.9.4" {inherit profileName;}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".cxx-qt-gen."0.6.1" = overridableMkRustCrate (profileName: rec {
      name = "cxx-qt-gen";
      version = "0.6.1";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "ede7c73dbfbcc234d8826919e257830c1789db2cac586546a87d2a82e3cbe5d5";
      };
      dependencies = {
        clang_format = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".clang-format."0.3.0" {inherit profileName;}).out;
        convert_case = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".convert_case."0.6.0" {inherit profileName;}).out;
        indoc = (buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".indoc."2.0.5" {profileName = "__noProfile";}).out;
        proc_macro2 = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.82" {inherit profileName;}).out;
        quote = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".quote."1.0.36" {inherit profileName;}).out;
        syn = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".syn."2.0.61" {inherit profileName;}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".cxx-qt-lib."0.6.1" = overridableMkRustCrate (profileName: rec {
      name = "cxx-qt-lib";
      version = "0.6.1";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "002f1a6119bcb7dfec67eb7c0803a7b1d595dc54610559faeac35133f22a5880";
      };
      features = builtins.concatLists [
        ["default"]
        ["qt_gui"]
        ["qt_qml"]
      ];
      dependencies = {
        cxx = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".cxx."1.0.122" {inherit profileName;}).out;
      };
      buildDependencies = {
        cxx_build = (buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".cxx-build."1.0.122" {profileName = "__noProfile";}).out;
        cxx_qt_lib_headers = (buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".cxx-qt-lib-headers."0.6.1" {profileName = "__noProfile";}).out;
        qt_build_utils = (buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".qt-build-utils."0.6.1" {profileName = "__noProfile";}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".cxx-qt-lib-headers."0.6.1" = overridableMkRustCrate (profileName: rec {
      name = "cxx-qt-lib-headers";
      version = "0.6.1";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "9abdeab6b77cfc5a53b724f3f62a37bcb5ac1423cccc2dba4c134f4273440b8c";
      };
      features = builtins.concatLists [
        ["default"]
        ["qt_gui"]
        ["qt_qml"]
      ];
    });

    "registry+https://github.com/rust-lang/crates.io-index".cxx-qt-macro."0.6.1" = overridableMkRustCrate (profileName: rec {
      name = "cxx-qt-macro";
      version = "0.6.1";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "699e8a668c03b03419b084960d72eed253632bb16349b33fd0a0c893b61b664c";
      };
      dependencies = {
        cxx_qt_gen = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".cxx-qt-gen."0.6.1" {inherit profileName;}).out;
        proc_macro2 = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.82" {inherit profileName;}).out;
        syn = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".syn."2.0.61" {inherit profileName;}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".cxxbridge-flags."1.0.122" = overridableMkRustCrate (profileName: rec {
      name = "cxxbridge-flags";
      version = "1.0.122";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "688c799a4a846f1c0acb9f36bb9c6272d9b3d9457f3633c7753c6057270df13c";
      };
      features = builtins.concatLists [
        ["default"]
      ];
    });

    "registry+https://github.com/rust-lang/crates.io-index".cxxbridge-macro."1.0.122" = overridableMkRustCrate (profileName: rec {
      name = "cxxbridge-macro";
      version = "1.0.122";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "928bc249a7e3cd554fd2e8e08a426e9670c50bbfc9a621653cfa9accc9641783";
      };
      dependencies = {
        proc_macro2 = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.82" {inherit profileName;}).out;
        quote = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".quote."1.0.36" {inherit profileName;}).out;
        syn = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".syn."2.0.61" {inherit profileName;}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".either."1.11.0" = overridableMkRustCrate (profileName: rec {
      name = "either";
      version = "1.11.0";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "a47c1c47d2f5964e29c61246e81db715514cd532db6b5116a25ea3c03d6780a2";
      };
      features = builtins.concatLists [
        ["use_std"]
      ];
    });

    "registry+https://github.com/rust-lang/crates.io-index".indoc."2.0.5" = overridableMkRustCrate (profileName: rec {
      name = "indoc";
      version = "2.0.5";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "b248f5224d1d606005e02c97f5aa4e88eeb230488bcc03bc9ca4d7991399f2b5";
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".itertools."0.11.0" = overridableMkRustCrate (profileName: rec {
      name = "itertools";
      version = "0.11.0";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "b1c173a5686ce8bfa551b3563d0c2170bf24ca44da99c7ca4bfdab5418c3fe57";
      };
      features = builtins.concatLists [
        ["default"]
        ["use_alloc"]
        ["use_std"]
      ];
      dependencies = {
        either = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".either."1.11.0" {inherit profileName;}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".jobserver."0.1.31" = overridableMkRustCrate (profileName: rec {
      name = "jobserver";
      version = "0.1.31";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "d2b099aaa34a9751c5bf0878add70444e1ed2dd73f347be99003d4577277de6e";
      };
      dependencies = {
        ${
          if hostPlatform.isUnix
          then "libc"
          else null
        } =
          (rustPackages."registry+https://github.com/rust-lang/crates.io-index".libc."0.2.154" {inherit profileName;}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".libc."0.2.154" = overridableMkRustCrate (profileName: rec {
      name = "libc";
      version = "0.2.154";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "ae743338b92ff9146ce83992f766a31066a91a8c84a45e0e9f21e7cf6de6d346";
      };
      features = builtins.concatLists [
        ["default"]
        ["std"]
      ];
    });

    "registry+https://github.com/rust-lang/crates.io-index".link-cplusplus."1.0.9" = overridableMkRustCrate (profileName: rec {
      name = "link-cplusplus";
      version = "1.0.9";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "9d240c6f7e1ba3a28b0249f774e6a9dd0175054b52dfbb61b16eb8505c3785c9";
      };
      features = builtins.concatLists [
        ["default"]
      ];
      buildDependencies = {
        cc = (buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".cc."1.0.97" {profileName = "__noProfile";}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".memchr."2.7.2" = overridableMkRustCrate (profileName: rec {
      name = "memchr";
      version = "2.7.2";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "6c8640c5d730cb13ebd907d8d04b52f55ac9a2eec55b440c8892f40d56c76c1d";
      };
      features = builtins.concatLists [
        ["alloc"]
        ["std"]
      ];
    });

    "registry+https://github.com/rust-lang/crates.io-index".minimal-lexical."0.2.1" = overridableMkRustCrate (profileName: rec {
      name = "minimal-lexical";
      version = "0.2.1";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "68354c5c6bd36d73ff3feceb05efa59b6acb7626617f4962be322a825e61f79a";
      };
      features = builtins.concatLists [
        ["std"]
      ];
    });

    "registry+https://github.com/rust-lang/crates.io-index".nom."7.1.3" = overridableMkRustCrate (profileName: rec {
      name = "nom";
      version = "7.1.3";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "d273983c5a657a70a3e8f2a01329822f3b8c8172b73826411a55751e404a0a4a";
      };
      features = builtins.concatLists [
        ["alloc"]
        ["default"]
        ["std"]
      ];
      dependencies = {
        memchr = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".memchr."2.7.2" {inherit profileName;}).out;
        minimal_lexical = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".minimal-lexical."0.2.1" {inherit profileName;}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".once_cell."1.19.0" = overridableMkRustCrate (profileName: rec {
      name = "once_cell";
      version = "1.19.0";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "3fdb12b2476b595f9358c5161aa467c2438859caa136dec86c26fdd2efe17b92";
      };
      features = builtins.concatLists [
        ["alloc"]
        ["default"]
        ["race"]
        ["std"]
      ];
    });

    "registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.82" = overridableMkRustCrate (profileName: rec {
      name = "proc-macro2";
      version = "1.0.82";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "8ad3d49ab951a01fbaafe34f2ec74122942fe18a3f9814c3268f1bb72042131b";
      };
      features = builtins.concatLists [
        ["default"]
        ["proc-macro"]
        ["span-locations"]
      ];
      dependencies = {
        unicode_ident = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".unicode-ident."1.0.12" {inherit profileName;}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".qt-build-utils."0.6.1" = overridableMkRustCrate (profileName: rec {
      name = "qt-build-utils";
      version = "0.6.1";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "d59c828fe2434dad34dd0c30a4ba037509b61dad92a55baf0dc42699e6aa2f10";
      };
      features = builtins.concatLists [
        ["link_qt_object_files"]
      ];
      dependencies = {
        cc = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".cc."1.0.97" {inherit profileName;}).out;
        thiserror = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".thiserror."1.0.60" {inherit profileName;}).out;
        versions = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".versions."5.0.1" {inherit profileName;}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".quote."1.0.36" = overridableMkRustCrate (profileName: rec {
      name = "quote";
      version = "1.0.36";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "0fa76aaf39101c457836aec0ce2316dbdc3ab723cdda1c6bd4e6ad4208acaca7";
      };
      features = builtins.concatLists [
        ["default"]
        ["proc-macro"]
      ];
      dependencies = {
        proc_macro2 = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.82" {inherit profileName;}).out;
      };
    });

    "unknown".rachat."0.1.0" = overridableMkRustCrate (profileName: rec {
      name = "rachat";
      version = "0.1.0";
      registry = "unknown";
      src = fetchCrateLocal workspaceSrc;
      dependencies = {
        cxx = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".cxx."1.0.122" {inherit profileName;}).out;
        cxx_qt = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".cxx-qt."0.6.1" {inherit profileName;}).out;
        cxx_qt_lib = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".cxx-qt-lib."0.6.1" {inherit profileName;}).out;
      };
      buildDependencies = {
        cxx_qt_build = (buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".cxx-qt-build."0.6.1" {profileName = "__noProfile";}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".scratch."1.0.7" = overridableMkRustCrate (profileName: rec {
      name = "scratch";
      version = "1.0.7";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "a3cf7c11c38cb994f3d40e8a8cde3bbd1f72a435e4c49e85d6553d8312306152";
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".static_assertions."1.1.0" = overridableMkRustCrate (profileName: rec {
      name = "static_assertions";
      version = "1.1.0";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "a2eb9349b6444b326872e140eb1cf5e7c522154d69e7a0ffb0fb81c06b37543f";
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".syn."2.0.61" = overridableMkRustCrate (profileName: rec {
      name = "syn";
      version = "2.0.61";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "c993ed8ccba56ae856363b1845da7266a7cb78e1d146c8a32d54b45a8b831fc9";
      };
      features = builtins.concatLists [
        ["clone-impls"]
        ["default"]
        ["derive"]
        ["extra-traits"]
        ["full"]
        ["parsing"]
        ["printing"]
        ["proc-macro"]
      ];
      dependencies = {
        proc_macro2 = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.82" {inherit profileName;}).out;
        quote = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".quote."1.0.36" {inherit profileName;}).out;
        unicode_ident = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".unicode-ident."1.0.12" {inherit profileName;}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".termcolor."1.4.1" = overridableMkRustCrate (profileName: rec {
      name = "termcolor";
      version = "1.4.1";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "06794f8f6c5c898b3275aebefa6b8a1cb24cd2c6c79397ab15774837a0bc5755";
      };
      dependencies = {
        ${
          if hostPlatform.isWindows
          then "winapi_util"
          else null
        } =
          (rustPackages."registry+https://github.com/rust-lang/crates.io-index".winapi-util."0.1.8" {inherit profileName;}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".thiserror."1.0.60" = overridableMkRustCrate (profileName: rec {
      name = "thiserror";
      version = "1.0.60";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "579e9083ca58dd9dcf91a9923bb9054071b9ebbd800b342194c9feb0ee89fc18";
      };
      dependencies = {
        thiserror_impl = (buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".thiserror-impl."1.0.60" {profileName = "__noProfile";}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".thiserror-impl."1.0.60" = overridableMkRustCrate (profileName: rec {
      name = "thiserror-impl";
      version = "1.0.60";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "e2470041c06ec3ac1ab38d0356a6119054dedaea53e12fbefc0de730a1c08524";
      };
      dependencies = {
        proc_macro2 = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.82" {inherit profileName;}).out;
        quote = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".quote."1.0.36" {inherit profileName;}).out;
        syn = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".syn."2.0.61" {inherit profileName;}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".unicode-ident."1.0.12" = overridableMkRustCrate (profileName: rec {
      name = "unicode-ident";
      version = "1.0.12";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "3354b9ac3fae1ff6755cb6db53683adb661634f67557942dea4facebec0fee4b";
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".unicode-segmentation."1.11.0" = overridableMkRustCrate (profileName: rec {
      name = "unicode-segmentation";
      version = "1.11.0";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "d4c87d22b6e3f4a18d4d40ef354e97c90fcb14dd91d7dc0aa9d8a1172ebf7202";
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".unicode-width."0.1.12" = overridableMkRustCrate (profileName: rec {
      name = "unicode-width";
      version = "0.1.12";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "68f5e5f3158ecfd4b8ff6fe086db7c8467a2dfdac97fe420f2b7c4aa97af66d6";
      };
      features = builtins.concatLists [
        ["default"]
      ];
    });

    "registry+https://github.com/rust-lang/crates.io-index".version_check."0.9.4" = overridableMkRustCrate (profileName: rec {
      name = "version_check";
      version = "0.9.4";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "49874b5167b65d7193b8aba1567f5c7d93d001cafc34600cee003eda787e483f";
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".versions."5.0.1" = overridableMkRustCrate (profileName: rec {
      name = "versions";
      version = "5.0.1";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "c73a36bc44e3039f51fbee93e39f41225f6b17b380eb70cc2aab942df06b34dd";
      };
      dependencies = {
        itertools = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".itertools."0.11.0" {inherit profileName;}).out;
        nom = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".nom."7.1.3" {inherit profileName;}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".winapi-util."0.1.8" = overridableMkRustCrate (profileName: rec {
      name = "winapi-util";
      version = "0.1.8";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "4d4cc384e1e73b93bafa6fb4f1df8c41695c8a91cf9c4c64358067d15a7b6c6b";
      };
      dependencies = {
        ${
          if hostPlatform.isWindows
          then "windows_sys"
          else null
        } =
          (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows-sys."0.52.0" {inherit profileName;}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".windows-sys."0.52.0" = overridableMkRustCrate (profileName: rec {
      name = "windows-sys";
      version = "0.52.0";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "282be5f36a8ce781fad8c8ae18fa3f9beff57ec1b52cb3de0789201425d9a33d";
      };
      features = builtins.concatLists [
        ["Win32"]
        ["Win32_Foundation"]
        ["Win32_Storage"]
        ["Win32_Storage_FileSystem"]
        ["Win32_System"]
        ["Win32_System_Console"]
        ["Win32_System_SystemInformation"]
        ["default"]
      ];
      dependencies = {
        windows_targets = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows-targets."0.52.5" {inherit profileName;}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".windows-targets."0.52.5" = overridableMkRustCrate (profileName: rec {
      name = "windows-targets";
      version = "0.52.5";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "6f0713a46559409d202e70e28227288446bf7841d3211583a4b53e3f6d96e7eb";
      };
      dependencies = {
        ${
          if hostPlatform.config == "aarch64-pc-windows-gnullvm"
          then "windows_aarch64_gnullvm"
          else null
        } =
          (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_aarch64_gnullvm."0.52.5" {inherit profileName;}).out;
        ${
          if hostPlatform.parsed.cpu.name == "aarch64" && hostPlatform.parsed.abi.name == "msvc"
          then "windows_aarch64_msvc"
          else null
        } =
          (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_aarch64_msvc."0.52.5" {inherit profileName;}).out;
        ${
          if hostPlatform.parsed.cpu.name == "i686" && hostPlatform.parsed.abi.name == "gnu"
          then "windows_i686_gnu"
          else null
        } =
          (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_i686_gnu."0.52.5" {inherit profileName;}).out;
        ${
          if hostPlatform.config == "i686-pc-windows-gnullvm"
          then "windows_i686_gnullvm"
          else null
        } =
          (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_i686_gnullvm."0.52.5" {inherit profileName;}).out;
        ${
          if hostPlatform.parsed.cpu.name == "i686" && hostPlatform.parsed.abi.name == "msvc"
          then "windows_i686_msvc"
          else null
        } =
          (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_i686_msvc."0.52.5" {inherit profileName;}).out;
        ${
          if hostPlatform.parsed.cpu.name == "x86_64" && hostPlatform.parsed.abi.name == "gnu"
          then "windows_x86_64_gnu"
          else null
        } =
          (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_x86_64_gnu."0.52.5" {inherit profileName;}).out;
        ${
          if hostPlatform.config == "x86_64-pc-windows-gnullvm"
          then "windows_x86_64_gnullvm"
          else null
        } =
          (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_x86_64_gnullvm."0.52.5" {inherit profileName;}).out;
        ${
          if (hostPlatform.parsed.cpu.name == "x86_64" || hostPlatform.parsed.cpu.name == "arm64ec") && hostPlatform.parsed.abi.name == "msvc"
          then "windows_x86_64_msvc"
          else null
        } =
          (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_x86_64_msvc."0.52.5" {inherit profileName;}).out;
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".windows_aarch64_gnullvm."0.52.5" = overridableMkRustCrate (profileName: rec {
      name = "windows_aarch64_gnullvm";
      version = "0.52.5";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "7088eed71e8b8dda258ecc8bac5fb1153c5cffaf2578fc8ff5d61e23578d3263";
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".windows_aarch64_msvc."0.52.5" = overridableMkRustCrate (profileName: rec {
      name = "windows_aarch64_msvc";
      version = "0.52.5";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "9985fd1504e250c615ca5f281c3f7a6da76213ebd5ccc9561496568a2752afb6";
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".windows_i686_gnu."0.52.5" = overridableMkRustCrate (profileName: rec {
      name = "windows_i686_gnu";
      version = "0.52.5";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "88ba073cf16d5372720ec942a8ccbf61626074c6d4dd2e745299726ce8b89670";
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".windows_i686_gnullvm."0.52.5" = overridableMkRustCrate (profileName: rec {
      name = "windows_i686_gnullvm";
      version = "0.52.5";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "87f4261229030a858f36b459e748ae97545d6f1ec60e5e0d6a3d32e0dc232ee9";
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".windows_i686_msvc."0.52.5" = overridableMkRustCrate (profileName: rec {
      name = "windows_i686_msvc";
      version = "0.52.5";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "db3c2bf3d13d5b658be73463284eaf12830ac9a26a90c717b7f771dfe97487bf";
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".windows_x86_64_gnu."0.52.5" = overridableMkRustCrate (profileName: rec {
      name = "windows_x86_64_gnu";
      version = "0.52.5";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "4e4246f76bdeff09eb48875a0fd3e2af6aada79d409d33011886d3e1581517d9";
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".windows_x86_64_gnullvm."0.52.5" = overridableMkRustCrate (profileName: rec {
      name = "windows_x86_64_gnullvm";
      version = "0.52.5";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "852298e482cd67c356ddd9570386e2862b5673c85bd5f88df9ab6802b334c596";
      };
    });

    "registry+https://github.com/rust-lang/crates.io-index".windows_x86_64_msvc."0.52.5" = overridableMkRustCrate (profileName: rec {
      name = "windows_x86_64_msvc";
      version = "0.52.5";
      registry = "registry+https://github.com/rust-lang/crates.io-index";
      src = fetchCratesIo {
        inherit name version;
        sha256 = "bec47e5bfd1bff0eeaf6d8b485cc1074891a197ab4225d504cb7a1ab88b02bf0";
      };
    });
  }
