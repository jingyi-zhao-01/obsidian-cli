#!/usr/bin/env bash
set -euo pipefail

# Generate PKGBUILD for AUR package
# Usage: generate-pkgbuild.sh <pkgname> <version> <sha256> <repo_owner> <repo_name>

build_pkgbuild() {
    local pkgname="$1"
    local version="$2"
    local sha256="$3"
    local repo_owner="$4"
    local repo_name="$5"

    printf '%s\n' \
        "pkgname=${pkgname}" \
        "pkgver=${version}" \
        "pkgrel=1" \
        'pkgdesc="Local-first CLI/TUI for indexing and querying Obsidian vaults"' \
        "arch=('x86_64')" \
        "url=\"https://github.com/${repo_owner}/${repo_name}\"" \
        "license=('Apache-2.0')" \
        "depends=('sqlite')" \
        "makedepends=('cargo' 'gcc' 'pkgconf')" \
        "options=('!lto')" \
        "source=(\"\${pkgname}-\${pkgver}.tar.gz::https://github.com/${repo_owner}/${repo_name}/archive/refs/tags/v\${pkgver}.tar.gz\")" \
        "sha256sums=(\"${sha256}\")" \
        '' \
        'build() {' \
        '  cd "${srcdir}/${pkgname}-${pkgver}"' \
        '' \
        '  # Prevent CI or user environment from injecting static Rust flags' \
        '  unset RUSTFLAGS' \
        '  unset CARGO_ENCODED_RUSTFLAGS' \
        '  unset CARGO_NET_OFFLINE' \
        '' \
        '  # Ensure GNU linker is used (avoid ld.lld issues)' \
        '  export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=gcc' \
        '' \
        '  cargo build --release --locked' \
        '}' \
        '' \
        'package() {' \
        '  cd "${srcdir}/${pkgname}-${pkgver}"' \
        '  install -Dm755 target/release/${pkgname} "${pkgdir}/usr/bin/${pkgname}"' \
        '}'
}

# Main execution
if [[ $# -ne 5 ]]; then
    echo "Usage: $0 <pkgname> <version> <sha256> <repo_owner> <repo_name>" >&2
    exit 1
fi

build_pkgbuild "$@"
