## MODIFIED Requirements

### Requirement: Command dispatch
系统 SHALL 解析 ccstart 参数并将每个支持的命令分发到对应处理器。当无参数调用时，SHALL 启动 TUI 选择器而非显示帮助。

#### Scenario: Named configuration dispatches to run
- **GIVEN** a user provides a configuration name without an explicit subcommand
- **WHEN** the CLI parses the invocation
- **THEN** it calls the run handler with the configuration name and trailing arguments

#### Scenario: 无参数启动 TUI 选择 Claude 配置
- **WHEN** 用户运行 `ccstart`（无任何参数）
- **THEN** 启动 TUI 选择器展示所有 Claude 配置，用户选中后使用该配置启动 claude

#### Scenario: cxstart 无参数启动 TUI 选择 Codex 渠道
- **WHEN** 用户运行 `cxstart`（无任何参数）
- **THEN** 启动 TUI 选择器展示所有 Codex 渠道，用户选中后使用该渠道启动 codex

#### Scenario: ccstart codex 无参数启动 TUI 选择 Codex 渠道
- **WHEN** 用户运行 `ccstart codex`（无 channel 参数）
- **THEN** 启动 TUI 选择器展示所有 Codex 渠道，用户选中后使用该渠道启动 codex
