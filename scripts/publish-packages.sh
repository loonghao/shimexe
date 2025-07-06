#!/bin/bash
# Enhanced shell script to publish shimexe packages to various package managers
# Usage: ./scripts/publish-packages.sh --version "0.1.3" --github-token "token" [--publish-homebrew] [--publish-scoop] [--dry-run]

set -euo pipefail

# Default values
VERSION=""
GITHUB_TOKEN=""
PUBLISH_HOMEBREW=false
PUBLISH_SCOOP=false
DRY_RUN=false
SCOOP_BUCKET_REPO="loonghao/scoop-bucket"
HOMEBREW_TAP_REPO="loonghao/homebrew-tap"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --version)
            VERSION="$2"
            shift 2
            ;;
        --github-token)
            GITHUB_TOKEN="$2"
            shift 2
            ;;
        --publish-homebrew)
            PUBLISH_HOMEBREW=true
            shift
            ;;
        --publish-scoop)
            PUBLISH_SCOOP=true
            shift
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --scoop-bucket-repo)
            SCOOP_BUCKET_REPO="$2"
            shift 2
            ;;
        --homebrew-tap-repo)
            HOMEBREW_TAP_REPO="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Validate required parameters
if [[ -z "$VERSION" ]]; then
    echo "Error: --version is required"
    exit 1
fi

if [[ -z "$GITHUB_TOKEN" ]]; then
    echo "Error: --github-token is required"
    exit 1
fi

# Configuration
REPO_OWNER="loonghao"
REPO_NAME="shimexe"
BASE_URL="https://github.com/$REPO_OWNER/$REPO_NAME"

echo "Publishing shimexe packages for version $VERSION"

# Function to get file hash
get_file_hash() {
    local url="$1"
    local temp_file=$(mktemp)
    
    if curl -sL "$url" -o "$temp_file"; then
        local hash=$(sha256sum "$temp_file" | cut -d' ' -f1)
        rm -f "$temp_file"
        echo "$hash"
    else
        echo "Failed to get hash for $url" >&2
        rm -f "$temp_file"
        return 1
    fi
}

# Function to update GitHub repository file
update_github_file() {
    local repo="$1"
    local path="$2"
    local content="$3"
    local message="$4"
    local token="$5"
    local branch="${6:-main}"
    
    # Get current file to get SHA
    local get_url="https://api.github.com/repos/$repo/contents/$path"
    local sha=""
    
    if response=$(curl -s -H "Authorization: token $token" \
                      -H "Accept: application/vnd.github.v3+json" \
                      -H "User-Agent: shimexe-publisher" \
                      "$get_url" 2>/dev/null); then
        sha=$(echo "$response" | jq -r '.sha // empty')
    fi
    
    # Prepare update payload
    local encoded_content=$(echo -n "$content" | base64 -w 0)
    local payload=$(jq -n \
        --arg message "$message" \
        --arg content "$encoded_content" \
        --arg branch "$branch" \
        --arg sha "$sha" \
        '{
            message: $message,
            content: $content,
            branch: $branch
        } + (if $sha != "" then {sha: $sha} else {} end)')
    
    # Update file
    local update_url="https://api.github.com/repos/$repo/contents/$path"
    curl -s -X PUT \
         -H "Authorization: token $token" \
         -H "Accept: application/vnd.github.v3+json" \
         -H "User-Agent: shimexe-publisher" \
         -H "Content-Type: application/json" \
         -d "$payload" \
         "$update_url" > /dev/null
}

# Define release URLs
declare -A ASSETS
ASSETS[windows-x64]="$BASE_URL/releases/download/v$VERSION/shimexe-$VERSION-x86_64-pc-windows-msvc.zip"
ASSETS[windows-arm64]="$BASE_URL/releases/download/v$VERSION/shimexe-$VERSION-aarch64-pc-windows-msvc.zip"
ASSETS[macos-x64]="$BASE_URL/releases/download/v$VERSION/shimexe-$VERSION-x86_64-apple-darwin.tar.xz"
ASSETS[macos-arm64]="$BASE_URL/releases/download/v$VERSION/shimexe-$VERSION-aarch64-apple-darwin.tar.xz"
ASSETS[linux-x64]="$BASE_URL/releases/download/v$VERSION/shimexe-$VERSION-x86_64-unknown-linux-gnu.tar.xz"
ASSETS[linux-arm64]="$BASE_URL/releases/download/v$VERSION/shimexe-$VERSION-aarch64-unknown-linux-gnu.tar.xz"

declare -A EXTRACT_DIRS
EXTRACT_DIRS[windows-x64]="shimexe-$VERSION-x86_64-pc-windows-msvc"
EXTRACT_DIRS[windows-arm64]="shimexe-$VERSION-aarch64-pc-windows-msvc"
EXTRACT_DIRS[macos-x64]="shimexe-$VERSION-x86_64-apple-darwin"
EXTRACT_DIRS[macos-arm64]="shimexe-$VERSION-aarch64-apple-darwin"
EXTRACT_DIRS[linux-x64]="shimexe-$VERSION-x86_64-unknown-linux-gnu"
EXTRACT_DIRS[linux-arm64]="shimexe-$VERSION-aarch64-unknown-linux-gnu"

echo "Calculating hashes for release assets..."

# Calculate hashes for all assets
declare -A HASHES
for platform in "${!ASSETS[@]}"; do
    echo "  Getting hash for $platform..."
    if hash=$(get_file_hash "${ASSETS[$platform]}"); then
        HASHES[$platform]="$hash"
        echo "    $hash"
    else
        echo "Failed to get hash for $platform"
        exit 1
    fi
done

# Update Scoop package
if [[ "$PUBLISH_SCOOP" == true ]]; then
    echo "Updating Scoop package..."
    
    if [[ -z "${HASHES[windows-x64]:-}" || -z "${HASHES[windows-arm64]:-}" ]]; then
        echo "Error: Windows hashes required for Scoop"
        exit 1
    fi
    
    # Create Scoop manifest
    scoop_manifest=$(jq -n \
        --arg version "$VERSION" \
        --arg url_x64 "${ASSETS[windows-x64]}" \
        --arg hash_x64 "${HASHES[windows-x64]}" \
        --arg extract_dir_x64 "${EXTRACT_DIRS[windows-x64]}" \
        --arg url_arm64 "${ASSETS[windows-arm64]}" \
        --arg hash_arm64 "${HASHES[windows-arm64]}" \
        --arg extract_dir_arm64 "${EXTRACT_DIRS[windows-arm64]}" \
        '{
            version: $version,
            description: "The Modern Executable Shim Manager - Transform any executable into a smart, portable shim with HTTP download support",
            homepage: "https://github.com/loonghao/shimexe",
            license: "MIT",
            architecture: {
                "64bit": {
                    url: $url_x64,
                    hash: $hash_x64,
                    extract_dir: $extract_dir_x64
                },
                arm64: {
                    url: $url_arm64,
                    hash: $hash_arm64,
                    extract_dir: $extract_dir_arm64
                }
            },
            bin: "shimexe.exe",
            checkver: {
                github: "https://github.com/loonghao/shimexe"
            },
            autoupdate: {
                architecture: {
                    "64bit": {
                        url: "https://github.com/loonghao/shimexe/releases/download/v$version/shimexe-$version-x86_64-pc-windows-msvc.zip",
                        extract_dir: "shimexe-$version-x86_64-pc-windows-msvc"
                    },
                    arm64: {
                        url: "https://github.com/loonghao/shimexe/releases/download/v$version/shimexe-$version-aarch64-pc-windows-msvc.zip",
                        extract_dir: "shimexe-$version-aarch64-pc-windows-msvc"
                    }
                }
            },
            notes: [
                "shimexe has been installed successfully!",
                "",
                "Quick start:",
                "  shimexe init --examples",
                "  shimexe add mytool --path https://example.com/tool.exe",
                "",
                "For more information, visit: https://github.com/loonghao/shimexe"
            ]
        }')
    
    if [[ "$DRY_RUN" == false ]]; then
        # Update Scoop bucket repository
        if update_github_file "$SCOOP_BUCKET_REPO" "bucket/shimexe.json" "$scoop_manifest" "chore: update shimexe to v$VERSION" "$GITHUB_TOKEN"; then
            echo "Updated Scoop bucket repository successfully!"
        else
            echo "Failed to update Scoop bucket"
            echo "Scoop manifest content:"
            echo "$scoop_manifest"
        fi
    else
        echo "Scoop manifest content:"
        echo "$scoop_manifest"
    fi
    
    echo "Scoop package updated successfully"
fi

# Update Homebrew formula
if [[ "$PUBLISH_HOMEBREW" == true ]]; then
    echo "Updating Homebrew formula..."

    required_platforms=("macos-x64" "macos-arm64" "linux-x64" "linux-arm64")
    for platform in "${required_platforms[@]}"; do
        if [[ -z "${HASHES[$platform]:-}" ]]; then
            echo "Error: Hash for $platform required for Homebrew"
            exit 1
        fi
    done

    # Create Homebrew formula
    homebrew_formula="class Shimexe < Formula
  desc \"The Modern Executable Shim Manager\"
  homepage \"https://github.com/loonghao/shimexe\"
  version \"$VERSION\"
  license \"MIT\"

  if OS.mac?
    if Hardware::CPU.arm?
      url \"${ASSETS[macos-arm64]}\"
      sha256 \"${HASHES[macos-arm64]}\"
    else
      url \"${ASSETS[macos-x64]}\"
      sha256 \"${HASHES[macos-x64]}\"
    end
  elsif OS.linux?
    if Hardware::CPU.arm?
      url \"${ASSETS[linux-arm64]}\"
      sha256 \"${HASHES[linux-arm64]}\"
    else
      url \"${ASSETS[linux-x64]}\"
      sha256 \"${HASHES[linux-x64]}\"
    end
  end

  def install
    bin.install \"shimexe\"

    # Install shell completions if available
    if (buildpath/\"completions\").exist?
      bash_completion.install \"completions/shimexe.bash\" => \"shimexe\"
      zsh_completion.install \"completions/_shimexe\"
      fish_completion.install \"completions/shimexe.fish\"
    end

    # Install man page if available
    if (buildpath/\"man\").exist?
      man1.install \"man/shimexe.1\"
    end
  end

  test do
    system \"#{bin}/shimexe\", \"--version\"
    system \"#{bin}/shimexe\", \"--help\"

    # Test basic functionality
    system \"#{bin}/shimexe\", \"init\"
    assert_predicate testpath/\".shimexe\", :exist?
  end
end"

    if [[ "$DRY_RUN" == false ]]; then
        # Update Homebrew tap repository
        if update_github_file "$HOMEBREW_TAP_REPO" "Formula/shimexe.rb" "$homebrew_formula" "chore: update shimexe to v$VERSION" "$GITHUB_TOKEN"; then
            echo "Updated Homebrew tap repository successfully!"
        else
            echo "Failed to update Homebrew tap"
            echo "Homebrew formula content:"
            echo "$homebrew_formula"
        fi
    else
        echo "Homebrew formula content:"
        echo "$homebrew_formula"
    fi

    echo "Homebrew formula updated successfully"
fi

# Update local package files
echo "Updating local package files..."

# Update local Scoop manifest
if [[ -f "pkg/scoop/shimexe.json" ]]; then
    jq --arg version "$VERSION" \
       --arg url_x64 "${ASSETS[windows-x64]}" \
       --arg hash_x64 "${HASHES[windows-x64]}" \
       --arg extract_dir_x64 "${EXTRACT_DIRS[windows-x64]}" \
       --arg url_arm64 "${ASSETS[windows-arm64]}" \
       --arg hash_arm64 "${HASHES[windows-arm64]}" \
       --arg extract_dir_arm64 "${EXTRACT_DIRS[windows-arm64]}" \
       '.version = $version |
        .architecture."64bit".url = $url_x64 |
        .architecture."64bit".hash = $hash_x64 |
        .architecture."64bit".extract_dir = $extract_dir_x64 |
        .architecture.arm64.url = $url_arm64 |
        .architecture.arm64.hash = $hash_arm64 |
        .architecture.arm64.extract_dir = $extract_dir_arm64' \
       "pkg/scoop/shimexe.json" > "pkg/scoop/shimexe.json.tmp" && \
    mv "pkg/scoop/shimexe.json.tmp" "pkg/scoop/shimexe.json"
    echo "Updated local Scoop manifest"
fi

# Update local Homebrew formula
if [[ -f "pkg/homebrew/shimexe.rb" ]]; then
    sed -i.bak \
        -e "s/version \"[0-9.]*\"/version \"$VERSION\"/" \
        -e "s/PLACEHOLDER_ARM64_SHA256/${HASHES[macos-arm64]}/" \
        -e "s/PLACEHOLDER_X86_64_SHA256/${HASHES[macos-x64]}/" \
        -e "s/PLACEHOLDER_LINUX_ARM64_SHA256/${HASHES[linux-arm64]}/" \
        -e "s/PLACEHOLDER_LINUX_X86_64_SHA256/${HASHES[linux-x64]}/" \
        "pkg/homebrew/shimexe.rb"
    rm -f "pkg/homebrew/shimexe.rb.bak"
    echo "Updated local Homebrew formula"
fi

# Summary
echo ""
echo "Package publishing summary:"
echo "========================="

if [[ "$PUBLISH_SCOOP" == true ]]; then
    echo "✓ Scoop: Updated"
else
    echo "- Scoop: Skipped"
fi

if [[ "$PUBLISH_HOMEBREW" == true ]]; then
    echo "✓ Homebrew: Updated"
else
    echo "- Homebrew: Skipped"
fi

if [[ "$DRY_RUN" == true ]]; then
    echo ""
    echo "Dry run completed. No packages were actually published."
else
    echo ""
    echo "Package publishing completed!"
fi

echo ""
echo "Next steps:"
echo "1. Commit and push the updated local package files"
echo "2. Test installations:"
if [[ "$PUBLISH_SCOOP" == true ]]; then
    echo "   - scoop bucket add loonghao https://github.com/$SCOOP_BUCKET_REPO"
    echo "   - scoop install shimexe"
fi
if [[ "$PUBLISH_HOMEBREW" == true ]]; then
    echo "   - brew tap $HOMEBREW_TAP_REPO"
    echo "   - brew install shimexe"
fi
echo "3. Monitor package manager repositories for any issues"
