# Feature: Auto-Update with Field-Level Diff

## Overview
Enhance `ccstart <name>` to display field-level change feedback when cache is updated, showing exactly which configuration fields changed (e.g., "api_key, model").

## Tasks

### Task 1: Create Diff Module
- **ID**: task-1
- **Description**: Create `src/config/diff.rs` module with types and functions for comparing JSON values and identifying changed fields at the top level.
- **File Scope**: `src/config/diff.rs` (new), `src/config/mod.rs`
- **Dependencies**: None
- **Test Command**: `cargo test diff --lib -- --nocapture`
- **Test Focus**:
  - Unchanged content returns empty diff
  - Single field change detection
  - Multiple field changes detection
  - Nested object change shows parent key only
  - Field addition and removal detection

**Deliverables**:
```rust
// src/config/diff.rs
pub enum CacheResult {
    Unchanged(PathBuf),
    Created(PathBuf),
    Updated { path: PathBuf, changed_fields: Vec<String> },
}

pub fn diff_json_fields(old: &Value, new: &Value) -> Vec<String>
```

### Task 2: Refactor CacheManager Return Type
- **ID**: task-2
- **Description**: Modify `CacheManager::ensure_cached()` to return `CacheResult` instead of `PathBuf`, integrating the diff module for field-level comparison.
- **File Scope**: `src/config/cache.rs`, `src/config/mod.rs`
- **Dependencies**: task-1
- **Test Command**: `cargo test cache --lib`
- **Test Focus**:
  - `ensure_cached` returns `Unchanged` when hash matches
  - `ensure_cached` returns `Created` when cache file doesn't exist
  - `ensure_cached` returns `Updated` with changed fields when content differs

**Changes Required**:
- Import `diff` module in `cache.rs`
- Change return type of `ensure_cached` from `AppResult<PathBuf>` to `AppResult<CacheResult>`
- Read existing cache content and compare using `diff_json_fields` before writing
- Export `CacheResult` from `src/config/mod.rs`

### Task 3: Update Run Command Display
- **ID**: task-3
- **Description**: Update `commands::run` to handle `CacheResult` and display appropriate feedback messages based on cache state.
- **File Scope**: `src/commands/run.rs`
- **Dependencies**: task-2
- **Test Command**: `cargo build && cargo run -- test-config 2>&1 | head -5`
- **Test Focus**:
  - No extra output when cache unchanged
  - Shows "[INFO] config created: {name}" for new cache
  - Shows "[INFO] config updated: {name} ({fields} changed)" for updates

**Output Format**:
```
[INFO] 使用配置: /path/to/cache.json
[INFO] 配置已更新: openai (api_key, model 已变更)
```

### Task 4: Update Update Command Feedback
- **ID**: task-4
- **Description**: Modify `commands::update` to use the same `CacheResult` for consistent field-level feedback when force-updating all configs.
- **File Scope**: `src/commands/update.rs`, `src/config/cache.rs`
- **Dependencies**: task-2
- **Test Command**: `cargo build && cargo run -- update 2>&1`
- **Test Focus**:
  - Shows changed fields for each updated config
  - Maintains existing delete notification behavior
  - Summary shows correct counts

**Changes Required**:
- Add `force_write_with_diff()` method to `CacheManager` or modify `force_write()` return type
- Update feedback format to include changed fields

### Task 5: Add Comprehensive Unit Tests
- **ID**: task-5
- **Description**: Add unit tests for the diff module covering edge cases and ensure code coverage meets requirements.
- **File Scope**: `src/config/diff.rs` (test module)
- **Dependencies**: task-1
- **Test Command**: `cargo test --lib -- --nocapture && cargo tarpaulin --out Html --output-dir coverage -p ccstart --lib 2>/dev/null || cargo test --lib`
- **Test Focus**:
  - Empty objects comparison
  - Null value handling
  - Array changes (show field as changed, not element diff)
  - Deep nested changes (show top-level key only)
  - Type changes (string to number, etc.)
  - Unicode field names

## Dependencies

```
task-1 (Diff Module)
    |
    v
task-2 (Refactor Cache) -----> task-5 (Tests)
    |
    +-------+-------+
    |               |
    v               v
task-3 (Run)    task-4 (Update)
```

## Acceptance Criteria
- [ ] `ccstart <name>` shows field-level diff when config changes
- [ ] Output format: `[INFO] 配置已更新: {name} ({field1}, {field2} 已变更)`
- [ ] No output for unchanged configs (only path info)
- [ ] `ccstart update` shows field-level diff for each config
- [ ] All existing tests pass
- [ ] New unit tests cover diff module edge cases
- [ ] Code coverage >= 90% for new diff module
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [ ] `cargo fmt --all -- --check` passes

## Technical Notes
- Use `serde_json::Value` for JSON comparison, avoiding deserialization to specific types
- Only report top-level field changes (nested changes show parent key)
- Maintain backward compatibility: existing cache files work without issues
- Field order in output should be alphabetically sorted for consistency
- Consider caching the old content read during hash comparison to avoid double-read
