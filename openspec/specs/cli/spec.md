# Spec: cli

## Purpose

定义 ccstart 的命令行入口、子命令行为和动态补全接口。

## Requirements

### Requirement: Command dispatch
The system SHALL parse ccstart arguments and dispatch each supported command to its owning handler.

#### Scenario: Named configuration dispatches to run
- **GIVEN** a user provides a configuration name without an explicit subcommand
- **WHEN** the CLI parses the invocation
- **THEN** it calls the run handler with the configuration name and trailing arguments

### Requirement: Completion candidates
The system SHALL expose provider names as dynamic completion candidates when the database is available.

#### Scenario: Completion filters by prefix
- **GIVEN** provider names are available from the database
- **WHEN** shell completion requests candidates for a partial prefix
- **THEN** matching provider names are returned as completion candidates
