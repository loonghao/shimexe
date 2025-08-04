#!/bin/bash
# Update package manager configuration files with new version
# This script is called by release-please to update version numbers in package manager files

set -euo pipefail

VERSION="${1:-}"
PROJECT_ROOT="${2:-.}"

if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version> [project_root]"
    echo "Example: $0 1.2.3"
    exit 1
fi

cd "$PROJECT_ROOT"

echo "Updating package manager configurations to version $VERSION"

# Function to update JSON files using jq
update_json_file() {
    local file_path="$1"
    local version="$2"
    
    if [ -f "$file_path" ]; then
        echo "Updating $file_path"
        
        if command -v jq >/dev/null 2>&1; then
            # Use jq for precise JSON manipulation
            jq --arg version "$version" '
                .version = $version |
                if .architecture."64bit" then
                    .architecture."64bit".url |= gsub("v[0-9]+\\.[0-9]+\\.[0-9]+"; "v" + $version) |
                    .architecture."64bit".url |= gsub("shimexe-[0-9]+\\.[0-9]+\\.[0-9]+"; "shimexe-" + $version) |
                    .architecture."64bit".extract_dir |= gsub("shimexe-[0-9]+\\.[0-9]+\\.[0-9]+"; "shimexe-" + $version)
                else . end |
                if .architecture.arm64 then
                    .architecture.arm64.url |= gsub("v[0-9]+\\.[0-9]+\\.[0-9]+"; "v" + $version) |
                    .architecture.arm64.url |= gsub("shimexe-[0-9]+\\.[0-9]+\\.[0-9]+"; "shimexe-" + $version) |
                    .architecture.arm64.extract_dir |= gsub("shimexe-[0-9]+\\.[0-9]+\\.[0-9]+"; "shimexe-" + $version)
                else . end
            ' "$file_path" > "${file_path}.tmp" && mv "${file_path}.tmp" "$file_path"
        else
            # Fallback to sed for basic replacements
            sed -i.bak \
                -e "s/\"version\": \"[^\"]*\"/\"version\": \"$version\"/g" \
                -e "s/v[0-9]\+\.[0-9]\+\.[0-9]\+/v$version/g" \
                -e "s/shimexe-[0-9]\+\.[0-9]\+\.[0-9]\+/shimexe-$version/g" \
                "$file_path"
            rm -f "${file_path}.bak"
        fi
        
        echo "✓ Updated $file_path"
    else
        echo "Warning: File not found: $file_path"
    fi
}

# Function to update Ruby files (Homebrew)
update_ruby_file() {
    local file_path="$1"
    local version="$2"
    
    if [ -f "$file_path" ]; then
        echo "Updating $file_path"
        
        sed -i.bak \
            -e "s/version \"[^\"]*\"/version \"$version\"/g" \
            -e "s/shimexe-[0-9]\+\.[0-9]\+\.[0-9]\+/shimexe-$version/g" \
            -e "s/v[0-9]\+\.[0-9]\+\.[0-9]\+/v$version/g" \
            "$file_path"
        rm -f "${file_path}.bak"
        
        echo "✓ Updated $file_path"
    else
        echo "Warning: File not found: $file_path"
    fi
}

# Function to update XML template files (Chocolatey)
update_xml_template() {
    local file_path="$1"
    local version="$2"
    
    if [ -f "$file_path" ]; then
        echo "Updating $file_path"
        
        sed -i.bak "s/{{VERSION}}/$version/g" "$file_path"
        rm -f "${file_path}.bak"
        
        echo "✓ Updated $file_path"
    else
        echo "Warning: File not found: $file_path"
    fi
}

# Update package manager files
echo "Starting package manager configuration updates..."

# Update Scoop configuration
update_json_file "pkg/scoop/shimexe.json" "$VERSION"

# Update Homebrew formula
update_ruby_file "pkg/homebrew/shimexe.rb" "$VERSION"

# Update Chocolatey template
update_xml_template "pkg/chocolatey/shimexe.nuspec.template" "$VERSION"

echo "All package manager configurations updated successfully!"
