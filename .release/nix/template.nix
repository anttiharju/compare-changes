{
  lib,
  rustPlatform,
  fetchFromGitHub,
  nix-update-script,
}:

rustPlatform.buildRustPackage rec {
  pname = "${PKG_REPO}";
  version = "${PKG_VERSION}";
  revision = "${PKG_REV}";

  src = fetchFromGitHub {
    owner = "${PKG_OWNER}";
    repo = "${PKG_REPO}";
    rev = revision;
    hash = "${PKG_HASH}";
  };

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  cargoBuildFlags = [ "--all-features" ];

  meta = {
    homepage = "${PKG_HOMEPAGE}";
    description = "${PKG_DESC}";
    changelog = "https://github.com/${PKG_OWNER}/${PKG_REPO}/releases/tag/v$${version}";
    license = lib.licenses.mit;
    maintainers = with lib.maintainers; [ ${PKG_OWNER} ];
    mainProgram = "${PKG_REPO}";
  };
}
