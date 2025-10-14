# Tasks: ccstart - Claude Settings 配置管理工具

**Input**: Design documents from `/specs/002-ccswitch-rust-claude/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/cli-interface.md

**Tests**: Tests are NOT included as they were not explicitly requested in the feature specification.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions
- **Single project**: `src/`, `tests/` at repository root
- Project type: Rust CLI tool

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic Rust project structure

 - [X] T001 Initialize Rust project with Cargo.toml dependencies (clap, serde, serde_json, anyhow, dirs, percent-encoding, clap_complete)
 - [X] T002 [P] Configure Cargo.toml with edition 2024, metadata, and feature flags
 - [X] T003 [P] Create project directory structure (src/config/, src/commands/, src/utils/)
 - [X] T004 [P] Setup Clippy and rustfmt configuration in .cargo/config.toml

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

 - [X] T005 Create error types module in src/error.rs (ConfigError, IoError, JsonError enums)
 - [X] T006 [P] Implement URL encoding utilities in src/utils/encoding.rs (encode_config_name, decode_config_name with percent-encoding)
 - [X] T007 [P] Implement filesystem utilities in src/utils/fs.rs (expand_tilde, ensure_dir_exists, get_config_paths)
 - [X] T008 Create data structures in src/config/parser.rs (ConfigCollection, ClaudeConfig, Provider structs with serde derives)
 - [X] T009 Implement config.json parser in src/config/parser.rs (parse_config_file function with error handling)
 - [X] T010 [P] Create config manager skeleton in src/config/manager.rs (ConfigManager struct and trait definition)
 - [X] T011 [P] Create CLI argument parser skeleton in src/main.rs (clap derive structs for Cli, Subcommands)
 - [X] T012 Setup utils module exports in src/utils/mod.rs
 - [X] T013 Setup config module exports in src/config/mod.rs

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - 初始化配置分离 (Priority: P1) 🎯 MVP

**Goal**: 读取 ~/.cc-switch/config.json，将混合配置拆分成独立的 config-<name>.json 文件存储在 ~/.cc-switch/separated/ 目录

**Independent Test**: 运行 `ccstart init`，检查 ~/.cc-switch/separated/ 目录下是否生成了多个 config-<name>.json 文件，每个文件包含对应 provider 的 settingsConfig

### Implementation for User Story 1

 - [X] T014 [P] [US1] Implement duplicate name handling logic in src/config/manager.rs (handle_duplicate_names function with suffix counter)
 - [X] T015 [P] [US1] Implement config extraction logic in src/config/manager.rs (extract_providers function: parse providers, handle duplicates, encode names)
 - [X] T016 [US1] Implement config file writing in src/config/manager.rs (write_separated_config function with atomic write and error handling)
 - [X] T017 [US1] Implement init command logic in src/commands/init.rs (read config.json, extract providers, write files to separated/)
 - [X] T018 [US1] Add re-initialization confirmation prompt in src/commands/init.rs (check if separated/ exists, prompt with dialoguer or stdin)
 - [X] T019 [US1] Add --force flag handling in src/commands/init.rs (skip prompt if --force is set)
 - [X] T020 [US1] Wire init command to main CLI in src/main.rs (match Subcommands::Init branch, call init command handler)
 - [X] T021 [US1] Add error messages and logging to stderr in src/commands/init.rs (use eprintln! for [INFO], [WARN], [ERROR])
 - [X] T022 [US1] Create commands module exports in src/commands/mod.rs

**Checkpoint**: At this point, User Story 1 should be fully functional - `ccstart init` creates separated configs

---

## Phase 4: User Story 2 - 快速切换 Claude 配置 (Priority: P1) 🎯 MVP

**Goal**: 通过 `ccstart <name> [args...]` 快速启动 Claude CLI 并使用指定配置，支持参数透传

**Independent Test**: 运行 `ccstart <config-name> "test prompt"`，验证是否正确调用 `claude --settings <对应配置文件路径> "test prompt"` 并透传退出码

### Implementation for User Story 2

 - [X] T023 [P] [US2] Implement config lookup logic in src/config/manager.rs (find_config_file function: encode name, check file existence)
 - [X] T024 [P] [US2] Implement Claude CLI execution in src/commands/run.rs (build command with --settings flag, handle args with proper quoting)
 - [X] T025 [US2] Add exit code passthrough in src/commands/run.rs (use std::process::Command, return child exit code)
 - [X] T026 [US2] Add config not found error handling in src/commands/run.rs (list available configs from separated/ directory)
 - [X] T027 [US2] Wire run command (<name> positional arg) to main CLI in src/main.rs (handle positional config name argument, pass trailing args)
 - [X] T028 [US2] Configure clap for trailing args in src/main.rs (use trailing_var_arg = true and allow_hyphen_values = true)
 - [X] T029 [US2] Add quotes handling for paths with spaces in src/commands/run.rs (ensure --settings path is quoted)

**Checkpoint**: At this point, User Stories 1 AND 2 should both work - MVP is complete (init + run)

---

## Phase 5: User Story 5 - 列出所有配置 (Priority: P2)

**Goal**: 提供 `ccstart list` 命令列出所有可用配置名称，便于用户查询和脚本使用

**Independent Test**: 运行 `ccstart list`，验证是否输出所有 separated/ 目录下的配置名称（每行一个），包含空格或特殊字符的名称使用双引号包裹

### Implementation for User Story 5

- [ ] T030 [P] [US5] Implement config listing logic in src/config/manager.rs (list_configs function: scan separated/ directory, parse config-*.json filenames)
- [ ] T031 [P] [US5] Implement filename decoding in src/config/manager.rs (decode config names from filenames using decode_config_name)
- [ ] T032 [US5] Implement list command in src/commands/list.rs (call manager.list_configs, format output with quotes for special chars)
- [ ] T033 [US5] Handle empty directory case in src/commands/list.rs (show error: "未找到配置，请先运行 'ccstart init'")
- [ ] T034 [US5] Wire list command to main CLI in src/main.rs (match Subcommands::List branch)
- [ ] T035 [US5] Add stdout-only output in src/commands/list.rs (print config names to stdout, keep logs on stderr)

**Checkpoint**: `ccstart list` shows all available configs with proper formatting

---

## Phase 6: User Story 4 - 更新配置 (Priority: P2)

**Goal**: 提供 `ccstart update` 命令重新同步 config.json 的变更到 separated/ 目录

**Independent Test**: 修改 config.json，运行 `ccstart update`，验证 separated/ 目录下的文件是否正确更新（新增、修改、删除）

### Implementation for User Story 4

- [ ] T036 [P] [US4] Implement config diff logic in src/config/manager.rs (compare_configs function: detect added, modified, deleted providers)
- [ ] T037 [P] [US4] Implement selective update in src/config/manager.rs (update_configs function: add new files, update changed files, delete removed files)
- [ ] T038 [US4] Implement update command in src/commands/update.rs (read config.json, compute diff, apply updates to separated/)
- [ ] T039 [US4] Add update summary output in src/commands/update.rs (report: X added, Y updated, Z deleted)
- [ ] T040 [US4] Wire update command to main CLI in src/main.rs (match Subcommands::Update branch)

**Checkpoint**: `ccstart update` correctly syncs config.json changes to separated/ directory

---

## Phase 7: User Story 3 - 命令行自动补全 (Priority: P2)

**Goal**: 提供 `ccstart completions <shell>` 命令生成 shell 补全脚本，支持动态补全配置名称

**Independent Test**: 运行 `ccstart completions bash`，验证输出有效的 bash 补全脚本；在 shell 中 source 后输入 `ccstart <Tab>` 验证是否显示所有可用配置

### Implementation for User Story 3

- [ ] T041 [P] [US3] Implement completions command skeleton in src/commands/completions.rs (accept shell type: bash, zsh, fish, powershell)
- [ ] T042 [P] [US3] Implement static completion generation in src/commands/completions.rs (use clap_complete::generate to output completion script to stdout)
- [ ] T043 [US3] Implement dynamic completion logic in src/main.rs (use CompleteEnv::complete() at program start to intercept completion requests)
- [ ] T044 [US3] Implement ValueCompleter for config names in src/main.rs (read separated/ directory, return config list with proper quoting)
- [ ] T045 [US3] Handle spaces and special chars in completions in src/main.rs (wrap config names with quotes in completion results)
- [ ] T046 [US3] Wire completions command to main CLI in src/main.rs (match Subcommands::Completions branch)
- [ ] T047 [US3] Add shell type validation in src/commands/completions.rs (validate shell argument, return error for unsupported shells)

**Checkpoint**: All user stories are now independently functional - shell completion enhances UX

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Final improvements that affect multiple user stories

- [ ] T048 [P] Add --help and --version flags in src/main.rs (configure clap with version, author, about metadata from Cargo.toml)
- [ ] T049 [P] Add examples to CLI help text in src/main.rs (add examples section with common usage patterns)
- [ ] T050 Improve error messages across all commands (ensure all errors have format: "错误: <描述>\n提示: <解决方案>")
- [ ] T051 [P] Add comprehensive error context with anyhow in all modules (use .context() to add contextual error information)
- [ ] T052 Code cleanup and clippy fixes (run cargo clippy --all-targets --all-features -- -D warnings)
- [ ] T053 [P] Update CLAUDE.md at project root with completed feature (add ccstart commands to Commands section)
- [ ] T054 Run quickstart.md validation (manually verify all quickstart commands work as documented)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-7)**: All depend on Foundational phase completion
  - US1 (Phase 3) can start after Foundational
  - US2 (Phase 4) depends on US1 (needs separated configs to exist)
  - US5 (Phase 5) can start after Foundational (independent of US1/US2)
  - US4 (Phase 6) depends on US1 (reuses init logic)
  - US3 (Phase 7) depends on US5 (completion needs config listing)
- **Polish (Phase 8)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Depends on US1 completion - needs separated configs to exist
- **User Story 5 (P2)**: Can start after Foundational (Phase 2) - Independent
- **User Story 4 (P2)**: Depends on US1 completion - reuses init logic (config parsing, file writing)
- **User Story 3 (P2)**: Depends on US5 completion - uses config listing for dynamic completion

### Within Each User Story

- Foundational tasks first (config parsing, utilities)
- Manager logic before command handlers
- Command handlers before CLI wiring
- Error handling throughout
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel (T002, T003, T004)
- All Foundational tasks marked [P] can run in parallel within their layer:
  - Layer 1 (independent): T005, T006, T007, T008 can run in parallel
  - Layer 2 (depends on Layer 1): T010, T011, T012, T013 can run in parallel
- Within US1: T014, T015 can run in parallel (different concerns in manager.rs)
- Within US2: T023, T024 can run in parallel (different files)
- Within US5: T030, T031 can run in parallel (different functions in manager.rs)
- Within US4: T036, T037 can run in parallel (different functions in manager.rs)
- Within US3: T041, T042 can run in parallel (static vs dynamic concerns)
- Polish tasks: T048, T049, T051, T053 can run in parallel (different files/concerns)

---

## Parallel Example: Foundational Phase

```bash
# Layer 1 - All independent, can run together:
Task: "Create error types in src/error.rs"
Task: "Implement URL encoding in src/utils/encoding.rs"
Task: "Implement filesystem utils in src/utils/fs.rs"
Task: "Create data structures in src/config/parser.rs"

# Layer 2 - After Layer 1 completes:
Task: "Create ConfigManager skeleton in src/config/manager.rs"
Task: "Create CLI parser in src/main.rs"
Task: "Setup utils module exports"
Task: "Setup config module exports"
```

## Parallel Example: User Story 1

```bash
# These tasks can run together (different functions):
Task: "Implement duplicate name handling in src/config/manager.rs"
Task: "Implement config extraction in src/config/manager.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 + User Story 2)

1. Complete Phase 1: Setup → Rust project ready
2. Complete Phase 2: Foundational → Core infrastructure ready (CRITICAL)
3. Complete Phase 3: User Story 1 → `ccstart init` works
4. Complete Phase 4: User Story 2 → `ccstart <name>` works
5. **STOP and VALIDATE**: Test init and run independently
6. **MVP COMPLETE** - Can now manage and switch Claude configs

### Incremental Delivery

1. **MVP**: Setup + Foundational + US1 + US2 → Core functionality works
2. **Enhancement 1**: Add US5 (list) → Easier config discovery
3. **Enhancement 2**: Add US4 (update) → Config maintenance
4. **Enhancement 3**: Add US3 (completions) → Better UX
5. **Polish**: Phase 8 → Production-ready

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (init)
   - Developer B: User Story 5 (list) - independent
3. After US1 completes:
   - Developer A: User Story 2 (run) - depends on US1
   - Developer B: User Story 4 (update) - depends on US1
4. After US5 completes:
   - Developer B: User Story 3 (completions) - depends on US5
5. Both developers: Phase 8 (polish) in parallel

---

## Task Count Summary

- **Phase 1 (Setup)**: 4 tasks
- **Phase 2 (Foundational)**: 9 tasks
- **Phase 3 (US1 - P1)**: 9 tasks
- **Phase 4 (US2 - P1)**: 7 tasks
- **Phase 5 (US5 - P2)**: 6 tasks
- **Phase 6 (US4 - P2)**: 5 tasks
- **Phase 7 (US3 - P2)**: 7 tasks
- **Phase 8 (Polish)**: 7 tasks

**Total**: 54 tasks

### Tasks per User Story

- **US1 (P1 - init)**: 9 tasks
- **US2 (P1 - run)**: 7 tasks
- **US3 (P2 - completions)**: 7 tasks
- **US4 (P2 - update)**: 5 tasks
- **US5 (P2 - list)**: 6 tasks
- **Shared (Setup + Foundational + Polish)**: 20 tasks

### Parallel Opportunities

- Setup: 3/4 tasks can run in parallel
- Foundational: 8/9 tasks can run in parallel (in 2 layers)
- US1: 2/9 pairs can run in parallel
- US2: 2/7 pairs can run in parallel
- US5: 2/6 pairs can run in parallel
- US4: 2/5 pairs can run in parallel
- US3: 2/7 pairs can run in parallel
- Polish: 4/7 tasks can run in parallel

**Total parallel opportunities**: ~17 task pairs identified

---

## Notes

- [P] tasks = different files or different concerns in same file, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Tests are NOT included as they were not requested in spec.md
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- MVP is US1 + US2 (init + run), representing core value proposition
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
