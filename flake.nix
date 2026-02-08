{
  description = "Rust development environment";

  nixConfig.extra-substituters = [
    "https://nix-community.cachix.org"
    "https://anttiharju.cachix.org"
  ];
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-25.11";
    nur-anttiharju.url = "github:anttiharju/nur-packages";
    nur-anttiharju.inputs.nixpkgs.follows = "nixpkgs";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      nur-anttiharju,
      fenix,
      ...
    }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;

      devPackages =
        pkgs: anttiharju: system:
        with pkgs;
        let
          rustToolchain = fenix.packages.${system}.combine [
            (fenix.packages.${system}.stable.withComponents [
              "cargo"
              "clippy"
              "rustc"
              "rustfmt"
              "rust-src"
            ])
            fenix.packages.${system}.targets.aarch64-apple-darwin.stable.rust-std
            fenix.packages.${system}.targets.aarch64-unknown-linux-musl.stable.rust-std
            fenix.packages.${system}.targets.x86_64-unknown-linux-musl.stable.rust-std
          ];
        in
        [
          rustToolchain
          toml-cli
          nur-anttiharju.legacyPackages.${system}.zig."custom" # TODO: switch back to upstream Zig once 0.16 is available through stable nixpkgs (https://codeberg.org/ziglang/zig/pulls/30628)
          # action-validator # disabled because it uses glob instead of this library
          actionlint
          anttiharju.relcheck
          anttiharju.compare-changes
          editorconfig-checker
          (python313.withPackages (
            ps: with ps; [
              mkdocs-material
            ]
          ))
          prettier
          rubocop
          shellcheck
          gh
          yq-go
          ripgrep
          # Everything below is required by GitHub Actions
          uutils-coreutils-noprefix
          bash
          git
          findutils
          gnutar
          curl
          jq
          gzip
          envsubst
          gawkInteractive
          xz
          gnugrep
        ];
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
          anttiharju = nur-anttiharju.packages.${system};

          # Package the in-repo zig wrappers
          zcc = pkgs.runCommand "zcc" { } ''
            mkdir -p $out/bin
            cp -a ${./.cargo/zcc}/* $out/bin/
            chmod +x $out/bin/*
          '';
        in
        {
          default = pkgs.mkShell {
            packages = (devPackages pkgs anttiharju system) ++ [
              fenix.packages.${system}.stable.rust-analyzer
              zcc
            ];

            shellHook = ''
              export SDKROOT=/dev/null
              export CC="zig cc"
              export AR="zig ar"
              export RANLIB="zig ranlib"
              export CC_aarch64_apple_darwin="${zcc}/bin/aarch64-apple-darwin.sh"
              export CC_aarch64_unknown_linux_musl="${zcc}/bin/aarch64-unknown-linux-musl.sh"
              export CC_x86_64_unknown_linux_musl="${zcc}/bin/x86_64-unknown-linux-musl.sh"
              ${
                if system == "x86_64-linux" then
                  ''export CARGO_BUILD_TARGET="x86_64-unknown-linux-musl"''
                else if system == "aarch64-linux" then
                  ''export CARGO_BUILD_TARGET="aarch64-unknown-linux-musl"''
                else
                  ""
              }
              lefthook install
            '';
          };
        }
      );

      packages = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
          anttiharju = nur-anttiharju.packages.${system};

          # Fix not being able to run the unpatched node binaries that GitHub Actions mounts into the container
          ld = pkgs.runCommand "ld" { } ''
            mkdir -p $out/lib64
            install -D -m755 ${pkgs.nix-ld}/libexec/nix-ld "$out/lib64/$(basename ${pkgs.stdenv.cc.bintools.dynamicLinker})"
          '';

          # Package the in-repo zig wrappers so we can bake them into the image (relative path ./.cargo/zcc)
          zcc = pkgs.runCommand "zcc" { } ''
            mkdir -p $out/bin
            cp -a ${./.cargo/zcc}/* $out/bin/
            chmod +x $out/bin/*
          '';
        in
        pkgs.lib.optionalAttrs (system == "x86_64-linux" || system == "aarch64-linux") {
          ci = pkgs.dockerTools.streamLayeredImage {
            name = "ci";
            tag = "current";
            contents = (devPackages pkgs anttiharju system) ++ [
              ld
              zcc
              pkgs.dockerTools.caCertificates
              pkgs.sudo
              pkgs.nix.out
              pkgs.dockerTools.usrBinEnv
            ];
            config = {
              User = "1001"; # https://github.com/actions/runner/issues/2033#issuecomment-1598547465
              Labels = {
                "org.opencontainers.image.description" =
                  "This CI container image (apart from the flake.nix definition) is not covered by the license(s) of the source GitHub repository.";
                "org.opencontainers.image.licenses" = "NOASSERTION";
              };
              Env = [
                "SDKROOT=/dev/null"
                "CC=zig cc"
                "AR=zig ar"
                "RANLIB=zig ranlib"
                "CC_aarch64_apple_darwin=/usr/local/bin/aarch64-apple-darwin.sh"
                "CC_aarch64_unknown_linux_musl=/usr/local/bin/aarch64-unknown-linux-musl.sh"
                "CC_x86_64_unknown_linux_musl=/usr/local/bin/x86_64-unknown-linux-musl.sh"
                "CARGO_BUILD_TARGET=${
                  if system == "x86_64-linux" then "x86_64-unknown-linux-musl" else "aarch64-unknown-linux-musl"
                }"
                "NIX_LD_LIBRARY_PATH=${
                  pkgs.lib.makeLibraryPath [
                    pkgs.stdenv.cc.cc.lib
                    pkgs.glibc
                  ]
                }"
                "NIX_LD=${pkgs.stdenv.cc.bintools.dynamicLinker}"
                # PATH has to be defined so that actions that manipulate it (e.g. setup-go) don't break the environment
                "PATH=/home/runner/.local/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
              ];
            };
            enableFakechroot = true;
            fakeRootCommands = ''
              #!${pkgs.runtimeShell}

              # https://docs.github.com/en/actions/reference/runners/github-hosted-runners#administrative-privileges
              ${pkgs.dockerTools.shadowSetup}
              useradd -u 1001 -m runner
              echo "runner ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/runner
              chmod 0440 /etc/sudoers.d/runner
              mkdir -p /etc/pam.d
              {
                echo "auth       sufficient   pam_permit.so"
                echo "account    sufficient   pam_permit.so"
                echo "session    sufficient   pam_permit.so"
              } > /etc/pam.d/sudo
              chmod u+s /sbin/sudo

              # Fix 'parallel golangci-lint is running'
              mkdir -p /tmp
              chmod 1777 /tmp

              # Enable 'nix flake update' inside the container
              mkdir -p /etc/nix
              echo "experimental-features = nix-command flakes" > /etc/nix/nix.conf

              # Fix 'mv: No such file or directory (os error 2)'
              mkdir -p /usr/local/bin
              chmod 0777 /usr/local/bin

              # Install zig cc wrappers to /usr/local/bin
              mkdir -p /usr/local/bin
              install -D -m755 ${zcc}/bin/aarch64-apple-darwin.sh /usr/local/bin/aarch64-apple-darwin.sh
              install -D -m755 ${zcc}/bin/aarch64-unknown-linux-musl.sh /usr/local/bin/aarch64-unknown-linux-musl.sh
              install -D -m755 ${zcc}/bin/cc.sh /usr/local/bin/cc
              install -D -m755 ${zcc}/bin/x86_64-unknown-linux-musl.sh /usr/local/bin/x86_64-unknown-linux-musl.sh

              # Just avoid extra diffs when using a Dockerfile to inspect changes
              mkdir -p /proc /dev /sys
            '';
          };
        }
      );
    };
}
