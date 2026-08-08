# Agent Instructions for `charming-x-ansi`

> [!IMPORTANT]
> **Subsequent Cycle Requirement**: On every development cycle, before doing any work, the agent MUST inspect [`UPSTREAM_MAPPING.md`](file:///Users/jonny/Projects/charming/charming-x-ansi/UPSTREAM_MAPPING.md) to verify that all upstream Go files and examples are accounted for. When adding, modifying, or refactoring files, the agent MUST update [`UPSTREAM_MAPPING.md`](file:///Users/jonny/Projects/charming/charming-x-ansi/UPSTREAM_MAPPING.md) to reflect the current state.
>
> Run `scripts/verify_mapping.sh` to mechanically verify that every file in `upstream-go/` is accounted for in `UPSTREAM_MAPPING.md`.

## Core Rules & Workflow
1. Refer to the workspace-level rule in [`/Users/jonny/Projects/charming/AGENTS.md`](file:///Users/jonny/Projects/charming/AGENTS.md).
2. Maintain 100% rustdoc documentation.
3. Every ported file MUST include the guiding comment header:
   ```rust
   //! Cleanroom Rust port of upstream Go source file: `ansi/<upstream-go-filepath>`
   //! Upstream Target Tag / Version: `v0.11.2`
   ```
4. Verify all tests pass with `cargo test --all-targets` before committing.
5. The upstream is the `github.com/charmbracelet/x` monorepo at module tag `ansi/v0.11.2`;
   the ported module lives in the `ansi/` subdirectory of `upstream-go/`. Other modules of
   the monorepo are out of scope and must be explicitly marked so in the mapping.
6. Multi-version rule: the dependency tree requires additional x/ansi versions (v0.10.3,
   v0.11.6, v0.11.7). Port the earliest first, then diff-forward per
   `/Users/jonny/Projects/charming/DEPENDENCY_PLAN.md` §6.

## Releases
- GitHub Releases MUST match upstream: every upstream release tag tracked by this port must
  exist as a `v*` tag and a GitHub release on this repo (the publish workflow creates the
  release automatically from the tag). If upstream has no tagged releases (pseudo-version
  pins), no GitHub release is required.
- To release: push the `v*` tag (e.g. `git tag v2.0.8 && git push origin v2.0.8`); the
  workflow runs tests, example parity, creates the GitHub Release, and attempts the
  crates.io publish (non-fatal without a registry token).
- The crates.io publish step is tag-gated; dev pushes only run tests and parity.
