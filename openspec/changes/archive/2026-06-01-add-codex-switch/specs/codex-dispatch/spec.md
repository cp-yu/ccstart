## ADDED Requirements

### Requirement: Codex channel dispatch
系统 SHALL 接受 `ccstart codex <channel> [args...]` 调用，从 cc-switch 数据库查询 `app_type='codex'` 的 provider，解析其 `settings_config` 并通过运行时注入启动 codex 进程。

#### Scenario: Successful codex launch with channel
- **GIVEN** 数据库中存在 `app_type='codex'` 且 `name='free'` 的 provider
- **WHEN** 用户执行 `ccstart codex free`
- **THEN** 系统以 `OPENAI_API_KEY` 环境变量 + `-c model_providers.<provider>.base_url="<url>"` + `-c model="<model>"` 启动 codex 进程，并透传退出码

#### Scenario: Channel not found
- **GIVEN** 数据库中不存在 `app_type='codex'` 且 `name='nonexistent'` 的 provider
- **WHEN** 用户执行 `ccstart codex nonexistent`
- **THEN** 系统输出错误信息并列出可用的 codex 渠道名称

#### Scenario: Trailing arguments are passed through
- **GIVEN** 数据库中存在有效的 codex provider
- **WHEN** 用户执行 `ccstart codex free "help me" --no-alt-screen`
- **THEN** 系统将 `"help me"` 和 `--no-alt-screen` 作为 codex 进程的参数透传

### Requirement: Codex config TOML parsing
系统 SHALL 从 `settings_config.config` TOML 字符串中提取 `model_provider`、`model` 和 `model_providers.<provider>.base_url` 三个字段。

#### Scenario: Standard config extraction
- **GIVEN** `settings_config.config` 为包含 `model_provider="custom"`、`model="gpt-5.5"` 和 `[model_providers.custom]` section 含 `base_url` 的 TOML 字符串
- **WHEN** 系统解析该 TOML
- **THEN** 提取出 `model_provider="custom"`、`model="gpt-5.5"` 和对应的 `base_url`

#### Scenario: Missing required field
- **GIVEN** `settings_config.config` TOML 字符串中缺少 `base_url` 字段
- **WHEN** 系统解析该 TOML
- **THEN** 系统报错并指出缺失的字段名

### Requirement: Codex list subcommand
系统 SHALL 提供 `ccstart codex list` 子命令，列出所有 `app_type='codex'` 的 provider 名称。

#### Scenario: List codex channels
- **GIVEN** 数据库中存在多个 `app_type='codex'` 的 provider
- **WHEN** 用户执行 `ccstart codex list`
- **THEN** 系统按 `sort_index` 和 `name` 排序输出所有 codex 渠道名称，含空格的名称用双引号包裹

#### Scenario: No codex channels available
- **GIVEN** 数据库中不存在 `app_type='codex'` 的 provider
- **WHEN** 用户执行 `ccstart codex list`
- **THEN** 系统输出提示信息并以非零退出码退出

### Requirement: Non-destructive execution
系统 SHALL 不修改 `~/.codex/` 目录中的任何文件，仅通过环境变量和命令行参数注入渠道配置。

#### Scenario: Home directory unchanged after execution
- **GIVEN** `~/.codex/config.toml` 和 `~/.codex/auth.json` 存在且内容已知
- **WHEN** 用户执行 `ccstart codex <channel>` 并 codex 进程退出
- **THEN** `~/.codex/config.toml` 和 `~/.codex/auth.json` 内容与执行前完全一致
