## Why

<!-- Smart routing: Design Summary found in explore conversation. Detail score 5/5, single subsystem. -->

当前 `ccstart`、`cxstart`、`ccstart codex` 无参数时仅显示 help 文本，用户必须记住配置名称才能启动。增加内联 TUI 选择器后，无参数即可交互式浏览和搜索 profile 列表，降低使用门槛。

## What Changes

- 新增 `src/tui/` 模块，提供 fzf 风格的内联选择器（crossterm raw mode，不启用 alternate screen）
- 支持 Normal / Filter 双模式状态机，vim 键位导航
- Filter 模式支持子串匹配 + 拼音首字母匹配（复用现有 `utils::pinyin`）
- `ccstart`（无参数）进入 TUI 选择 claude profile
- `cxstart`（无参数）进入 TUI 选择 codex channel
- `ccstart codex`（无参数）进入 TUI 选择 codex channel
- 新增 `crossterm` 依赖

## Capabilities

### New Capabilities

- `tui-select`: 内联交互式列表选择器，支持 vim 键位导航和模糊搜索过滤

### Modified Capabilities

- `cli`: 三个入口（ccstart/cxstart/ccstart codex）无参数时从显示帮助改为进入 TUI 选择界面

## Impact

- 新增依赖：`crossterm` crate
- 修改文件：`src/main.rs`（三处无参数分支）、`Cargo.toml`
- 新增文件：`src/tui/mod.rs`、`src/tui/filter.rs`
- 二进制体积预计增加 ~200-300KB
