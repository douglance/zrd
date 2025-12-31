# Rename Project from "zlyph" to "zrd" Implementation Plan

## Overview

Rename all occurrences of "zlyph" to "zrd" throughout the workspace, including directory names, package names, binary names, import statements, config paths, and documentation. The goal is a complete, consistent rename with zero references to the old name remaining.

## Success Criteria

- [ ] All directories renamed: `zlyph-core` -> `zrd-core`, `zlyph-tui` -> `zrd-tui`, `zlyph-gpui` -> `zrd-gpui`
- [ ] All Cargo.toml package names updated
- [ ] All binary names updated: `zlyph` -> `zrd`, `zlyph-gui` -> `zrd-gui`
- [ ] All Rust imports updated: `zlyph_core` -> `zrd_core`
- [ ] Config directory updated: `~/.config/zlyph/` -> `~/.config/zrd/`
- [ ] GitHub workflow updated with new artifact names
- [ ] `cargo build` succeeds
- [ ] `cargo test -p zrd-core` passes
- [ ] `grep -r "zlyph" --include="*.rs" --include="*.toml"` returns no results

## Technical Approach

Rename directories first to avoid Git tracking issues, then update all file contents. The order matters because Cargo uses directory names to locate packages. By renaming directories first, we can then update Cargo.toml files to match the new paths.

### Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Directory rename order | Directories before content | Cargo expects member paths to match filesystem |
| Config migration | Change path only, no migration | Users can manually move existing config if needed |
| Binary names | `zrd` (TUI), `zrd-gui` (GPUI) | Matches pattern of short primary name |
| Documentation update | Full update | Keeps docs accurate and avoids confusion |

## Implementation Phases

### Phase 1: Rename Directories (estimated: 2min)

**Goal**: Rename all three package directories to their new names.

**Pre-conditions**:
- Clean git working tree (commit or stash pending changes)
- No processes holding locks on files in the directories

**Steps**:
1. [ ] Rename `zlyph-core/` to `zrd-core/`
2. [ ] Rename `zlyph-tui/` to `zrd-tui/`
3. [ ] Rename `zlyph-gpui/` to `zrd-gpui/`

**Commands**:
```bash
cd /Users/douglance/Developer/lv/dright
git mv zlyph-core zrd-core
git mv zlyph-tui zrd-tui
git mv zlyph-gpui zrd-gpui
```

**Verification**:
```bash
ls -la | grep -E "^d.*zrd"
# Should show: zrd-core, zrd-gpui, zrd-tui
```

**Rollback**:
```bash
git mv zrd-core zlyph-core
git mv zrd-tui zlyph-tui
git mv zrd-gpui zlyph-gpui
```

---

### Phase 2: Update Root Cargo.toml (estimated: 2min)

**Goal**: Update workspace configuration to reference new directory names.

**Pre-conditions**:
- Phase 1 complete (directories renamed)

**Steps**:
1. [ ] Update workspace members list in `/Users/douglance/Developer/lv/dright/Cargo.toml`:
   - `"zlyph-core"` -> `"zrd-core"`
   - `"zlyph-gpui"` -> `"zrd-gpui"`
   - `"zlyph-tui"` -> `"zrd-tui"`
2. [ ] Update repository URL:
   - `https://github.com/douglance/zlyph` -> `https://github.com/douglance/zrd`

**File**: `/Users/douglance/Developer/lv/dright/Cargo.toml`

**Before**:
```toml
[workspace]
members = [
    "zlyph-core",
    "zlyph-gpui",
    "zlyph-tui",
]
resolver = "2"

[workspace.package]
...
repository = "https://github.com/douglance/zlyph"
```

**After**:
```toml
[workspace]
members = [
    "zrd-core",
    "zrd-gpui",
    "zrd-tui",
]
resolver = "2"

[workspace.package]
...
repository = "https://github.com/douglance/zrd"
```

**Verification**:
```bash
grep -E "zlyph|zrd" /Users/douglance/Developer/lv/dright/Cargo.toml
# Should only show "zrd" references
```

**Rollback**:
Revert file changes with `git checkout -- Cargo.toml`

---

### Phase 3: Update Package Cargo.toml Files (estimated: 5min)

**Goal**: Update package names, binary names, and dependency paths in each package's Cargo.toml.

**Pre-conditions**:
- Phases 1-2 complete

**Steps**:

#### 3.1: Update `/Users/douglance/Developer/lv/dright/zrd-core/Cargo.toml`
1. [ ] Change package name: `zlyph-core` -> `zrd-core`
2. [ ] Update description to reference "Zrd" instead of "Zlyph"

**Before**:
```toml
[package]
name = "zlyph-core"
...
description = "Core editing logic for Zlyph text editor (platform-agnostic)"
```

**After**:
```toml
[package]
name = "zrd-core"
...
description = "Core editing logic for Zrd text editor (platform-agnostic)"
```

#### 3.2: Update `/Users/douglance/Developer/lv/dright/zrd-tui/Cargo.toml`
1. [ ] Change package name: `zlyph-tui` -> `zrd-tui`
2. [ ] Change binary name: `zlyph` -> `zrd`
3. [ ] Update dependency path: `../zlyph-core` -> `../zrd-core`
4. [ ] Change dependency name: `zlyph-core` -> `zrd-core`

**Before**:
```toml
[package]
name = "zlyph-tui"
...

[[bin]]
name = "zlyph"
path = "src/main.rs"

[dependencies]
zlyph-core = { path = "../zlyph-core" }
```

**After**:
```toml
[package]
name = "zrd-tui"
...

[[bin]]
name = "zrd"
path = "src/main.rs"

[dependencies]
zrd-core = { path = "../zrd-core" }
```

#### 3.3: Update `/Users/douglance/Developer/lv/dright/zrd-gpui/Cargo.toml`
1. [ ] Change package name: `zlyph-gpui` -> `zrd-gpui`
2. [ ] Change binary name: `zlyph-gui` -> `zrd-gui`
3. [ ] Update dependency path: `../zlyph-core` -> `../zrd-core`
4. [ ] Change dependency name: `zlyph-core` -> `zrd-core`

**Before**:
```toml
[package]
name = "zlyph-gpui"
...

[[bin]]
name = "zlyph-gui"
path = "src/main.rs"

[dependencies]
zlyph-core = { path = "../zlyph-core" }
```

**After**:
```toml
[package]
name = "zrd-gpui"
...

[[bin]]
name = "zrd-gui"
path = "src/main.rs"

[dependencies]
zrd-core = { path = "../zrd-core" }
```

**Verification**:
```bash
cargo check 2>&1 | head -20
# Should start resolving dependencies without "not found" errors for packages
```

**Rollback**:
```bash
git checkout -- zrd-core/Cargo.toml zrd-tui/Cargo.toml zrd-gpui/Cargo.toml
```

---

### Phase 4: Update Rust Source Files (estimated: 5min)

**Goal**: Update all `use zlyph_core` imports to `use zrd_core`.

**Pre-conditions**:
- Phases 1-3 complete

**Files to update**:

#### 4.1: `/Users/douglance/Developer/lv/dright/zrd-tui/src/main.rs`
1. [ ] Line 19: `use zlyph_core::{EditorAction, EditorEngine};` -> `use zrd_core::{EditorAction, EditorEngine};`
2. [ ] Line 535: Update comment mentioning `zlyph-gui`
3. [ ] Lines 561, 568, 569: Update references to `zlyph-gui` binary -> `zrd-gui`

#### 4.2: `/Users/douglance/Developer/lv/dright/zrd-gpui/src/main.rs`
1. [ ] Line 10: `use zlyph_core::EditorEngine;` -> `use zrd_core::EditorEngine;`

#### 4.3: `/Users/douglance/Developer/lv/dright/zrd-gpui/src/editor.rs`
1. [ ] Line 7: `use zlyph_core::{EditorAction, EditorEngine};` -> `use zrd_core::{EditorAction, EditorEngine};`
2. [ ] Line 116: `zlyph_core::BufferPosition` -> `zrd_core::BufferPosition`
3. [ ] Line 124: `zlyph_core::BufferPosition` -> `zrd_core::BufferPosition`

#### 4.4: `/Users/douglance/Developer/lv/dright/zrd-gpui/src/editor_old.rs`
1. [ ] Line 7: `use zlyph_core::{EditorAction, EditorEngine};` -> `use zrd_core::{EditorAction, EditorEngine};`

#### 4.5: `/Users/douglance/Developer/lv/dright/zrd-core/tests/engine_tests.rs`
1. [ ] Line 1: `use zlyph_core::{BufferPosition, EditorAction, EditorEngine};` -> `use zrd_core::{BufferPosition, EditorAction, EditorEngine};`

**Verification**:
```bash
cargo check
# Should succeed with no errors
```

**Rollback**:
```bash
git checkout -- zrd-tui/src/main.rs zrd-gpui/src/main.rs zrd-gpui/src/editor.rs zrd-gpui/src/editor_old.rs zrd-core/tests/engine_tests.rs
```

---

### Phase 5: Update Config Path (estimated: 2min)

**Goal**: Update the default config directory from `~/.config/zlyph/` to `~/.config/zrd/`.

**Pre-conditions**:
- Phase 4 complete

**Steps**:
1. [ ] Update `/Users/douglance/Developer/lv/dright/zrd-core/src/engine.rs` line 700:
   - `.join("zlyph")` -> `.join("zrd")`

**Before**:
```rust
PathBuf::from(home)
    .join(".config")
    .join("zlyph")
    .join("default.txt")
```

**After**:
```rust
PathBuf::from(home)
    .join(".config")
    .join("zrd")
    .join("default.txt")
```

**Verification**:
```bash
cargo build -p zrd-core
grep -n "zrd" zrd-core/src/engine.rs | grep config
# Should show the updated config path
```

**Rollback**:
```bash
git checkout -- zrd-core/src/engine.rs
```

---

### Phase 6: Update GitHub Workflow (estimated: 3min)

**Goal**: Update release workflow with new binary and asset names.

**Pre-conditions**:
- Phase 5 complete

**Steps**:
1. [ ] Update `/Users/douglance/Developer/lv/dright/.github/workflows/release.yml`:

| Line | Old Value | New Value |
|------|-----------|-----------|
| 35 | `artifact_name: zlyph` | `artifact_name: zrd` |
| 36 | `asset_name: zlyph-linux-x86_64` | `asset_name: zrd-linux-x86_64` |
| 39 | `artifact_name: zlyph` | `artifact_name: zrd` |
| 40 | `asset_name: zlyph-macos-x86_64` | `asset_name: zrd-macos-x86_64` |
| 43 | `artifact_name: zlyph` | `artifact_name: zrd` |
| 44 | `asset_name: zlyph-macos-aarch64` | `asset_name: zrd-macos-aarch64` |
| 47 | `artifact_name: zlyph.exe` | `artifact_name: zrd.exe` |
| 48 | `asset_name: zlyph-windows-x86_64.exe` | `asset_name: zrd-windows-x86_64.exe` |

**Verification**:
```bash
grep -c "zrd" .github/workflows/release.yml
# Should show 8 (all artifact and asset names)
grep -c "zlyph" .github/workflows/release.yml
# Should show 0
```

**Rollback**:
```bash
git checkout -- .github/workflows/release.yml
```

---

### Phase 7: Update Documentation (estimated: 10min)

**Goal**: Update all documentation to reference the new name.

**Pre-conditions**:
- Phase 6 complete

**Files to update** (all instances of `zlyph` -> `zrd`, `Zlyph` -> `Zrd`):

1. [ ] `/Users/douglance/Developer/lv/dright/README.md`
2. [ ] `/Users/douglance/Developer/lv/dright/FEATURES.md`
3. [ ] `/Users/douglance/Developer/lv/dright/STATUS.md`
4. [ ] `/Users/douglance/Developer/lv/dright/CLAUDE.md`
5. [ ] `/Users/douglance/Developer/lv/dright/REFACTOR_PLAN.md`
6. [ ] `/Users/douglance/Developer/lv/dright/TERMINAL_CONFIG.md`
7. [ ] `/Users/douglance/Developer/lv/dright/TEST_SELECTIONS.md`
8. [ ] `/Users/douglance/Developer/lv/dright/KEYBOARD_DEBUG.md`
9. [ ] `/Users/douglance/Developer/lv/dright/zrd-gpui/FEATURES.md`
10. [ ] `/Users/douglance/Developer/lv/dright/zrd-gpui/USAGE.md`
11. [ ] `/Users/douglance/Developer/lv/dright/zrd-gpui/TEST_LIVE_RELOAD.md`
12. [ ] `/Users/douglance/Developer/lv/dright/docs/plans/tui-mouse-support-plan.md`

**Key replacements**:
- `zlyph` -> `zrd` (command names, package names)
- `zlyph-core` -> `zrd-core`
- `zlyph-tui` -> `zrd-tui`
- `zlyph-gpui` -> `zrd-gpui`
- `zlyph-gui` -> `zrd-gui`
- `Zlyph` -> `Zrd` (capitalized references)
- `~/.config/zlyph/` -> `~/.config/zrd/`
- `github.com/douglance/zlyph` -> `github.com/douglance/zrd`

**Verification**:
```bash
grep -r "zlyph" --include="*.md" .
# Should return no results
```

**Rollback**:
```bash
git checkout -- *.md zrd-gpui/*.md docs/plans/*.md
```

---

### Phase 8: Build and Test (estimated: 5min)

**Goal**: Verify the rename is complete and everything works.

**Pre-conditions**:
- All previous phases complete

**Steps**:
1. [ ] Delete Cargo.lock to force regeneration
2. [ ] Run `cargo build` to verify workspace builds
3. [ ] Run `cargo test -p zrd-core` to verify tests pass
4. [ ] Run `cargo clippy` to check for any issues
5. [ ] Final grep to ensure no `zlyph` references remain in code

**Commands**:
```bash
cd /Users/douglance/Developer/lv/dright

# Remove old lock file
rm Cargo.lock

# Build everything
cargo build

# Run core tests
cargo test -p zrd-core

# Check for any remaining references
grep -r "zlyph" --include="*.rs" --include="*.toml" --include="*.yml" .
# Should return nothing

# Full verification
cargo clippy
```

**Verification**:
All commands complete successfully with no errors.

**Rollback**:
Full rollback requires reverting all phases in reverse order, or:
```bash
git checkout -- .
git clean -fd
```

---

## Risk Mitigation

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Missed reference causes build failure | Medium | Low | Comprehensive grep verification after each phase |
| Git history fragmentation | Low | Low | Use `git mv` for directory renames to preserve history |
| Existing user config not migrated | Medium | Medium | Document in release notes; config path is new anyway |
| Cargo.lock conflicts | Low | Low | Delete and regenerate after rename |

## Dependencies

- Git (for `git mv` to preserve history)
- Rust toolchain (for build verification)

## Out of Scope

- Migrating existing user data from `~/.config/zlyph/` to `~/.config/zrd/`
- Updating the GitHub repository name (requires manual GitHub settings change)
- Updating any external references (crates.io, blog posts, etc.)
- Renaming the local directory from `dright` to something else

## Summary of All Files Modified

### Directories Renamed
- `zlyph-core/` -> `zrd-core/`
- `zlyph-tui/` -> `zrd-tui/`
- `zlyph-gpui/` -> `zrd-gpui/`

### Configuration Files (4)
- `/Users/douglance/Developer/lv/dright/Cargo.toml`
- `/Users/douglance/Developer/lv/dright/zrd-core/Cargo.toml`
- `/Users/douglance/Developer/lv/dright/zrd-tui/Cargo.toml`
- `/Users/douglance/Developer/lv/dright/zrd-gpui/Cargo.toml`

### Source Files (5)
- `/Users/douglance/Developer/lv/dright/zrd-tui/src/main.rs`
- `/Users/douglance/Developer/lv/dright/zrd-gpui/src/main.rs`
- `/Users/douglance/Developer/lv/dright/zrd-gpui/src/editor.rs`
- `/Users/douglance/Developer/lv/dright/zrd-gpui/src/editor_old.rs`
- `/Users/douglance/Developer/lv/dright/zrd-core/src/engine.rs`

### Test Files (1)
- `/Users/douglance/Developer/lv/dright/zrd-core/tests/engine_tests.rs`

### CI/CD Files (1)
- `/Users/douglance/Developer/lv/dright/.github/workflows/release.yml`

### Documentation Files (12)
- `/Users/douglance/Developer/lv/dright/README.md`
- `/Users/douglance/Developer/lv/dright/FEATURES.md`
- `/Users/douglance/Developer/lv/dright/STATUS.md`
- `/Users/douglance/Developer/lv/dright/CLAUDE.md`
- `/Users/douglance/Developer/lv/dright/REFACTOR_PLAN.md`
- `/Users/douglance/Developer/lv/dright/TERMINAL_CONFIG.md`
- `/Users/douglance/Developer/lv/dright/TEST_SELECTIONS.md`
- `/Users/douglance/Developer/lv/dright/KEYBOARD_DEBUG.md`
- `/Users/douglance/Developer/lv/dright/zrd-gpui/FEATURES.md`
- `/Users/douglance/Developer/lv/dright/zrd-gpui/USAGE.md`
- `/Users/douglance/Developer/lv/dright/zrd-gpui/TEST_LIVE_RELOAD.md`
- `/Users/douglance/Developer/lv/dright/docs/plans/tui-mouse-support-plan.md`

### Auto-regenerated (1)
- `/Users/douglance/Developer/lv/dright/Cargo.lock` (delete and let cargo regenerate)

**Total: 3 directories + 23 files**
