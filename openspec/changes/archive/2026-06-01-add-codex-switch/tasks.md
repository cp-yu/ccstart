### Task 1: ProviderDao 泛化 app_type 参数

**Goal**: 将 `ProviderDao` 的三个查询方法从硬编码 `app_type='claude'` 改为接受 `app_type` 参数，保持现有调用点行为不变。

**Files**:
- Modify: `src/db/provider.rs`
- Modify: `src/commands/run.rs`
- Modify: `src/commands/list.rs`
- Modify: `src/commands/update.rs`
- Modify: `src/main.rs`

**Requirements**:
- `list_all`、`get_by_name`、`list_names` 接受 `app_type: &str` 参数
- SQL WHERE 条件使用参数化绑定
- 现有调用点传入 `"claude"` 保持行为不变

#### Checks

- [x] C1 验证 claude 查询行为不变
  - Verifies: `specs/provider/spec.md` / Requirement "Claude provider queries" / Scenario "Query claude providers (backward compatible)"
  - Command: `cargo test`
  - Expect: 所有现有测试通过

- [x] C2 验证 codex 查询返回正确结果
  - Verifies: `specs/provider/spec.md` / Requirement "Claude provider queries" / Scenario "Query codex providers"
  - Command: `cargo build && cargo run -- codex list`
  - Expect: 输出所有 codex 渠道名称

### Task 2: Codex 子命令 CLI 定义与分发

**Goal**: 在 clap CLI 中新增 `codex` 子命令入口（支持 `list` 和默认 `<channel> [args...]`），并在 `run_app` 中分发。

**Files**:
- Modify: `src/main.rs`
- Modify: `src/commands/mod.rs`
- Create: `src/commands/codex.rs`

**Requirements**:
- 新增 `Commands::Codex` 枚举变体
- `channel` 参数挂载 `codex_channel_completer`
- `args` 使用 `trailing_var_arg` + `allow_hyphen_values` 透传
- `codex list` 分发到 list 处理函数

#### Checks

- [x] C3 验证 codex 子命令解析
  - Verifies: `specs/codex-dispatch/spec.md` / Requirement "Codex channel dispatch" / Scenario "Successful codex launch with channel"
  - Command: `cargo build`
  - Expect: 编译成功，无 warning

- [x] C4 验证 codex list 子命令
  - Verifies: `specs/codex-dispatch/spec.md` / Requirement "Codex list subcommand" / Scenario "List codex channels"
  - Command: `cargo run -- codex list`
  - Expect: 输出所有 codex 渠道名称，含空格的名称用双引号包裹

### Task 3: Codex 渠道启动逻辑

**Goal**: 实现 `commands::codex::run`，从 DB 查询 codex provider → 解析 TOML → 构造 `codex` 命令并执行。

**Files**:
- Modify: `src/commands/codex.rs`
- Modify: `Cargo.toml`

**Requirements**:
- 新增 `toml` crate 依赖
- 从 `settings_config` JSON 提取 `auth.OPENAI_API_KEY`
- 从 `settings_config.config` TOML 字符串解析 `model_provider`、`model`、`base_url`
- 构造 `Command::new("codex")` 注入 env + `-c` 参数 + 用户 args
- 退出码透传（含 unix 信号处理）

#### Checks

- [x] C5 验证 TOML 解析正确提取字段
  - Verifies: `specs/codex-dispatch/spec.md` / Requirement "Codex config TOML parsing" / Scenario "Standard config extraction"
  - Command: `cargo test`
  - Expect: TOML 解析单元测试通过

- [x] C6 验证缺失字段报错
  - Verifies: `specs/codex-dispatch/spec.md` / Requirement "Codex config TOML parsing" / Scenario "Missing required field"
  - Command: `cargo test`
  - Expect: 缺失字段测试通过，错误信息包含字段名

- [x] C7 验证渠道不存在时的错误处理
  - Verifies: `specs/codex-dispatch/spec.md` / Requirement "Codex channel dispatch" / Scenario "Channel not found"
  - Command: `cargo run -- codex nonexistent`
  - Expect: 输出错误信息并列出可用渠道

- [x] C8 验证参数透传
  - Verifies: `specs/codex-dispatch/spec.md` / Requirement "Codex channel dispatch" / Scenario "Trailing arguments are passed through"
  - Command: `cargo build`
  - Expect: 编译成功；手动验证 `ccstart codex free --help` 透传 `--help` 给 codex

### Task 4: Codex 渠道动态补全

**Goal**: 实现 `codex_channel_completer` 函数，为 codex 子命令的 channel 参数提供 shell 动态自动补全。

**Files**:
- Modify: `src/main.rs`

**Requirements**:
- 查询 `app_type='codex'` 的 provider 名称
- 按前缀过滤候选
- 数据库不可用时返回空列表（不崩溃）

#### Checks

- [x] C9 验证补全返回 codex 渠道
  - Verifies: `specs/codex-completion/spec.md` / Requirement "Codex channel dynamic completion" / Scenario "Completion returns all codex channels"
  - Command: `_CLAP_IFS=$'\013' _CLAP_COMPLETE_INDEX=2 _CLAP_COMPLETE_COMP_TYPE=9 _CLAP_COMPLETE_SPACE=true COMPLETE=bash target/debug/ccstart -- ccstart codex ''`
  - Expect: 补全输出包含 codex 渠道名称

- [x] C10 验证前缀过滤
  - Verifies: `specs/codex-completion/spec.md` / Requirement "Codex channel dynamic completion" / Scenario "Completion filters by prefix"
  - Evidence: 代码审查 `codex_channel_completer` 中的前缀匹配逻辑
  - Expect: 与 `config_name_completer` 相同的 `starts_with` 过滤模式

- [x] C11 验证数据库不可用时不崩溃
  - Verifies: `specs/codex-completion/spec.md` / Requirement "Codex channel dynamic completion" / Scenario "Database unavailable during completion"
  - Evidence: 代码审查 `codex_channel_completer` 中的 `if let Ok(...)` 模式
  - Expect: 使用与 `config_name_completer` 相同的容错模式（`if let Ok` 链）

### Task 5: 非破坏性验证与集成测试

**Goal**: 验证整个 codex 路径不修改 `~/.codex/` 目录，端到端集成测试。

**Files**:
- Modify: `src/commands/codex.rs`

**Requirements**:
- 确认 `Command::new("codex")` 不设置 `CODEX_HOME` 环境变量
- 确认不写入任何文件
- `cargo clippy` 无 warning

#### Checks

- [x] C12 验证非破坏性执行
  - Verifies: `specs/codex-dispatch/spec.md` / Requirement "Non-destructive execution" / Scenario "Home directory unchanged after execution"
  - Evidence: 代码审查 `commands::codex::run` 中无 `fs::write`/`fs::create_dir`/`env("CODEX_HOME")` 调用
  - Expect: 仅使用 `Command::new("codex").env("OPENAI_API_KEY", ...).arg("-c", ...)` 模式

- [x] C13 验证代码质量
  - Verifies: `specs/codex-dispatch/spec.md` / Requirement "Codex channel dispatch" / Scenario "Successful codex launch with channel"
  - Command: `cargo clippy --all-targets --all-features -- -D warnings`
  - Expect: 零 warning
