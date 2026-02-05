#!/bin/bash
# Code coverage script for obsidian-cli
# Generates coverage reports in multiple formats for SonarQube/SonarCloud integration

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Obsidian CLI - Code Coverage Report Generator${NC}"
echo "=================================================="

# Check if cargo-tarpaulin is installed
if ! command -v cargo-tarpaulin &> /dev/null; then
    echo -e "${YELLOW}Installing cargo-tarpaulin...${NC}"
    cargo install cargo-tarpaulin --version 0.31.0
fi

# Create coverage directory
mkdir -p coverage

echo -e "${YELLOW}Generating coverage report...${NC}"

# Run tests with coverage
# Generate both LCOV and Cobertura XML formats for maximum compatibility with SonarQube
cargo tarpaulin \
    --out Xml \
    --output-dir coverage \
    --timeout 300 \
    --exclude-files tests/* \
    --skip-clean \
    --run-types Tests

# Also generate LCOV format as backup
cargo tarpaulin \
    --out Lcov \
    --output-dir coverage \
    --timeout 300 \
    --exclude-files tests/* \
    --skip-clean \
    --run-types Tests

echo -e "${GREEN}Coverage reports generated successfully!${NC}"
echo ""
echo "Generated files:"
ls -lh coverage/

echo ""
echo -e "${YELLOW}SonarQube Integration:${NC}"
echo "For SonarCloud/SonarQube, use one of:"
echo "  - coverage/cobertura.xml (for sonar.coverageReportPaths)"
echo "  - coverage/lcov.info (for sonar.coverage.exclusions)"
echo ""
echo "Example sonar-project.properties:"
echo "  sonar.sources=src"
echo "  sonar.tests=tests"
echo "  sonar.coverageReportPaths=coverage/cobertura.xml"
echo "  sonar.coverage.exclusions=tests/**"
