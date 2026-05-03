{
  description = "Rust development environment for compare-changes";

  nixConfig.extra-substituters = [
    "https://nix-community.cachix.org"
    "https://anttiharju.cachix.org"
  ];
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixpkgs-unstable";
    nur-anttiharju = {
      url = "github:anttiharju/nur-packages";
      inputs.nixpkgs.follows = "nixpkgs";
    };
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
          (pkgs.runCommand "zcc" { } ''
            mkdir -p $out/bin
            cp -a ${./.cargo/zcc}/* $out/bin/
            chmod +x $out/bin/*
          '')
          rustToolchain
          toml-cli
          zig
          action-validator
          actionlint
          anttiharju.relcheck
          anttiharju.compare-changes
          editorconfig-checker
          zensical
          prettier
          rubocop
          shellcheck
          gh
          zizmor
          gitMinimal
          curl
          jq
          envsubst
        ];

      # Shared environment variables for both devShell and CI
      zigEnv =
        system:
        {
          SDKROOT = "/dev/null";
          RANLIB = "zig ranlib";
          AR = "zig ar";
          CC = "zig cc";
          CC_aarch64_apple_darwin = "aarch64-apple-darwin.sh";
          CC_x86_64_unknown_linux_musl = "x86_64-unknown-linux-musl.sh";
          CC_aarch64_unknown_linux_musl = "aarch64-unknown-linux-musl.sh";
        }
        // (
          if system == "x86_64-linux" then # default target would be gnu, hence we need to explicitly set it to musl
            { CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl"; }
          else if system == "aarch64-linux" then
            { CARGO_BUILD_TARGET = "aarch64-unknown-linux-musl"; }
          else
            { }
        );

      # Convert to "KEY=value" statements for the CI container
      envToList = env: builtins.map (k: "${k}=${env.${k}}") (builtins.attrNames env);

      # Convert to bash export statements (setting them in the env section is not robust, because stdenv's clang overrides CC/AR/RANLIB/SDKROOT)
      envToExports =
        env:
        builtins.concatStringsSep "\n" (
          builtins.map (k: "export ${k}=\"${env.${k}}\"") (builtins.attrNames env)
        );
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
          anttiharju = nur-anttiharju.packages.${system};
        in
        {
          default = pkgs.mkShell {
            packages = (devPackages pkgs anttiharju system) ++ [
              fenix.packages.${system}.stable.rust-analyzer
            ];

            shellHook =
              let
                env = zigEnv system;
              in
              ''
                ${envToExports env}
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

        in
        pkgs.lib.optionalAttrs (system == "x86_64-linux" || system == "aarch64-linux") {
          ci = pkgs.dockerTools.streamLayeredImage {
            name = "ci";
            tag = "flake";
            contents =
              (devPackages pkgs anttiharju system)
              ++ pkgs.stdenv.initialPath
              ++ [
                ld
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
              Env = (envToList (zigEnv system)) ++ [
                "NIX_LD=${pkgs.stdenv.cc.bintools.dynamicLinker}"
                "NIX_LD_LIBRARY_PATH=${
                  pkgs.lib.makeLibraryPath [
                    pkgs.stdenv.cc.cc.lib
                    pkgs.glibc
                  ]
                }"
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

              # Just avoid extra diffs when using a Dockerfile to inspect changes
              mkdir -p /proc /dev /sys
            '';
          };
        }
      );
    };
}
