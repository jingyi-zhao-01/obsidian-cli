.PHONY: all clean help syntax-check build-test

PKGNAME = obsidian-cli-inspector
REPO_OWNER = jingyi-zhao-01
REPO_NAME = obsidian-cli-inspector
VERSION ?= unknown

help:
	@echo "AUR publishing targets:"
	@echo "  make all VERSION=x.y.z      - Generate PKGBUILD and .SRCINFO"
	@echo "  make syntax-check           - Validate PKGBUILD syntax"
	@echo "  make build-test             - Full clean Arch build test"
	@echo "  make clean                  - Remove generated files"

all:
	@if [ "$(VERSION)" = "unknown" ]; then \
	  echo "ERROR: VERSION not specified"; \
	  echo "Usage: make VERSION=x.y.z"; \
	  exit 1; \
	fi
	@echo "Downloading tarball for v$(VERSION)..."
	@rm -f source.tar.gz
	@curl -sL "https://github.com/$(REPO_OWNER)/$(REPO_NAME)/archive/refs/tags/v$(VERSION).tar.gz" -o source.tar.gz || { echo "ERROR: Failed to fetch tarball"; rm -f source.tar.gz; exit 1; }
	@SHA256=$$(sha256sum source.tar.gz | awk '{print $$1}') && \
	  echo "Generating PKGBUILD..." && \
	  { \
	    printf '%s\n' \
	      'pkgname=$(PKGNAME)' \
	      'pkgver=$(VERSION)' \
	      'pkgrel=1' \
	      'pkgdesc="Local-first CLI/TUI for indexing and querying Obsidian vaults"' \
	      "arch=('x86_64')" \
	      'url="https://github.com/$(REPO_OWNER)/$(REPO_NAME)"' \
	      "license=('Apache')" \
	      'depends=()' \
	      "makedepends=('cargo')" \
	      'source=("$${pkgname}-$${pkgver}.tar.gz::https://github.com/$(REPO_OWNER)/$(REPO_NAME)/archive/refs/tags/v$${pkgver}.tar.gz")' \
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
	@rm -f source.tar.gz && \
	  echo "Generating .SRCINFO..." && \
	  if command -v makepkg >/dev/null 2>&1; then \
	    makepkg --printsrcinfo > .SRCINFO; \
	  elif [ -f /etc/os-release ] && grep -qi ubuntu /etc/os-release; then \
	    echo "Detected Ubuntu runner; generating .SRCINFO from PKGBUILD template..." && \
	    printf '%s\n' \
	      "pkgbase = $(PKGNAME)" \
	      "\tpkgdesc = Local-first CLI/TUI for indexing and querying Obsidian vaults" \
	      "\tpkgver = $(VERSION)" \
	      "\tpkgrel = 1" \
	      "\turl = https://github.com/$(REPO_OWNER)/$(REPO_NAME)" \
	      "\tarch = x86_64" \
	      "\tlicense = Apache" \
	      "\tmakedepends = cargo" \
	      "\tsource = $(PKGNAME)-$(VERSION).tar.gz::https://github.com/$(REPO_OWNER)/$(REPO_NAME)/archive/refs/tags/v$(VERSION).tar.gz" \
	      "\tsha256sums = $(SHA256)" \
	      "" \
	      "pkgname = $(PKGNAME)" \
	    > .SRCINFO; \
	  elif command -v docker >/dev/null 2>&1; then \
	    docker run --rm -v "$$(pwd)":/pkg -w /pkg archlinux:latest bash -lc "\
	      pacman -Sy --noconfirm --needed base-devel >/dev/null && \
	      useradd -m builder && \
	      chown -R builder:builder /pkg && \
	      su builder -s /bin/bash -c 'cd /pkg && makepkg --printsrcinfo'\
	    " > .SRCINFO; \
	  else \
	    echo "ERROR: makepkg not found and Docker unavailable. Cannot generate .SRCINFO."; \
	    exit 127; \
	  fi && \
	  echo "✓ Generated PKGBUILD and .SRCINFO for v$(VERSION)"

syntax-check:
	@if [ ! -f PKGBUILD ]; then echo "ERROR: PKGBUILD not found"; exit 1; fi
	@echo "Validating PKGBUILD syntax..."
	@docker run --rm -v "$$(pwd)":/pkg archlinux:latest bash -c "pacman -Syu --noconfirm base-devel git && cd /pkg && makepkg --check" || exit 1
	@echo "✓ PKGBUILD syntax is valid"

build-test:
	@if [ ! -f PKGBUILD ]; then echo "ERROR: PKGBUILD not found"; exit 1; fi
	@echo "Running full clean Arch build test..."
	@docker run --rm -v "$$(pwd)":/pkg archlinux:latest bash -c "\
	  set -e; \
	  pacman -Syu --noconfirm base-devel git rust cargo; \
	  useradd -m builduser; \
	  chown -R builduser:builduser /pkg; \
	  su builduser -c 'cd /pkg && makepkg -si --noconfirm --cleanbuild --clean'; \
	" || exit 1
	@echo "✓ Build test passed"

clean:
	@rm -f PKGBUILD .SRCINFO source.tar.gz
	@echo "✓ Cleaned AUR artifacts"
