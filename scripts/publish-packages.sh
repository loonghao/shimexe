#!/usr/bin/env bash
# Publish shimexe to package managers (Scoop bucket + Homebrew tap).
# Uses pre-built GitHub Release assets - no building required.
#
# Usage:
#   scripts/publish-packages.sh --version 0.5.15 --github-token "$GH_TOKEN" \
#     --publish-scoop --scoop-bucket-repo loonghao/scoop-bucket \
#     --publish-homebrew --homebrew-tap-repo loonghao/homebrew-tap
#   scripts/publish-packages.sh --version 0.5.15 --dry-run

set -euo pipefail

REPO="loonghao/shimexe"
VERSION=""
GITHUB_TOKEN="${GH_TOKEN:-}"
SCOOP_TOKEN=""
HOMEBREW_TOKEN=""
PUBLISH_SCOOP=false
PUBLISH_HOMEBREW=false
SCOOP_BUCKET_REPO="loonghao/scoop-bucket"
HOMEBREW_TAP_REPO="loonghao/homebrew-tap"
DRY_RUN=false

log()  { echo "[publish-packages] $*"; }
warn() { echo "[publish-packages] WARNING: $*" >&2; }
die()  { echo "[publish-packages] ERROR: $*" >&2; exit 1; }

while [[ $# -gt 0 ]]; do
  case "$1" in
    --version)           VERSION="$2"; shift 2 ;;
    --github-token)      GITHUB_TOKEN="$2"; shift 2 ;;
    --scoop-token)       SCOOP_TOKEN="$2"; shift 2 ;;
    --homebrew-token)    HOMEBREW_TOKEN="$2"; shift 2 ;;
    --publish-scoop)     PUBLISH_SCOOP=true; shift ;;
    --scoop-bucket-repo) SCOOP_BUCKET_REPO="${2:-$SCOOP_BUCKET_REPO}"; shift 2 ;;
    --publish-homebrew)  PUBLISH_HOMEBREW=true; shift ;;
    --homebrew-tap-repo) HOMEBREW_TAP_REPO="${2:-$HOMEBREW_TAP_REPO}"; shift 2 ;;
    --dry-run)           DRY_RUN=true; shift ;;
    *) die "Unknown argument: $1" ;;
  esac
done

[[ -n "$VERSION" ]] || die "--version is required"
[[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+ ]] || die "invalid version: $VERSION (expected x.y.z without a v prefix)"

SCOOP_TOKEN="${SCOOP_TOKEN:-$GITHUB_TOKEN}"
HOMEBREW_TOKEN="${HOMEBREW_TOKEN:-$GITHUB_TOKEN}"

BASE_URL="https://github.com/${REPO}/releases/download/v${VERSION}"
WORK_DIR="$(mktemp -d)"
trap 'rm -rf "$WORK_DIR"' EXIT

# fetch <url> <out-file>
fetch() {
  local url="$1" out="$2"
  if [[ -n "$GITHUB_TOKEN" ]]; then
    curl -fsSL -H "Authorization: token ${GITHUB_TOKEN}" "$url" -o "$out"
  else
    curl -fsSL "$url" -o "$out"
  fi
}

sha256_of() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  else
    shasum -a 256 "$1" | awk '{print $1}'
  fi
}

git_bot_setup() {
  git config --global user.name "github-actions[bot]"
  git config --global user.email "github-actions[bot]@users.noreply.github.com"
}

# clone_and_push <repo> <token> <dir> <commit-msg> <src-file> <dst-path>
clone_and_push() {
  local repo="$1" token="$2" dir="$3" msg="$4" src="$5" dst="$6"
  local clean_token url
  clean_token="$(printf '%s' "$token" | tr -d '\n\r')"
  if [[ -z "$clean_token" ]]; then
    die "no token available to push to ${repo} (pass the repo-specific token or --github-token)"
  fi
  url="https://x-access-token:${clean_token}@github.com/${repo}.git"
  git clone --depth 1 "$url" "$dir"
  mkdir -p "$(dirname "${dir}/${dst}")"
  cp "$src" "${dir}/${dst}"
  git -C "$dir" add "$dst"
  if git -C "$dir" diff --cached --quiet; then
    log "no changes for ${repo}"
  else
    git -C "$dir" commit -m "$msg"
    git -C "$dir" push
    log "pushed ${dst} to ${repo}"
  fi
}

# build_scoop_manifest <x64-hash> [arm64-hash] -> manifest JSON on stdout
build_scoop_manifest() {
  local x64_hash="$1" arm64_hash="${2:-}"
  jq -n \
    --arg version "$VERSION" \
    --arg x64_hash "$x64_hash" \
    --arg arm64_hash "$arm64_hash" \
    '{
       version: $version,
       description: "The Modern Executable Shim Manager - Transform any executable into a smart, portable shim with HTTP download support",
       homepage: "https://github.com/loonghao/shimexe",
       license: "MIT",
       architecture: {
         "64bit": {url: ("https://github.com/loonghao/shimexe/releases/download/v" + $version + "/shimexe-x86_64-pc-windows-msvc.zip"), hash: $x64_hash},
         "arm64": {url: ("https://github.com/loonghao/shimexe/releases/download/v" + $version + "/shimexe-aarch64-pc-windows-msvc.zip"), hash: $arm64_hash}
       },
       bin: "shimexe.exe",
       checkver: {github: "https://github.com/loonghao/shimexe"},
       autoupdate: {
         architecture: {
           "64bit": {url: ("https://github.com/loonghao/shimexe/releases/download/v" + "$" + "version/shimexe-x86_64-pc-windows-msvc.zip")},
           "arm64": {url: ("https://github.com/loonghao/shimexe/releases/download/v" + "$" + "version/shimexe-aarch64-pc-windows-msvc.zip")}
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
     }'
}

publish_scoop() {
  log "Publishing Scoop manifest for shimexe v${VERSION} (bucket: ${SCOOP_BUCKET_REPO})"
  local x64_hash arm64_hash="" manifest

  fetch "${BASE_URL}/shimexe-x86_64-pc-windows-msvc.zip" "${WORK_DIR}/x64.zip"
  x64_hash="$(sha256_of "${WORK_DIR}/x64.zip")"
  [[ "$x64_hash" =~ ^[0-9a-f]{64}$ ]] || die "invalid x64 sha256: ${x64_hash}"

  if fetch "${BASE_URL}/shimexe-aarch64-pc-windows-msvc.zip" "${WORK_DIR}/arm64.zip" 2>/dev/null; then
    arm64_hash="$(sha256_of "${WORK_DIR}/arm64.zip")"
    [[ "$arm64_hash" =~ ^[0-9a-f]{64}$ ]] || die "invalid arm64 sha256: ${arm64_hash}"
  else
    warn "no ARM64 Windows asset for v${VERSION}; publishing 64-bit only manifest"
  fi

  manifest="$(build_scoop_manifest "$x64_hash" "$arm64_hash")"
  if [[ -z "$arm64_hash" ]]; then
    manifest="$(echo "$manifest" | jq 'del(.architecture.arm64) | del(.autoupdate.architecture.arm64)')"
  fi
  echo "$manifest" | jq . > "${WORK_DIR}/shimexe.json"
  jq -e . "${WORK_DIR}/shimexe.json" > /dev/null

  if [[ "$DRY_RUN" == true ]]; then
    cp "${WORK_DIR}/shimexe.json" pkg/scoop/shimexe.json
    log "dry-run: updated pkg/scoop/shimexe.json"
    return 0
  fi

  git_bot_setup
  clone_and_push "$SCOOP_BUCKET_REPO" "$SCOOP_TOKEN" "${WORK_DIR}/bucket" \
    "Update shimexe to ${VERSION}" "${WORK_DIR}/shimexe.json" "bucket/shimexe.json"
}

build_formula() {
  local mac_x64="$1" mac_arm64="$2" linux_x64="$3" linux_arm64="$4"
  cat <<RUBY
class Shimexe < Formula
  desc "The Modern Executable Shim Manager"
  homepage "https://github.com/loonghao/shimexe"
  version "${VERSION}"
  license "MIT"

  if OS.mac?
    if Hardware::CPU.arm?
      url "https://github.com/loonghao/shimexe/releases/download/v${VERSION}/shimexe-aarch64-apple-darwin.tar.gz"
      sha256 "${mac_arm64}"
    else
      url "https://github.com/loonghao/shimexe/releases/download/v${VERSION}/shimexe-x86_64-apple-darwin.tar.gz"
      sha256 "${mac_x64}"
    end
  elsif OS.linux?
    if Hardware::CPU.arm?
      url "https://github.com/loonghao/shimexe/releases/download/v${VERSION}/shimexe-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "${linux_arm64}"
    else
      url "https://github.com/loonghao/shimexe/releases/download/v${VERSION}/shimexe-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "${linux_x64}"
    end
  end

  def install
    bin.install "shimexe"
  end

  test do
    system "#{bin}/shimexe", "--version"
  end
end
RUBY
}

publish_homebrew() {
  log "Publishing Homebrew formula for shimexe v${VERSION} (tap: ${HOMEBREW_TAP_REPO})"
  local mac_x64 mac_arm64 linux_x64 linux_arm64

  # Every architecture must exist for the formula to be complete;
  # otherwise skip publishing instead of writing a broken formula.
  if ! fetch "${BASE_URL}/shimexe-x86_64-apple-darwin.tar.gz" "${WORK_DIR}/mac-x64.tar.gz" 2>/dev/null; then
    warn "no macOS assets for v${VERSION}; skipping Homebrew publish"
    return 0
  fi
  fetch "${BASE_URL}/shimexe-aarch64-apple-darwin.tar.gz" "${WORK_DIR}/mac-arm64.tar.gz"
  fetch "${BASE_URL}/shimexe-x86_64-unknown-linux-gnu.tar.gz" "${WORK_DIR}/linux-x64.tar.gz"
  fetch "${BASE_URL}/shimexe-aarch64-unknown-linux-gnu.tar.gz" "${WORK_DIR}/linux-arm64.tar.gz"

  mac_x64="$(sha256_of "${WORK_DIR}/mac-x64.tar.gz")"
  mac_arm64="$(sha256_of "${WORK_DIR}/mac-arm64.tar.gz")"
  linux_x64="$(sha256_of "${WORK_DIR}/linux-x64.tar.gz")"
  linux_arm64="$(sha256_of "${WORK_DIR}/linux-arm64.tar.gz")"

  build_formula "$mac_x64" "$mac_arm64" "$linux_x64" "$linux_arm64" > "${WORK_DIR}/shimexe.rb"

  if [[ "$DRY_RUN" == true ]]; then
    cp "${WORK_DIR}/shimexe.rb" pkg/homebrew/shimexe.rb
    log "dry-run: updated pkg/homebrew/shimexe.rb"
    return 0
  fi

  git_bot_setup
  clone_and_push "$HOMEBREW_TAP_REPO" "$HOMEBREW_TOKEN" "${WORK_DIR}/tap" \
    "shimexe ${VERSION}" "${WORK_DIR}/shimexe.rb" "Formula/shimexe.rb"
}

update_chocolatey_template() {
  if [[ -f pkg/chocolatey/shimexe.nuspec.template ]]; then
    sed -i "s/{{VERSION}}/${VERSION}/g" pkg/chocolatey/shimexe.nuspec.template
    log "updated pkg/chocolatey/shimexe.nuspec.template to ${VERSION}"
  fi
}

if [[ "$DRY_RUN" == true ]]; then
  log "dry-run mode: updating local package files only"
  publish_scoop
  publish_homebrew
  update_chocolatey_template
  log "dry-run complete"
  exit 0
fi

if [[ "$PUBLISH_SCOOP" == true ]]; then
  publish_scoop
fi
if [[ "$PUBLISH_HOMEBREW" == true ]]; then
  publish_homebrew
fi

log "publish complete"
