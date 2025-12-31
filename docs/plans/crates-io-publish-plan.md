# Zrd Crates.io Publishing Plan

## Overview

Publish the zrd text editor ecosystem to crates.io, consisting of three crates: `zrd-core` (shared editing engine), `zrd-tui` (terminal interface), and `zrd-gpui` (GUI interface). This enables users to install zrd via `cargo install zrd-tui`.

## Success Criteria

- [ ] `zrd-core` published and installable as a library dependency
- [ ] `zrd-tui` published with working `cargo install zrd-tui` producing `zrd` binary
- [ ] `cargo publish --dry-run` passes for all published crates
- [ ] GitHub Actions automatically publishes on version tags
- [ ] All tests pass before publishing

## Crate Name Availability

All names are **available** (verified via crates.io API returning 404):
- `zrd` - Available
- `zrd-core` - Available
- `zrd-tui` - Available
- `zrd-gpui` - Available

## Technical Approach

Publish crates in dependency order: `zrd-core` first, then `zrd-tui`. Defer `zrd-gpui` to a future release due to GPUI's platform limitations (macOS only) and to reduce initial complexity.

### Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Publish zrd-gpui? | Defer to v0.2.0 | GPUI is macOS-only; reduces complexity for initial release |
| Version strategy | 0.1.0 for all | Already configured in workspace; standard for initial release |
| Readme location | Root README.md | Use workspace README with crate-specific sections |
| Keywords | editor, text-editor, tui, cli | Standard discoverability terms |
| Categories | command-line-utilities, text-editors | Official crates.io categories |

---

## Implementation Phases

### Phase 1: Cargo.toml Metadata (estimated: 15min)

**Goal**: All required crates.io fields populated and dry-run passes.

**Pre-conditions**:
- Working directory at project root
- All tests passing

**Steps**:

1. [ ] Update workspace `Cargo.toml` to add shared metadata:

```toml
[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Doug Lance"]
license = "MIT"
repository = "https://github.com/douglance/zrd"
readme = "README.md"
keywords = ["editor", "text-editor", "tui"]
categories = ["command-line-utilities", "text-editors"]
```

2. [ ] Update `zrd-core/Cargo.toml`:

```toml
[package]
name = "zrd-core"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
readme.workspace = true
keywords = ["editor", "text-editor", "buffer", "undo"]
categories = ["text-editors", "text-processing"]
description = "Core editing engine for zrd text editor - platform-agnostic buffer management with undo/redo"
```

3. [ ] Update `zrd-tui/Cargo.toml` with version on zrd-core dependency:

```toml
[package]
name = "zrd-tui"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
readme.workspace = true
keywords.workspace = true
categories.workspace = true
description = "A minimal terminal text editor with vim-like efficiency"

[[bin]]
name = "zrd"
path = "src/main.rs"

[dependencies]
zrd-core = { version = "0.1.0", path = "../zrd-core" }
ratatui = "0.26"
crossterm = "0.27"
anyhow = "1.0"
notify = "6.1"
```

4. [ ] Update `zrd-gpui/Cargo.toml` to add version (for future publishing):

```toml
[dependencies]
zrd-core = { version = "0.1.0", path = "../zrd-core" }
gpui = "0.2"
```

**Verification**:
```bash
cargo publish --dry-run -p zrd-core
cargo publish --dry-run -p zrd-tui
```

Expected: Both commands complete with "warning: aborting upload due to dry run"

**Rollback**:
```bash
git checkout -- Cargo.toml zrd-*/Cargo.toml
```

---

### Phase 2: Documentation (estimated: 20min)

**Goal**: Crate documentation meets crates.io quality standards.

**Pre-conditions**:
- Phase 1 complete

**Steps**:

1. [ ] Add crate-level documentation to `zrd-core/src/lib.rs`:

```rust
//! # zrd-core
//!
//! Platform-agnostic text editing engine for the zrd editor family.
//!
//! This crate provides the core buffer management, cursor movement, and
//! undo/redo functionality shared between zrd-tui and zrd-gpui.
//!
//! ## Features
//!
//! - UTF-8 aware text buffer with line-based storage
//! - Full cursor navigation (character, word, line, document)
//! - Selection support with anchor-based model
//! - Undo/redo with intelligent chunking (500ms grouping)
//! - List continuation (markdown, numbered lists)
//! - File I/O with automatic directory creation
//!
//! ## Example
//!
//! ```rust
//! use zrd_core::{EditorEngine, EditorAction};
//!
//! let mut engine = EditorEngine::new();
//! engine.handle_action(EditorAction::TypeString("Hello, world!".into()));
//! engine.handle_action(EditorAction::MoveToBeginningOfLine);
//! engine.handle_action(EditorAction::SelectWordRight);
//!
//! assert_eq!(engine.state().line(0), Some("Hello, world!"));
//! ```

pub mod actions;
pub mod engine;
pub mod state;

pub use actions::EditorAction;
pub use engine::EditorEngine;
pub use state::{BufferPosition, EditorState};
```

2. [ ] Ensure all public types have doc comments (already done for most)

3. [ ] Run doc tests:
```bash
cargo test --doc -p zrd-core
```

**Verification**:
```bash
cargo doc -p zrd-core --no-deps --open
```

Expected: Documentation renders correctly with examples

**Rollback**:
```bash
git checkout -- zrd-core/src/lib.rs
```

---

### Phase 3: Update GitHub Actions (estimated: 15min)

**Goal**: Automated publishing workflow that handles workspace correctly.

**Pre-conditions**:
- Phases 1-2 complete
- CARGO_TOKEN secret configured in GitHub repository

**Steps**:

1. [ ] Update `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

permissions:
  contents: write

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - name: Run tests
        run: cargo test --all

  build:
    needs: test
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            artifact_name: zrd
            asset_name: zrd-linux-x86_64
          - os: macos-latest
            target: x86_64-apple-darwin
            artifact_name: zrd
            asset_name: zrd-macos-x86_64
          - os: macos-latest
            target: aarch64-apple-darwin
            artifact_name: zrd
            asset_name: zrd-macos-aarch64
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            artifact_name: zrd.exe
            asset_name: zrd-windows-x86_64.exe

    runs-on: ${{ matrix.os }}

    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
        with:
          target: ${{ matrix.target }}

      - name: Build TUI
        run: cargo build --release --target ${{ matrix.target }} -p zrd-tui

      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.asset_name }}
          path: ./target/${{ matrix.target }}/release/${{ matrix.artifact_name }}

  release:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Download all artifacts
        uses: actions/download-artifact@v4
        with:
          path: artifacts

      - name: Create release
        uses: softprops/action-gh-release@v2
        with:
          files: artifacts/**/*
          generate_release_notes: true

  publish-crates:
    needs: release
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1

      - name: Publish zrd-core
        run: cargo publish -p zrd-core --token ${{ secrets.CARGO_TOKEN }}

      - name: Wait for crates.io index
        run: sleep 30

      - name: Publish zrd-tui
        run: cargo publish -p zrd-tui --token ${{ secrets.CARGO_TOKEN }}
```

**Verification**:
```bash
# Validate YAML syntax
cat .github/workflows/release.yml | python3 -c "import yaml, sys; yaml.safe_load(sys.stdin)"
```

**Rollback**:
```bash
git checkout -- .github/workflows/release.yml
```

---

### Phase 4: Pre-publish Verification (estimated: 10min)

**Goal**: Verify everything works before actual publish.

**Pre-conditions**:
- Phases 1-3 complete

**Steps**:

1. [ ] Run full test suite:
```bash
cargo test --all
```

2. [ ] Run clippy:
```bash
cargo clippy --all -- -D warnings
```

3. [ ] Format check:
```bash
cargo fmt --all -- --check
```

4. [ ] Dry-run both crates:
```bash
cargo publish --dry-run -p zrd-core
cargo publish --dry-run -p zrd-tui
```

5. [ ] Build release binaries locally:
```bash
cargo build --release -p zrd-tui
./target/release/zrd --help
```

**Verification**:
All commands complete without errors.

**Rollback**:
N/A - verification only.

---

### Phase 5: Publish to Crates.io (estimated: 10min)

**Goal**: Crates live on crates.io.

**Pre-conditions**:
- Phase 4 complete
- crates.io account with API token
- Changes committed and pushed

**Steps**:

1. [ ] Ensure logged into crates.io:
```bash
cargo login
```

2. [ ] Publish zrd-core first:
```bash
cargo publish -p zrd-core
```

3. [ ] Wait for crates.io index update (30-60 seconds):
```bash
sleep 60
```

4. [ ] Publish zrd-tui:
```bash
cargo publish -p zrd-tui
```

5. [ ] Create and push git tag:
```bash
git tag v0.1.0
git push origin v0.1.0
```

**Verification**:
```bash
# Test installation from crates.io
cargo install zrd-tui

# Verify binary works
zrd --help
```

**Rollback**:
Crates.io publishes cannot be fully rolled back, but you can:
1. Yank the version: `cargo yank --version 0.1.0 zrd-core`
2. This prevents new installs but existing installs continue working

---

## Risk Mitigation

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Name squatting between dry-run and publish | Low | High | Publish same day as final verification |
| zrd-tui publish fails due to zrd-core not indexed | Medium | Medium | 60-second wait between publishes; retry if needed |
| CARGO_TOKEN secret missing | Medium | High | Verify secret exists before tagging |
| Breaking API after publish | Medium | High | Use 0.1.x for breaking changes in 0.x series |
| GPUI platform issues block install | Low | Medium | Deferred GPUI to v0.2.0 |

---

## Dependencies

- **crates.io account**: Required for publishing
- **CARGO_TOKEN GitHub secret**: Required for CI publishing
- **gpui 0.2.x**: External dependency for zrd-gpui (deferred)

---

## Out of Scope

- Publishing `zrd-gpui` (deferred to v0.2.0 due to macOS-only GPUI)
- Setting up crates.io badges in README
- Changelog automation
- Semantic versioning automation
- Multiple maintainer accounts

---

## Post-Publish Checklist

After successful publish:

1. [ ] Verify crates appear on crates.io
2. [ ] Test `cargo install zrd-tui` on a clean machine
3. [ ] Update README with crates.io badge
4. [ ] Announce release (if desired)
5. [ ] Plan zrd-gpui publish for v0.2.0

---

## Future Considerations (v0.2.0+)

1. **zrd-gpui publishing**:
   - Document macOS-only requirement clearly
   - Consider `[target.'cfg(target_os = "macos")'.dependencies]` pattern
   - May need platform-specific installation instructions

2. **Version synchronization**:
   - All crates share workspace version
   - Consider cargo-release for automation

3. **Feature flags**:
   - Could add optional features to zrd-core (e.g., `serde` for serialization)
