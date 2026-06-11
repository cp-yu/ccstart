# Spec: pinyin-resolve

## Purpose

约束中文渠道名称的拼音首字母模糊匹配行为。

## ADDED Requirements

### Requirement: 拼音首字母匹配
The system SHALL 在精确匹配失败且输入全为 ASCII 字母时，尝试用拼音首字母匹配 codex 渠道名。

#### Scenario: 唯一首字母命中
- **GIVEN** 数据库中存在 codex provider "小丑"（拼音首字母 xc）且无其他 xc 开头的渠道
- **WHEN** 用户输入 channel 为 "xc"
- **THEN** 解析为 "小丑"

#### Scenario: 多个首字母命中
- **GIVEN** 数据库中存在 "小丑"（xc）和 "新城"（xc）
- **WHEN** 用户输入 channel 为 "xc"
- **THEN** 列出所有匹配候选并退出，不启动 codex

#### Scenario: 无命中
- **GIVEN** 数据库中无拼音首字母匹配 "zz" 的 codex provider
- **WHEN** 用户输入 channel 为 "zz"
- **THEN** 报错并列出所有可用渠道

#### Scenario: 非全 ASCII 输入不触发拼音匹配
- **GIVEN** 用户输入 channel 包含非 ASCII 字符（如 "小丑"）
- **WHEN** 精确匹配失败
- **THEN** 直接报错，不尝试拼音匹配

### Requirement: 补全中支持拼音前缀
The system SHALL 在 shell 动态补全中同时返回拼音首字母匹配的候选。

#### Scenario: 补全返回拼音匹配候选
- **GIVEN** 数据库中存在 codex provider "小丑"
- **WHEN** shell 补全请求前缀为 "xc" 的候选
- **THEN** "小丑" 出现在候选列表中
