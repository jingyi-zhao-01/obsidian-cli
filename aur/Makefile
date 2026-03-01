.PHONY: all validate download checksum generate-pkgbuild generate-srcinfo \
        syntax-check build-test clean help

PKGNAME = obsidian-cli-inspector
REPO_OWNER = jingyi-zhao-01
REPO_NAME = obsidian-cli-inspector
VERSION ?= unknown

TARBALL = source.tar.gz
SHA256_FILE = .sha256


# ============================================================
# Help
# ============================================================

help:
	@echo "AUR targets:"
	@echo "  make all VERSION=x.y.z"
	@echo "  make syntax-check"
	@echo "  make build-test"
	@echo "  make clean"

# ============================================================
# Entry Point
# ============================================================

all: validate download checksum generate-pkgbuild generate-srcinfo
	@echo "✓ Generated PKGBUILD and .SRCINFO for v$(VERSION)"


# ============================================================
# Step 1: Validate Input
# ============================================================

validate:
	@if [ "$(VERSION)" = "unknown" ]; then \
	  echo "ERROR: VERSION not specified"; \
	  echo "Usage: make all VERSION=x.y.z"; \
	  exit 1; \
	fi

# ============================================================
# Step 2: Download Tarball
# ============================================================

download:
	@echo "Downloading tarball for v$(VERSION)..."
	@rm -f $(TARBALL)
	@curl -sL "https://github.com/$(REPO_OWNER)/$(REPO_NAME)/archive/refs/tags/v$(VERSION).tar.gz" -o $(TARBALL)

# ============================================================
# Step 3: Compute SHA256
# ============================================================

checksum:
	@sha256sum $(TARBALL) | awk '{print $$1}' > $(SHA256_FILE)

# ============================================================
# Step 4: Generate PKGBUILD
# ============================================================

generate-pkgbuild:
	@echo "Generating PKGBUILD..."
	@SHA256=$$(cat $(SHA256_FILE)) && \
	{ \
	  printf '%s\n' \
	    "pkgname=$(PKGNAME)" \
	    "pkgver=$(VERSION)" \
	    "pkgrel=1" \
	    'pkgdesc="Local-first CLI/TUI for indexing and querying Obsidian vaults"' \
	    "arch=('x86_64')" \
	    "url=\"https://github.com/$(REPO_OWNER)/$(REPO_NAME)\"" \
	    "license=('Apache')" \
	    "depends=()" \
	    "makedepends=('cargo')" \
	    "source=(\"\$${pkgname}-\$${pkgver}.tar.gz::https://github.com/$(REPO_OWNER)/$(REPO_NAME)/archive/refs/tags/v\$${pkgver}.tar.gz\")" \
	    "sha256sums=(\"$$SHA256\")" \
	    '' \
	    'build() {' \
	    '  cd "$${srcdir}/$${pkgname}-$${pkgver}"' \
	    '  cargo build --release --locked' \
	    '}' \
	    '' \
	    'package() {' \
	    '  cd "$${srcdir}/$${pkgname}-$${pkgver}"' \
	    '  install -Dm755 target/release/$${pkgname} "$${pkgdir}/usr/bin/$${pkgname}"' \
	    '}'; \
	} > PKGBUILD

# ============================================================
# Step 5: Generate .SRCINFO
# ============================================================

generate-srcinfo:
	@echo "Generating .SRCINFO..."
	@if command -v makepkg >/dev/null 2>&1; then \
	  makepkg --printsrcinfo > .SRCINFO; \
	elif command -v docker >/dev/null 2>&1; then \
	  docker run --rm -v "$$(pwd)":/pkg -w /pkg archlinux:latest bash -lc "\
	    pacman -Sy --noconfirm --needed base-devel cargo >/dev/null && \
	    useradd -m builder && \
	    chown -R builder:builder /pkg && \
	    su builder -s /bin/bash -c 'cd /pkg && makepkg --printsrcinfo'\
	  " > .SRCINFO; \
	else \
	  echo "ERROR: makepkg not found and Docker unavailable."; \
	  exit 127; \
	fi

# ============================================================
# Syntax Check (Arch container)
# ============================================================

syntax-check:
	@if [ ! -f PKGBUILD ]; then echo "ERROR: PKGBUILD not found"; exit 1; fi
	@echo "Validating PKGBUILD syntax..."
	@docker run --rm -v "$$(pwd)":/pkg archlinux:latest bash -c "\
	  pacman -Syu --noconfirm base-devel cargo git >/dev/null && \
	  cd /pkg && makepkg --check \
	"
	@echo "✓ PKGBUILD syntax is valid"

# ============================================================
# Full Clean Arch Build Test
# ============================================================

build-test:
	@if [ ! -f PKGBUILD ]; then echo "ERROR: PKGBUILD not found"; exit 1; fi
	@echo "Running clean Arch build test..."
	@docker run --rm -v "$$(pwd)":/pkg archlinux:latest bash -c "\
	  set -e; \
	  pacman -Syu --noconfirm base-devel cargo git >/dev/null && \
	  useradd -m builder; \
	  chown -R builder:builder /pkg; \
	  su builder -s /bin/bash -c 'cd /pkg && makepkg -i --noconfirm --cleanbuild --clean'; \
	"
	@echo "✓ Clean build test passed"

# ============================================================
# Cleanup
# ============================================================

clean:
	@rm -f PKGBUILD .SRCINFO $(TARBALL) $(SHA256_FILE)
	@echo "✓ Cleaned AUR artifacts"