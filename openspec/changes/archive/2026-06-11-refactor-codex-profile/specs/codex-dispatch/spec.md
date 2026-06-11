# Spec: codex-dispatch

## Purpose

约束 codex 渠道启动的命令组装和进程执行行为。

## MODIFIED Requirements

### Requirement: Codex channel dispatch
系统 SHALL 接受 `ccstart codex <channel> [args...]` 调用，从 cc-switch 数据库查询 `app_type='codex'` 的 provider，通过 `-p` profile 机制启动 codex 进程。

#### Scenario: 使用 profile 启动 codex
- **GIVEN** 数据库中存在 `app_type='codex'` 且 `name='packyapi'` 的 provider
- **WHEN** 用户执行 `ccstart codex packyapi`
- **THEN** 系统以 `OPENAI_API_KEY=<auth值>` 环境变量 + `codex -p ccstart-<hash> ` 启动 codex 进程

#### Scenario: 透传参数
- **GIVEN** 数据库中存在有效的 codex provider
- **WHEN** 用户执行 `ccstart codex packyapi "help me" --no-alt-screen`
- **THEN** 参数透传为 `codex -p ccstart-<hash> "help me" --no-alt-screen`

#### Scenario: 渠道不存在且拼音无匹配
- **GIVEN** 数据库中不存在精确匹配或拼音匹配的 codex provider
- **WHEN** 用户执行 `ccstart codex nonexistent`
- **THEN** 系统输出错误信息并列出可用的 codex 渠道名称

### Requirement: Codex config TOML 直传
系统 SHALL 将 `settings_config.config` 的完整 TOML 内容直接写入 profile 文件，不做字段提取或转换。

#### Scenario: 完整 TOML 写入 profile
- **GIVEN** provider 的 `settings_config.config` 包含 model、model_providers、mcp_servers 等多段 TOML
- **WHEN** 系统生成 profile 文件
- **THEN** profile 文件内容与 `settings_config.config` 字符串完全一致
