# Implementation Plan: ccstart - Claude Settings 配置管理工具

**Branch**: `002-ccswitch-rust-claude` | **Date**: 2025-10-14 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/002-ccswitch-rust-claude/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

ccstart 是一个 Rust 编写的 CLI 工具，用于管理和快速切换 Claude CLI 的配置文件。它将 ccswitch 的混合配置文件（~/.cc-switch/config.json）拆分成独立的命名配置，并提供便捷的命令行接口和自动补全功能。核心价值是通过 `ccstart <name>` 快速启动 Claude 并应用指定配置，支持配置初始化、更新和列表查询。

## Technical Context

**Language/Version**: Rust 2024 edition
**Primary Dependencies**: NEEDS CLARIFICATION (需研究 CLI 框架、JSON 处理、Shell 补全生成)
**Storage**: 文件系统（~/.cc-switch/config.json, ~/.cc-switch/separated/）
**Testing**: cargo test（Rust 标准测试框架）
**Target Platform**: Linux/macOS（基于 Unix-like 文件系统路径）
**Project Type**: single（CLI 工具）
**Performance Goals**: 配置切换 <1秒，配置初始化 <5秒，补全响应 <0.5秒
**Constraints**: 无并发控制机制，假设单用户操作
**Scale/Scope**: 支持至少 50 个配置项，单个配置文件 <10MB

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### KISS原则 (Keep It Simple, Stupid)
- [x] 解决方案是否采用最简单的方法？✅ 采用直接的文件读写、JSON 解析和命令行封装，无复杂抽象
- [x] 是否避免了不必要的抽象和过度设计？✅ 功能直接映射到子命令，无多余的架构层次
- [x] 如果引入复杂度，是否有充分理由说明为什么简单方案不可行？✅ 无不必要的复杂度引入

### 轮子复用原则 (Don't Reinvent the Wheel)
- [x] 是否调研了现有的开源解决方案（库、框架、工具）？✅ **Phase 0 已完成调研**：
  - ✅ Rust CLI 框架：选择 clap v4+（Rust 标准，15.6k stars，完整功能支持）
  - ✅ JSON 处理库：选择 serde_json（68,519 个 crates 使用，事实标准）
  - ✅ Shell 补全生成：选择 clap_complete（官方插件，动态补全支持）
  - ✅ 配置管理工具：调研了 7+ 工具（cctx, CCProfileSwitch 等），无现成方案满足需求
- [x] 如果选择自己实现，是否记录了理由？✅ 配置管理逻辑是特定于 ccswitch 的 config.json 格式，需要自定义实现；但会使用成熟库处理 CLI 解析、JSON 处理等基础功能。详见 research.md
- [x] 使用的第三方库是否满足：活跃维护、良好文档、社区支持、许可证兼容？✅ **Phase 0 已验证**：
  - clap：MIT/Apache 2.0，461,000+ 项目使用，活跃维护
  - serde_json：MIT/Apache 2.0，2,200 万次/月下载，官方维护
  - clap_complete：Apache 2.0/MIT，官方插件
  - percent-encoding：MIT/Apache 2.0，800M+ 下载，官方 Rust URL 工作组维护

### Linus三问原则
- [x] **What**: 功能目标是否清晰明确？✅ 管理和快速切换 Claude CLI 配置文件
- [x] **Why**: 是否明确说明了为什么需要这个功能和为什么这样实现？✅ 简化多配置环境下的使用，ccswitch 的配置管理增强工具
- [x] **Better**: 是否考虑了其他实现方案，并说明为什么当前方案是合理选择？✅ **Phase 0 已评估**：调研了 7+ 现有工具，确认无更好的现成方案。可大量借鉴 cctx 和 CCProfileSwitch 的设计模式。详见 research.md

## Project Structure

### Documentation (this feature)

```
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```
src/
├── main.rs           # CLI 入口点，命令行参数解析
├── config/           # 配置相关模块
│   ├── mod.rs        # 配置模块导出
│   ├── parser.rs     # config.json 解析逻辑
│   └── manager.rs    # 配置文件管理（读写、分离、更新）
├── commands/         # 子命令实现
│   ├── mod.rs
│   ├── init.rs       # ccstart init 实现
│   ├── update.rs     # ccstart update 实现
│   ├── list.rs       # ccstart list 实现
│   ├── run.rs        # ccstart <name> 实现（调用 claude）
│   └── completions.rs # ccstart completions 实现
├── utils/            # 工具函数
│   ├── mod.rs
│   ├── fs.rs         # 文件系统操作（路径处理、目录创建）
│   └── encoding.rs   # URL 编码/解码（配置名称处理）
└── error.rs          # 错误类型定义

tests/
├── integration/      # 集成测试
│   ├── init_test.rs
│   ├── update_test.rs
│   ├── list_test.rs
│   └── run_test.rs
├── unit/             # 单元测试
│   ├── config_test.rs
│   ├── encoding_test.rs
│   └── parser_test.rs
└── fixtures/         # 测试用例数据
    └── sample_config.json
```

**Structure Decision**:
采用标准的 Rust CLI 项目结构（Single project）。主要理由：
- ccstart 是一个独立的命令行工具，无前后端分离需求
- 模块按功能职责划分：config（配置解析）、commands（子命令）、utils（工具函数）
- 测试分为 integration（端到端测试）和 unit（单元测试）两层
- 符合 Rust 社区约定，易于理解和维护

## Complexity Tracking

*Fill ONLY if Constitution Check has violations that must be justified*

根据KISS原则，任何复杂度的引入都必须在此处明确说明理由：

| 复杂度类型 | 为什么需要 | 为什么更简单的方案不可行 |
|-----------|------------|-------------------------|
| [例如：自定义解析器] | [具体需求] | [为什么现有库不满足] |
| [例如：多层抽象] | [当前需求] | [为什么直接实现不够] |
