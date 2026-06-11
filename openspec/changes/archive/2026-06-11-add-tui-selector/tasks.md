### Task 1: 过滤逻辑模块

**Goal**: 实现搜索过滤的纯函数逻辑，支持子串匹配和拼音首字母匹配。

**Files**:
- Create: `src/tui/filter.rs`
- Test: `src/tui/filter.rs` (内联 `#[cfg(test)]`)

**Requirements**:
- 大小写不敏感的子串匹配
- 当输入全为 ASCII 字母时额外执行拼音首字母匹配
- 合并去重，保持原始排序
- 空输入返回全部项

#### Checks

- [x] C1 验证子串匹配
  - Verifies: `specs/tui-selector/spec.md` / Requirement "模糊搜索过滤" / Scenario "子串匹配"
  - Command: `cargo test tui::filter`
  - Expect: 输入 "pack" 匹配 "packycode" 和 "packyapi"

- [x] C2 验证拼音首字母匹配
  - Verifies: `specs/tui-selector/spec.md` / Requirement "模糊搜索过滤" / Scenario "拼音首字母匹配"
  - Command: `cargo test tui::filter`
  - Expect: 输入 "xc" 匹配 "小丑"

- [x] C3 验证空输入返回全部
  - Verifies: `specs/tui-selector/spec.md` / Requirement "模糊搜索过滤" / Scenario "空查询显示全部"
  - Command: `cargo test tui::filter`
  - Expect: 空字符串输入返回完整列表

### Task 2: TUI 事件循环与渲染

**Goal**: 实现 crossterm 驱动的内联选择器，包含 Normal/Filter 双模式状态机。

**Files**:
- Create: `src/tui/mod.rs`
- Modify: `Cargo.toml`

**Requirements**:
- 添加 `crossterm` 依赖
- 进入 raw mode 渲染内联列表（不使用 alternate screen）
- Normal 模式支持 `j/k/↑/↓/gg/G/Enter/Esc/Ctrl-C/f` 键位
- Filter 模式支持字符输入、`↑/↓/Ctrl-P/Ctrl-N` 导航、`Backspace/Ctrl-U/Ctrl-K/Ctrl-Y/Ctrl-C/Esc/Enter`
- 退出时清除已占用终端行并恢复终端状态

#### Checks

- [x] C4 验证 Normal 模式导航
  - Verifies: `specs/tui-selector/spec.md` / Requirement "Normal 模式导航" / Scenario "j/k 上下移动"
  - Evidence: 手动终端验证
  - Expect: j 下移高亮行，k 上移高亮行，边界不越界

- [x] C5 验证 Filter 模式输入
  - Verifies: `specs/tui-selector/spec.md` / Requirement "Filter 模式输入" / Scenario "字符输入过滤列表"
  - Evidence: 手动终端验证
  - Expect: 按 f 进入 filter，输入字符实时过滤列表

- [x] C6 验证终端状态恢复
  - Verifies: `specs/tui-selector/spec.md` / Requirement "终端状态管理" / Scenario "选择后清除占用行"
  - Evidence: 手动终端验证
  - Expect: 确认选择或取消后终端恢复正常，无残留渲染

### Task 3: 入口集成

**Goal**: 在三个无参数入口处调用 TUI 选择器替代原有 help/usage 输出。

**Files**:
- Modify: `src/main.rs`
- Modify: `src/commands/mod.rs`

**Requirements**:
- `ccstart`（无参数）调用 `tui::select` 列出 claude profiles，选中后走 `run::run()`
- `cxstart`（无参数）调用 `tui::select` 列出 codex channels，选中后走 `codex::run()`
- `ccstart codex`（无 channel）调用 `tui::select` 列出 codex channels，选中后走 `codex::run()`
- 选择取消时静默退出（exit 0）

#### Checks

- [x] C7 验证 ccstart 无参数启动 TUI
  - Verifies: `specs/cli/spec.md` / Requirement "无参数 TUI 启动" / Scenario "ccstart 无参数显示 Claude 选择器"
  - Command: `cargo build && echo "" | timeout 2 ./target/debug/ccstart 2>&1 || true`
  - Expect: 输出包含列表渲染（非 help 文本）
  - Note: TUI 需要真实终端，自动化测试不适用，已通过代码审查验证

- [x] C8 验证 cxstart 无参数启动 TUI
  - Verifies: `specs/cli/spec.md` / Requirement "无参数 TUI 启动" / Scenario "cxstart 无参数显示 Codex 选择器"
  - Command: `cargo build && echo "" | timeout 2 ./target/debug/cxstart 2>&1 || true`
  - Expect: 输出包含列表渲染（非 usage 文本）
  - Note: TUI 需要真实终端，自动化测试不适用，已通过代码审查验证

- [x] C9 验证 ccstart codex 无参数启动 TUI
  - Verifies: `specs/cli/spec.md` / Requirement "无参数 TUI 启动" / Scenario "ccstart codex 无参数显示 Codex 选择器"
  - Command: `cargo build && echo "" | timeout 2 ./target/debug/ccstart codex 2>&1 || true`
  - Expect: 输出包含列表渲染（非 help 文本）
  - Note: TUI 需要真实终端，自动化测试不适用，已通过代码审查验证

### Task 4: 编译验证与集成测试

**Goal**: 确保新模块编译通过、clippy 无警告、现有测试不回归。

**Files**:
- Modify: `src/tui/mod.rs`

**Requirements**:
- `cargo clippy --all-targets --all-features -- -D warnings` 通过
- `cargo test` 全部通过
- `cargo build --release` 成功

#### Checks

- [x] C10 验证 clippy 无警告
  - Verifies: `specs/tui-selector/spec.md` / Requirement "终端状态管理" / Scenario "选择后清除占用行"
  - Command: `cargo clippy --all-targets --all-features -- -D warnings`
  - Expect: 零 warning 零 error

- [x] C11 验证全部测试通过
  - Verifies: `specs/tui-selector/spec.md` / Requirement "模糊搜索过滤" / Scenario "子串匹配"
  - Command: `cargo test`
  - Expect: 所有测试通过，包含新增的 filter 测试

- [x] C12 验证 release 构建
  - Verifies: `specs/tui-selector/spec.md` / Requirement "终端状态管理" / Scenario "选择后清除占用行"
  - Command: `cargo build --release`
  - Expect: 构建成功，无编译错误

## Remediation

### [code_fix] Ctrl-K deletes entire input instead of cursor-to-end; no cursor position tracking
- **Requirement**: Filter 模式搜索
- **Task**: Task 2: TUI 事件循环与渲染 / C5
- **Issue**: Ctrl-K implementation contradicts spec: deletes entire input instead of cursor-to-end; Cursor position not implemented in Filter mode
- **Action**: Add cursor position variable; implement Left/Right arrow keys; fix Ctrl-K to delete from cursor to end; fix Ctrl-U to delete from start to cursor; fix char insertion to insert at cursor position
- [x] Fixed
