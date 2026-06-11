# Spec: codex-completion

## Purpose

约束 codex 渠道 shell 动态补全的行为，包括拼音前缀匹配。

## MODIFIED Requirements

### Requirement: Codex channel dynamic completion
系统 SHALL 为 `ccstart codex <channel>` 和 `cxstart <channel>` 的 channel 参数提供 shell 动态自动补全，候选值从数据库 `app_type='codex'` 的 provider 名称实时查询，并支持拼音首字母前缀过滤。

#### Scenario: 补全返回所有 codex 渠道
- **GIVEN** 数据库中存在 `app_type='codex'` 的 provider（如 packyapi、小丑、xl）
- **WHEN** shell 补全请求 channel 候选且前缀为空
- **THEN** 返回所有 codex provider 名称

#### Scenario: ASCII 前缀过滤
- **GIVEN** 数据库中存在 codex provider 名称 `packyapi`、`packycode`、`小丑`
- **WHEN** shell 补全请求前缀为 `pa` 的候选
- **THEN** 返回 `packyapi` 和 `packycode`

#### Scenario: 拼音首字母前缀过滤
- **GIVEN** 数据库中存在 codex provider "小丑"（拼音首字母 xc）
- **WHEN** shell 补全请求前缀为 "xc" 的候选
- **THEN** "小丑" 出现在候选列表中

#### Scenario: 数据库不可用时返回空
- **GIVEN** cc-switch 数据库文件不存在或不可读
- **WHEN** shell 补全请求候选
- **THEN** 返回空列表
