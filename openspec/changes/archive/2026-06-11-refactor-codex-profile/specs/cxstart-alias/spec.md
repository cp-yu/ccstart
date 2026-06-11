# Spec: cxstart-alias

## Purpose

约束 `cxstart` 快捷入口的行为：通过 argv[0] 检测隐含 codex 子命令。

## ADDED Requirements

### Requirement: argv[0] 路由
The system SHALL 在进程名为 `cxstart` 时将命令行参数视为 `ccstart codex <channel> [args...]`。

#### Scenario: cxstart 直接启动 codex 渠道
- **GIVEN** 二进制通过 symlink 或副本以 `cxstart` 名称调用
- **WHEN** 用户执行 `cxstart 小丑 "help me"`
- **THEN** 等价于 `ccstart codex 小丑 "help me"`

#### Scenario: cxstart list 列出 codex 渠道
- **GIVEN** 二进制以 `cxstart` 名称调用
- **WHEN** 用户执行 `cxstart list`
- **THEN** 等价于 `ccstart codex list`

#### Scenario: cxstart 无参数显示帮助
- **GIVEN** 二进制以 `cxstart` 名称调用
- **WHEN** 用户执行 `cxstart`（无参数）
- **THEN** 显示 codex 子命令帮助信息

### Requirement: CI Release 产出 cxstart 副本
The system SHALL 在 GitHub Actions release 工作流中产出 `cxstart` 可执行文件副本。

#### Scenario: Release 包含 cxstart
- **GIVEN** release 工作流构建 Linux x86_64 产物
- **WHEN** 构建完成
- **THEN** 产物中包含 `cxstart-linux-x64`（为 `ccstart-linux-x64` 的硬拷贝）
