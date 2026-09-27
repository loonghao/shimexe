# AGENTS.md — shimexe

> Navigation map, not a reference manual. Follow the links; don't read
> everything upfront.

shimexe is a cross-platform executable shim manager written in Rust. It turns
any executable into a portable shim with environment-variable expansion,
TOML configuration, and HTTP download support.

---

## Repository Contract

**Rust workspace — run cargo at the workspace root, as CI does.**

| Task | Command |
|------|---------|
| Build | `cargo build --workspace` |
| Release build | `cargo build --release --workspace` |
| Test | `cargo test --workspace` |
| Lint | `cargo clippy --all-targets --all-features -- -D warnings` |
| Format check | `cargo fmt --all --check` |
| Format | `cargo fmt --all` |
| Docs | `cargo doc --no-deps --document-private-items --workspace` |

**Repository layout**

| Path | Role |
|------|------|
| `crates/` | Workspace members |
| `src/` | Top-level crate sources |
| `tests/` | Integration tests |
| `docs/` | Documentation |
| `examples/` | Example shim configurations |
| `pkg/`, `scripts/` | Packaging and release helpers |
| `assets/` | Icons and static assets |

**Release flow** — `release-please` on `main` drives `CHANGELOG.md` and the version in
`Cargo.toml` from Conventional Commit subjects. Tagging and binary
publishing run in CI. Never edit `CHANGELOG.md` or a version string by hand.

**Prohibitions**

- Do not edit `CHANGELOG.md` or version strings manually.
- Do not add a second agent contract file at the repository root; `AGENTS.md` is the single source.
- Do not weaken `clippy` lints — CI builds with `-D warnings`.
- Do not commit an unformatted tree — `cargo fmt --all --check` runs in CI.

---

## Agent Contract Files

`AGENTS.md` is the **only** agent contract file at the repository root. It is the
native instruction file for Codex, OpenCode, Cursor, GitHub Copilot, Windsurf,
Cline, Roo Code, Kiro, Trae, and Augment, and Claude Code falls back to it when
no `CLAUDE.md` exists — so do not add `CLAUDE.md`, `GEMINI.md`, `CURSOR.md`, or
any other vendor-specific variant.

**Gemini CLI exception:** Gemini CLI defaults its context file to `GEMINI.md`. To
make it read `AGENTS.md`, set `context.fileName` once in `~/.gemini/settings.json`:

```json
{
  "context": {
    "fileName": ["AGENTS.md", "GEMINI.md"]
  }
}
```
