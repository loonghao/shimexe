#!/bin/bash
# Script to update package manager configurations with new version

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get version from command line or Cargo.toml
VERSION=${1:-$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')}

if [[ -z "$VERSION" ]]; then
    echo -e "${RED}Error: Could not determine version${NC}"
    echo "Usage: $0 [version]"
    exit 1
fi

echo -e "${BLUE}Updating package managers for version ${GREEN}$VERSION${NC}"

# Update Homebrew formula
echo -e "${YELLOW}Updating Homebrew formula...${NC}"
sed -i.bak "s/version \".*\"/version \"$VERSION\"/" homebrew/shimexe.rb
echo -e "${GREEN}✓ Updated homebrew/shimexe.rb${NC}"

# Update Scoop manifest
echo -e "${YELLOW}Updating Scoop manifest...${NC}"
sed -i.bak "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" scoop/shimexe.json
sed -i.bak "s|/v[0-9.]\+/|/v$VERSION/|g" scoop/shimexe.json
sed -i.bak "s/shimexe-[0-9.]\+-/shimexe-$VERSION-/g" scoop/shimexe.json
echo -e "${GREEN}✓ Updated scoop/shimexe.json${NC}"

# Clean up backup files
rm -f homebrew/shimexe.rb.bak scoop/shimexe.json.bak

echo -e "${GREEN}✅ All package managers updated to version $VERSION${NC}"
echo -e "${BLUE}Next steps:${NC}"
echo -e "  1. Commit the changes"
echo -e "  2. Create and push a git tag: ${YELLOW}git tag v$VERSION && git push origin v$VERSION${NC}"
echo -e "  3. The release workflow will automatically update the package repositories"
