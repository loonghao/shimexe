# Nix Build Acceleration

This project supports reproducible builds through `flake.nix`.

## Quick Start

```bash
# Build shimexe with dependency caching split from source build
nix build .#shimexe

# Enter development shell (rust, clippy, rustfmt, sccache)
nix develop
```

## Why this is faster

- `craneLib.buildDepsOnly` builds dependency artifacts separately.
- Code-only changes can reuse cached dependency artifacts.
- Team members and CI can share binary caches for the same derivation.

## CI suggestion

Use `nix build` (or `nix flake check`) in CI and enable a binary cache
(for example Cachix or your internal cache) to maximize cache hit ratio.
