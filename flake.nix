{
  description = "Rust development environment";

  nixConfig.extra-substituters = [
    "https://nix-community.cachix.org"
  ];
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-25.05";
    nixpkgs-unstable.url = "github:nixos/nixpkgs?ref=nixos-unstable";
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
      nixpkgs-unstable,
      nur-anttiharju,
      fenix,
      ...
    }:
    let
      container_version = "1.0.0";
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;

      devPackages =
        pkgs: pkgs-unstable: anttiharju: system:
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
            fenix.packages.${system}.targets.x86_64-unknown-linux-gnu.stable.rust-std
            fenix.packages.${system}.targets.aarch64-unknown-linux-gnu.stable.rust-std
          ];
        in
        [
          rustToolchain
          zig
          action-validator
          actionlint
          anttiharju.relcheck
          editorconfig-checker
          (python313.withPackages (
            ps: with ps; [
              mkdocs-material
            ]
          ))
          pkgs-unstable.prettier
          rubocop
          shellcheck
          gh
          yq-go
          toml-cli
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
      container_version = container_version; # This is here so that 'nix eval .#container_version --raw' works
      devShells = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
          pkgs-unstable = import nixpkgs-unstable { inherit system; };
          anttiharju = nur-anttiharju.packages.${system};
        in
        {
          default = pkgs.mkShell {
            packages = (devPackages pkgs pkgs-unstable anttiharju system) ++ [
              fenix.packages.${system}.stable.rust-analyzer
            ];

            shellHook = "lefthook install";
          };
        }
      );

      packages = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
          pkgs-unstable = import nixpkgs-unstable { inherit system; };
          anttiharju = nur-anttiharju.packages.${system};

          # Fix not being able to run the unpatched node binaries that GitHub Actions mounts into the container
          nix-ld-setup = pkgs.runCommand "nix-ld-setup" { } ''
            mkdir -p $out/lib64
            install -D -m755 ${pkgs.nix-ld}/libexec/nix-ld "$out/lib64/$(basename ${pkgs.stdenv.cc.bintools.dynamicLinker})"
          '';
        in
        pkgs.lib.optionalAttrs (system == "x86_64-linux" || system == "aarch64-linux") {
          ci = pkgs.dockerTools.streamLayeredImage {
            name = "ci";
            tag = container_version;
            contents = (devPackages pkgs pkgs-unstable anttiharju system) ++ [
              nix-ld-setup
              pkgs.dockerTools.caCertificates
              pkgs.sudo
              pkgs.nix.out
              pkgs.dockerTools.usrBinEnv
            ];
            config = {
              User = "1001"; # https://github.com/actions/runner/issues/2033#issuecomment-1598547465
              Env = [
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

              # Enable nix-command experimental feature
              mkdir -p /etc/nix
              echo "experimental-features = nix-command" > /etc/nix/nix.conf
            '';
          };
        }
      );
    };
}
