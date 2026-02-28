#!/usr/bin/env bash
set -euo pipefail

########################################
# Configuration
########################################

REPO_OWNER="jingyi-zhao-01"
REPO_NAME="obsidian-cli-inspector"
PKGNAME="obsidian-cli-inspector"
ARCH=("x86_64")

########################################
# Extract version from Cargo.toml
########################################

VERSION=$(grep '^version' Cargo.toml | head -n1 | sed 's/version = "\(.*\)"/\1/')

if [[ -z "$VERSION" ]]; then
  echo "Failed to extract version from Cargo.toml"
  exit 1
fi

TARBALL_URL="https://github.com/${REPO_OWNER}/${REPO_NAME}/archive/refs/tags/v${VERSION}.tar.gz"
TARBALL_FILE="${PKGNAME}-${VERSION}.tar.gz"

########################################
# Download source tarball
########################################

rm -f "$TARBALL_FILE"
curl -L "$TARBALL_URL" -o "$TARBALL_FILE"

########################################
# Compute SHA256
########################################

SHA256=$(sha256sum "$TARBALL_FILE" | awk '{print $1}')

rm -f "$TARBALL_FILE"

########################################
# Generate PKGBUILD
########################################

cat > PKGBUILD <<EOF
pkgname=${PKGNAME}
pkgver=${VERSION}
pkgrel=1
pkgdesc="Local-first CLI/TUI for indexing and querying Obsidian vaults"
arch=('${ARCH[@]}')
url="https://github.com/${REPO_OWNER}/${REPO_NAME}"
license=('Apache')
depends=()
makedepends=('cargo')
source=("\${pkgname}-\${pkgver}.tar.gz::${TARBALL_URL}")
sha256sums=('${SHA256}')

build() {
  cd "\${srcdir}/${REPO_NAME}-${VERSION}"
  cargo build --release --locked
}

package() {
  cd "\${srcdir}/${REPO_NAME}-${VERSION}"
  install -Dm755 "target/release/${PKGNAME}" "\${pkgdir}/usr/bin/${PKGNAME}"
  install -Dm644 LICENSE "\${pkgdir}/usr/share/licenses/\${pkgname}/LICENSE"
}
EOF

echo "PKGBUILD generated for version ${VERSION}"