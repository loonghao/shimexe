# AGENTS.md — shimexe

> Cross-platform executable shim manager written in Rust. Ships as a CLI plus a
> reusable `shimexe-core` library; consumed by developers and by package
> managers (crates.io, Homebrew, Scoop, Chocolatey, Nix).
> Navigation map for AI agents, not a reference manual. Follow the links; do
> not read everything up front.

## Build & test

No justfile in this repo — use cargo directly.

```bash
cargo build --workspace       # build the CLI binary and shimexe-core
cargo test --workspace        # unit + integration tests (tests/*.rs)
cargo build --release         # LTO + panic=abort + stripped release binary
cargo fmt --check             # rustfmt gate (CI job "Rustfmt")
cargo clippy --workspace      # lint gate (CI job "Clippy")
cargo doc --no-deps           # API docs (CI job "Docs")
```

Full local equivalent of CI:

```bash
cargo fmt --check && cargo clippy --workspace && cargo test --workspace
```

`build.rs` embeds a Windows icon via `winres`; on non-Windows hosts it is a
no-op. `flake.nix` exists for Nix users (`nix flake check` is a CI job).

## Repo layout

| Path | Role |
|---|---|
| `src/main.rs` | CLI entry point (`[[bin]] shimexe`) |
| `src/commands/` | One module per subcommand: `add`, `remove`, `list`, `run`, `update`, `update_check`, `auto_update`, `init`, `validate` |
| `src/shim_manager.rs`, `src/path_manager.rs` | CLI-level shim orchestration and PATH handling |
| `crates/shimexe-core/` | The reusable library — config, downloader, archive, template, runner, updater, traits |
| `crates/shimexe-core/benches/` | Criterion benchmarks (`scripts/run-benchmarks.ps1`) |
| `tests/` | Integration tests (`*_tests.rs`), one file per subsystem |
| `docs/` | `shim-configuration.md`, `api-guide.md`, `PACKAGE_MANAGERS.md`, `NIX.md`, `icon-design.md` |
| `scripts/` | Installers plus `update-package-versions.{sh,ps1}` (bumps every package-manifest version) |
| `pkg/` | Package-manager definitions: `homebrew/`, `scoop/`, `chocolatey/` |
| `examples/` | `vx_integration_simple.rs` — using shimexe-core as a library |
| `assets/`, `build-icon.ps1` | Icon sources and the icon build helper |

## Release

- release-please drives versioning from Conventional Commits on `main`
  (`release-please-config.json`, `release-type: rust`, `cargo-workspace`
  plugin). `.release-please-manifest.json` is the single source of version truth.
- `feat:` → minor, `fix:` → patch, `chore:`/`docs:`/`ci:` → **no release**.
- Use `chore:`/`docs:` for config and doc work so release-please does not cut a
  valueless version.
- On release, `release.yml` / `release-assets.yml` / `package-publish.yml`
  publish the crates.io crate, GitHub release binaries, and the Homebrew, Scoop
  and Chocolatey packages. The root `Cargo.toml` `[package].version` and the
  workspace version are both managed by release-please — do not hand-edit.

## Do / Don't

- **Do** keep reusable logic in `crates/shimexe-core`; the root crate is a thin
  CLI over it.
- **Do** add an integration test under `tests/` for any new config or shim
  behaviour — that is where the existing coverage lives.
- **Do** keep `edition = "2021"` and the workspace `[lints]` (clippy
  `uninlined_format_args` is deliberately allowed).
- **Don't** hand-edit version numbers. Read
  `.release-please-manifest.json` first, then use
  `scripts/update-package-versions.sh` for package manifests.
- **Don't** add `CLAUDE.md` / `GEMINI.md` / `CURSOR.md` / `ANTHROPIC.md` /
  `OPENAI.md` / `COPILOT.md` / `CODEBUDDY.md` / `.cursorrules` / `.clinerules` /
  `.windsurfrules` at the root. This file is the only agent contract file.
- **Don't** hardcode an exact version in tests (`assert_eq!(VERSION, "X.Y.Z")`)
  — release-please bumps will break it. Compare semantically or read the crate
  version from Cargo metadata.
- **Don't** commit build artifacts to the repo root (`target/`, `*.exe`,
  `coverage.json`, `clippy_check.txt`, `commit_msg.txt`).
