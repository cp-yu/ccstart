## Context

ccstart 当前仅支持 Claude Code 配置切换。cc-switch 数据库中已有 13 个 `app_type='codex'` 的 provider，其 `settings_config` 结构为 `{"auth":{"OPENAI_API_KEY":"..."}, "config":"<TOML字符串>"}`。

实测确认 codex CLI 无 `--settings` 等价参数，但支持：
- `OPENAI_API_KEY` 环境变量注入 auth
- `-c key=value` 覆盖 `config.toml` 中的值（支持嵌套路径如 `model_providers.custom.base_url`）

这意味着可以在不写入任何文件的情况下，通过运行时注入完成渠道切换。

## Goals / Non-Goals

**Goals:**
- 新增 `ccstart codex <channel> [args...]` 子命令，非破坏性启动 codex
- 新增 `ccstart codex list` 列出所有 codex 渠道
- codex 渠道名称支持 shell 动态自动补全
- `ProviderDao` 泛化以支持多 `app_type` 查询

**Non-Goals:**
- 不为 codex 实现缓存/文件物化（无需）
- 不修改 `~/.codex/` 目录中的任何文件
- 不改变现有 claude 路径的任何行为
- 不支持 codex 的 `--profile` 机制（无法切换 auth）
- 不纳入 codex 到 `ccstart list` 或 `ccstart update`

## Decisions

### D1: 运行时注入而非文件物化

**选择**: `OPENAI_API_KEY` env + `-c` 参数覆盖

**替代方案**:
- A) `CODEX_HOME` 重定向：每渠道独立目录，丢失 `~/.codex` 共享态（AGENTS.md/prompts/skills/memories/history）
- B) 原地覆写 `~/.codex/{config.toml,auth.json}`：破坏性、并发不安全
- C) `CODEX_HOME` + 软链共享资产：复杂、Windows 受限

**理由**: 运行时注入是唯一同时满足「非破坏」「保留共享态」「并发安全」的方案。实测 `codex doctor` 验证 `-c` + env 组合完全生效。

### D2: 注入字段为 OPENAI_API_KEY + base_url + model

**选择**: 从 DB 的 TOML 配置中提取 `model_provider`（用于构造 `-c` 路径）、`model`、`base_url`，加上 `auth.OPENAI_API_KEY`。

**理由**: 各渠道 model 不同（gpt-5.5-pro20x / gpt-5.3-codex / gpt-5.2），仅注入 base_url 会导致 model 不匹配。`model_provider` 名称不统一，需动态构造 `-c` 路径。

### D3: 引入 `toml` crate 解析 config 字符串

**选择**: 使用 `toml` crate 正式解析 `settings_config.config` TOML 字符串。

**替代方案**: 逐行字符串匹配提取值。

**理由**: `base_url` 位于嵌套 `[model_providers.<name>]` section 下，字符串匹配对 TOML 嵌套结构脆弱。`toml` crate 是 Rust 生态标准选择，零风险。

### D4: `ProviderDao` 方法接受 `app_type` 参数

**选择**: 为 `list_all`/`get_by_name`/`list_names` 添加 `app_type: &str` 参数。

**替代方案**: 新增 `CodexProviderDao` 独立 DAO。

**理由**: 三个方法逻辑完全相同，仅 WHERE 条件不同。参数化最简洁，避免代码重复。现有调用点传入 `"claude"` 即可保持行为不变。

### D5: `Codex` 作为 clap 子命令，保留直接参数透传

**选择**: `Commands::Codex { channel, args }` 枚举变体，`channel == "list"` 且无额外参数时分发到 list，否则按渠道启动 codex。

**理由**: 与用户期望的 `ccstart codex <channel>` / `ccstart codex list` 形态一致，同时避免 clap 嵌套子命令拦截 `ccstart codex free --help` 这类应透传给 codex 的参数。

## Risks / Trade-offs

- [codex 未来版本改变 `-c` 行为] → 启动前可选验证（当前不做，留为未来增强）
- [`model_provider` 名称在 TOML 中不存在对应 section] → 解析时校验并报错
- [渠道名含 URL 特殊字符如 `https://icoe.pp.ua/`] → shell 补全正常工作（clap 处理），用户需引号包裹
- [`~/.codex/config.toml` 中无 `model_providers.custom` section] → `-c` 覆盖已有 section 的字段，不创建新 section；用户需确保基础 config 中有对应 provider 定义
