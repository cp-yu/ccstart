## Why

<!-- routing: Design Summary found in conversation; detail score 5/5; single subsystem; proceeding directly -->

ccstart 当前仅支持 Claude Code 的配置切换。cc-switch 数据库中已有多个 `app_type='codex'` 的 provider，但 ccstart 无法使用它们。需要新增 codex 渠道切换能力，通过运行时注入（`OPENAI_API_KEY` 环境变量 + `-c` 参数覆盖）实现非破坏性启动，不改动 `~/.codex/` 任何文件。

## What Changes

- 新增 `ccstart codex <channel> [args...]` 子命令，使用指定渠道启动 codex
- 新增 `ccstart codex list` 子命令，列出所有可用 codex 渠道
- 新增 codex 渠道名称的 shell 动态自动补全
- `ProviderDao` 查询方法泛化，支持按 `app_type` 参数查询（现有 claude 行为不变）
- 新增 `toml` crate 依赖，用于解析 codex 的 config TOML 字符串

## Capabilities

### New Capabilities

- `codex-dispatch`: codex 子命令分发、渠道启动和参数透传
- `codex-completion`: codex 渠道名称的 shell 动态自动补全

### Modified Capabilities

- `provider`: 查询方法从硬编码 `app_type='claude'` 泛化为接受 `app_type` 参数

## Impact

- `Cargo.toml` — 新增 `toml` 依赖
- `src/main.rs` — 新增 `Commands::Codex` 枚举变体、`codex_channel_completer` 函数、分发逻辑
- `src/commands/mod.rs` — 声明 `pub mod codex`
- `src/commands/codex.rs` — 新文件，codex run + list 实现
- `src/db/provider.rs` — `list_all`/`get_by_name`/`list_names` 接受 `app_type` 参数
- 不影响：`cache.rs`、`diff.rs`、`run.rs`、`update.rs`、`list.rs`、`encoding.rs`
