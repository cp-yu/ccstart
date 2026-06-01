## MODIFIED Requirements

### Requirement: Claude provider queries
系统 SHALL 在 `ProviderDao` 的 `list_all`、`get_by_name`、`list_names` 方法中接受 `app_type` 参数，返回指定类型的 provider 并保持配置的排序。

#### Scenario: Query claude providers (backward compatible)
- **GIVEN** 数据库中存在 `app_type='claude'` 和 `app_type='codex'` 的 provider
- **WHEN** 调用 `list_names("claude")`
- **THEN** 仅返回 `app_type='claude'` 的 provider 名称，按 `sort_index` 和 `name` 排序

#### Scenario: Query codex providers
- **GIVEN** 数据库中存在 `app_type='codex'` 的 provider
- **WHEN** 调用 `list_names("codex")`
- **THEN** 仅返回 `app_type='codex'` 的 provider 名称，按 `sort_index` 和 `name` 排序

#### Scenario: Get provider by name and app_type
- **GIVEN** 数据库中存在 `app_type='codex'` 且 `name='free'` 的 provider
- **WHEN** 调用 `get_by_name("codex", "free")`
- **THEN** 返回该 provider 的完整数据（id、name、settings_config）
