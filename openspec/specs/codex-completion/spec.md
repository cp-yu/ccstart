# codex-completion Specification

## Purpose
此规约记录变更 add-codex-switch 引入的行为，请在后续同步或归档前补全正式 Purpose。
## Requirements
### Requirement: Codex channel dynamic completion
系统 SHALL 为 `ccstart codex <channel>` 的 channel 参数提供 shell 动态自动补全，候选值从数据库 `app_type='codex'` 的 provider 名称实时查询。

#### Scenario: Completion returns all codex channels
- **GIVEN** 数据库中存在 `app_type='codex'` 的 provider（如 free、packyapi、packycode）
- **WHEN** shell 补全请求 `ccstart codex` 后的候选
- **THEN** 返回所有 codex provider 名称作为补全候选

#### Scenario: Completion filters by prefix
- **GIVEN** 数据库中存在 codex provider 名称 `free`、`packyapi`、`packycode`
- **WHEN** shell 补全请求前缀为 `pa` 的候选
- **THEN** 仅返回 `packyapi` 和 `packycode`

#### Scenario: Database unavailable during completion
- **GIVEN** cc-switch 数据库文件不存在或不可读
- **WHEN** shell 补全请求候选
- **THEN** 返回空列表（不报错、不崩溃）
