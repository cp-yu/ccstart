# Spec: codex-profile-cache

## Purpose

约束 codex profile 文件的 Read-Through Cache 行为：生成、更新、命名和清理。

## ADDED Requirements

### Requirement: Profile 文件命名
The system SHALL 使用 `ccstart-<sha256(channel_name)[..8]>.config.toml` 作为 profile 文件名，存放于 `~/.codex/` 目录。

#### Scenario: 中文渠道名生成合法 profile 文件名
- **GIVEN** 数据库中存在 codex provider name 为 "小丑"
- **WHEN** 系统计算其 profile 文件名
- **THEN** 生成 `ccstart-<sha256("小丑") 前8位hex>.config.toml`，仅含 `[a-z0-9-.]` 字符

#### Scenario: ASCII 渠道名同样使用 hash 命名
- **GIVEN** 数据库中存在 codex provider name 为 "packyapi"
- **WHEN** 系统计算其 profile 文件名
- **THEN** 生成 `ccstart-<sha256("packyapi") 前8位hex>.config.toml`

### Requirement: Read-Through Cache 写入
The system SHALL 在 codex 启动前确保 profile 文件内容与数据库一致，采用内容哈希比较决定是否写入。

#### Scenario: 缓存不存在时创建
- **GIVEN** profile 文件尚不存在
- **WHEN** 系统准备启动 codex
- **THEN** 将 `settings_config.config` 原始 TOML 写入 profile 文件并返回 Created 状态

#### Scenario: 内容未变时复用
- **GIVEN** profile 文件已存在且 SHA256 与数据库 TOML 一致
- **WHEN** 系统准备启动 codex
- **THEN** 直接复用现有文件并返回 Unchanged 状态

#### Scenario: 内容变化时原子更新
- **GIVEN** profile 文件已存在但 SHA256 与数据库 TOML 不同
- **WHEN** 系统准备启动 codex
- **THEN** 原子写入新内容并返回 Updated 状态

### Requirement: 过期 profile 清理
The system SHALL 在 `ccstart update` 执行时移除不再对应任何 codex provider 的 `ccstart-*` 前缀 profile 文件。

#### Scenario: Update 清理孤儿 profile
- **GIVEN** `~/.codex/` 下存在 `ccstart-abcd1234.config.toml` 但无对应 provider
- **WHEN** 用户执行 `ccstart update`
- **THEN** 该文件被删除并报告
