# ccstart 工具调研报告

**调研日期**: 2025-10-14
**调研目标**: 确定是否有现成工具可以解决 ccstart 的需求，或可借鉴的类似工具

---

## 一、调研结果总结

### 1.1 是否找到现成工具？

**结论：找到多个类似工具，但没有完全匹配 ccstart 核心需求的现成方案。**

找到 **7+ 个** Claude CLI 配置管理工具，但它们的关注点主要在：
- ✅ **配置切换**（所有工具都支持）
- ✅ **OAuth/API Key 管理**（部分工具）
- ✅ **多环境管理**（大部分工具）
- ❌ **从混合配置文件中分离配置**（**无工具支持**）

**ccstart 的独特需求**：
1. 从 `~/.cc-switch/config.json` 混合配置文件中**提取并分离**各个 provider 的 settingsConfig
2. 管理分离后的独立配置文件（`config-<name>.json`）
3. 提供快速切换功能

**核心差异**：
- 现有工具假设配置文件已经是独立的，直接进行切换
- ccstart 需要先**解析嵌套的 JSON 结构**（`claude.providers[].settingsConfig`），然后**分离存储**
- 这是一个**配置预处理 + 管理工具**，而不是纯粹的切换工具

---

## 二、类似工具分析

### 2.1 Claude CLI 配置管理工具

#### 🏆 **Tool 1: cctx (Rust)**
- **GitHub**: https://github.com/nwiizo/cctx
- **语言**: Rust
- **Stars**: 活跃项目
- **发布**: crates.io 上架

**设计特点**：
1. **kubectx 风格**的 CLI 交互（简洁、直观）
2. **多层级配置支持**：
   - User-level: `~/.claude/settings.json`
   - Project-level: `./.claude/settings.json`
   - Local-level: `./.claude/settings.local.json`
3. **配置存储方式**：
   - 每个配置作为独立 JSON 文件存储在 `~/.claude/settings/` 目录
   - 使用 `.cctx-state.json` 跟踪当前激活的配置
4. **交互体验**：
   - 支持 fzf 模糊搜索
   - 内置 fallback fuzzy finder
   - 彩色输出和视觉指示器
5. **Shell 集成**：
   - 使用 `clap` 生成补全脚本
   - 支持 bash/zsh/fish/powershell

**技术栈**：
```toml
clap = { version = "4.5", features = ["derive", "env"] }
serde = "1.0"
serde_json = "1.0"
dirs = "5.0"              # 跨平台路径处理
colored = "2.1"           # 终端颜色
dialoguer = "0.11"        # 交互式提示
anyhow = "1.0"            # 错误处理
```

**可借鉴点**：
- ✅ 独立 JSON 文件存储每个配置（与 ccstart 需求一致）
- ✅ 状态文件跟踪当前配置
- ✅ 使用 `clap` 的 derive API 简化 CLI 构建
- ✅ `dialoguer` 提供交互式选择（可用于 `ccstart init` 确认）
- ✅ 使用 `dirs` crate 处理跨平台路径

**不适用点**：
- ❌ 不支持从混合配置文件中分离
- ❌ 不处理嵌套 JSON 结构解析

---

#### 🥈 **Tool 2: CCProfileSwitch (Python)**
- **GitHub**: https://github.com/biostochastics/CCProfileSwitch
- **语言**: Python (pipx 安装)
- **特点**: OAuth token 管理 + Z-AI 集成

**设计特点**：
1. **安全存储**：使用系统 Keyring（macOS Keychain/Windows Credential Manager）
2. **Provider 支持**：
   - Claude OAuth tokens
   - Z-AI API keys
   - 区分不同认证方式
3. **Shell 集成设计**（亮点）：
   - 提供 `cpswitch` 命令包装器（用于 Z-AI）
   - 自动设置环境变量：
     ```bash
     export ANTHROPIC_BASE_URL="https://api.z.ai/api/anthropic"
     export ANTHROPIC_AUTH_TOKEN="<key>"
     unset ANTHROPIC_API_KEY
     ```
   - 通过 `source shell-integration.sh` 集成到 shell
4. **配置透传**：
   - 直接修改 `~/.claude/.credentials.json` 或系统 Keyring
   - 需要重启 Claude Code 生效

**可借鉴点**：
- ✅ **Shell 包装脚本设计**：可以为 ccstart 提供 `ccstart <name>` 直接调用 `claude --settings ...` 的能力
- ✅ **环境变量透传模式**：避免硬编码路径
- ✅ **初始化向导**：`claude-profile init` 自动检测当前配置
- ✅ **描述性错误信息**：每个错误都有清晰的解决方案

**不适用点**：
- ❌ Python 实现（ccstart 使用 Rust）
- ❌ 关注于认证管理，而非配置文件分离

---

#### 🥉 **Tool 3: CC-Switch (Tauri + Rust)**
- **网站**: https://www.vibesparking.com/en/blog/ai/claude-code/cc-switch/
- **GitHub**: https://github.com/breakstring/cccs (另一个类似项目)
- **类型**: 桌面应用（GUI）

**设计特点**：
1. **配置覆盖模式**：
   - 将选定的 provider 配置**覆盖**到主配置文件
   - 自动备份当前配置（支持回滚）
2. **Provider 预设**：
   - 内置多个 AI provider 配置（DeepSeek, Qwen, GLM, Kimi）
   - 一键切换
3. **备份策略**：
   - 时间戳命名的备份文件
   - 支持配置迁移

**可借鉴点**：
- ✅ **自动备份机制**：在修改前备份原配置
- ✅ **配置验证**：切换前验证 JSON 格式
- ⚠️ GUI 设计不适用（ccstart 是纯 CLI）

**不适用点**：
- ❌ Tauri 桌面应用（ccstart 需要纯 CLI）
- ❌ 覆盖模式（ccstart 使用 `--settings` 参数切换）

---

### 2.2 其他 CLI 配置管理工具

#### 🔧 **Git Profile Manager**
- **GitHub**: https://github.com/agc93/git-profile-manager
- **语言**: .NET/C#

**设计模式**：
- 管理多个 Git 配置 profile（user.name, user.email, etc.）
- 子命令设计：`list`, `use`, `add`, `remove`
- 存储独立配置文件，使用时应用到 `.git/config`

**可借鉴点**：
- ✅ **子命令结构**清晰（init, update, list, <name>）
- ✅ **配置应用模式**：存储 → 应用分离

---

#### 🔧 **AWS Profile Switcher**
- **示例**: https://github.com/johnnyopao/awsp
- **常见实现**: Shell 脚本或 Python

**设计模式**：
1. **环境变量切换**：
   ```bash
   export AWS_PROFILE=production
   ```
2. **Shell 集成**：
   - 在 prompt 中显示当前 profile
   - 使用 `fzf` 进行交互式选择
3. **配置来源**：读取 `~/.aws/credentials` 和 `~/.aws/config`

**可借鉴点**：
- ✅ **环境变量模式**：虽然 ccstart 使用 `--settings` 参数，但可以考虑 `CLAUDE_SETTINGS` 环境变量作为补充
- ✅ **fzf 集成**：提供更好的交互体验（可选功能）
- ✅ **Prompt 集成**：在 shell prompt 显示当前配置

---

#### 🔧 **direnv / asdf / nvm**
**设计理念**：
- **direnv**: 目录级环境变量自动加载（`.envrc` 文件）
- **asdf**: 统一的运行时版本管理（`.tool-versions` 文件）
- **nvm**: Node.js 版本管理（`.nvmrc` 文件）

**共同模式**：
1. **项目级配置文件**：在项目目录放置配置文件（`.nvmrc`, `.tool-versions`）
2. **自动检测**：`cd` 到目录时自动加载配置
3. **Shell hook**：通过 shell hook（`cd` 钩子）实现自动切换
4. **Shim 机制**：通过 shim 目录拦截命令调用

**可借鉴点**：
- ⚠️ **自动切换模式**不适用（ccstart 是手动切换）
- ✅ **Shell hook 集成**：可以考虑为 ccstart 提供 shell function
- ✅ **配置文件检测**：扫描目录下的配置文件

---

## 三、建议

### 3.1 ccstart 应该自己实现

**理由**：

1. ✅ **独特的核心需求**：
   - 从混合配置文件（`config.json`）中提取和分离配置
   - 解析 `claude.providers[].settingsConfig` 嵌套结构
   - **没有任何现成工具支持这个功能**

2. ✅ **实现复杂度可控**：
   - 配置解析：使用 `serde_json`（成熟库）
   - 文件操作：使用 `std::fs` 和 `dirs`
   - CLI 框架：使用 `clap`（已被 cctx 验证）
   - 预计核心代码 < 500 行

3. ✅ **可借鉴大量成熟模式**：
   - CLI 结构：参考 cctx
   - Shell 集成：参考 CCProfileSwitch
   - 补全生成：使用 `clap_complete`
   - 错误处理：使用 `anyhow` 或 `thiserror`

4. ✅ **Rust 生态支持完善**：
   - JSON 处理：`serde` + `serde_json`
   - CLI 框架：`clap` v4
   - 文件路径：`dirs`, `path-clean`
   - URL 编码：`percent-encoding`（用于文件名特殊字符）

5. ❌ **无法通过扩展现有工具实现**：
   - cctx: 不处理嵌套 JSON，且侧重于 settings.json 切换
   - CCProfileSwitch: Python 实现，架构不匹配
   - CC-Switch: GUI 应用，过于重量级

---

### 3.2 建议的实现策略

#### **阶段 1: MVP 核心功能** (对应 spec.md 中的 P1)
1. **配置分离**：
   - 读取 `~/.cc-switch/config.json`
   - 解析 `claude.providers` 嵌套结构
   - 提取 `settingsConfig` 并保存为独立文件
   - 实现重复名称处理（后缀编号）

2. **配置切换**：
   - `ccstart <name>` 命令
   - 调用 `claude --settings /path/to/config-<name>.json "$@"`
   - 透传退出码

**参考项目**: cctx（文件结构）、CCProfileSwitch（命令透传）

---

#### **阶段 2: 用户体验增强** (对应 spec.md 中的 P2)
1. **Shell 补全**：
   - 使用 `clap_complete`
   - 提供 `ccstart completions <shell>` 命令
   - 输出到 stdout（用户自行重定向）

2. **配置更新**：
   - `ccstart update` 命令
   - 重新读取 `config.json` 并同步到 `separated/`

3. **配置列表**：
   - `ccstart list` 命令
   - 扫描 `separated/` 目录

**参考项目**: cctx（补全实现）

---

#### **阶段 3: 可选增强**
1. **交互式选择**：
   - 使用 `dialoguer` 实现 `ccstart` 无参数时的交互式选择
   - 可选集成 `fzf`（通过 `which` 检测）

2. **配置验证**：
   - JSON schema 验证（使用 `jsonschema` crate）
   - 检查必需字段

3. **Shell 集成脚本**：
   - 提供 `ccstart-init.sh`，定义 shell function
   - 类似 nvm 的 `nvm use` 模式

**参考项目**: cctx（交互式）、direnv（shell 集成）

---

## 四、可借鉴的设计模式

### 4.1 CLI 架构模式

#### ✅ **模式 1: 子命令 + 位置参数混合**（推荐）
**来源**: cctx, git-profile-manager

```rust
// 使用 clap derive API
#[derive(Parser)]
#[command(name = "ccstart")]
#[command(about = "Claude Settings 配置管理工具")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// 配置名称（用于快速切换）
    name: Option<String>,

    /// 传递给 Claude CLI 的额外参数
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    claude_args: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// 初始化配置分离
    Init {
        #[arg(long)]
        yes: bool,
    },
    /// 更新配置
    Update,
    /// 列出所有配置
    List,
    /// 生成 shell 补全脚本
    Completions {
        shell: clap_complete::Shell,
    },
}
```

**优点**：
- 清晰的子命令分离
- 支持 `ccstart <name>` 快速切换（无需子命令）
- 自动生成帮助信息

---

#### ✅ **模式 2: 配置存储结构**
**来源**: cctx

```
~/.cc-switch/
├── config.json                    # 源混合配置（只读）
├── separated/                     # 分离后的配置目录
│   ├── config-packycode.json
│   ├── config-work.json
│   └── config-personal.json
└── .ccstart-state.json           # 状态跟踪（可选）
```

**优点**：
- 独立的 `separated/` 目录避免污染源配置
- 状态文件可以记录：
  - 上次使用的配置
  - 配置创建时间戳
  - 来源配置的 hash（用于检测更新）

---

#### ✅ **模式 3: Shell 补全生成**
**来源**: cctx, clap_complete 文档

```rust
use clap_complete::{generate, Shell};

fn generate_completions(shell: Shell) {
    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "ccstart", &mut io::stdout());
}

// 动态补全逻辑（补全配置名称）
fn complete_config_names() -> Vec<String> {
    let separated_dir = dirs::home_dir()
        .unwrap()
        .join(".cc-switch/separated");

    std::fs::read_dir(separated_dir)
        .unwrap()
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension()? == "json" {
                let name = path.file_stem()?.to_str()?;
                Some(name.strip_prefix("config-")?.to_string())
            } else {
                None
            }
        })
        .collect()
}
```

**优点**：
- `clap_complete` 自动生成大部分补全逻辑
- 只需实现动态值的补全（配置名称列表）

---

#### ✅ **模式 4: 配置名称安全化**
**来源**: spec.md + URL encoding 标准

```rust
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

fn sanitize_config_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == ' ' || c == '-' || c == '_' {
                c.to_string()
            } else {
                // URL encode 特殊字符
                utf8_percent_encode(&c.to_string(), NON_ALPHANUMERIC).to_string()
            }
        })
        .collect()
}

fn config_name_to_filename(name: &str) -> String {
    format!("config-{}.json", sanitize_config_name(name))
}

// 示例：
// "my config" -> "config-my config.json"
// "client/project" -> "config-client%2Fproject.json"
```

**优点**：
- 空格保留（符合 spec 要求）
- 文件系统不安全字符（`/`, `:`, 等）被编码
- 可逆（可以从文件名恢复原始名称）

---

#### ✅ **模式 5: 命令透传 + 退出码处理**
**来源**: CCProfileSwitch 的 shell 集成

```rust
use std::process::{Command, ExitCode};

fn run_claude_with_config(config_name: &str, args: &[String]) -> ExitCode {
    let config_path = get_config_path(config_name);

    // 检查配置文件是否存在
    if !config_path.exists() {
        eprintln!("错误: 配置 '{}' 不存在", config_name);
        eprintln!("可用配置: {:?}", list_configs());
        return ExitCode::from(1);
    }

    // 构建命令
    let mut cmd = Command::new("claude");
    cmd.arg("--settings").arg(&config_path);

    // 添加用户参数（需要处理空格）
    for arg in args {
        cmd.arg(arg);
    }

    // 执行并透传退出码
    match cmd.status() {
        Ok(status) => {
            ExitCode::from(status.code().unwrap_or(1) as u8)
        }
        Err(e) => {
            eprintln!("错误: 无法执行 claude 命令: {}", e);
            eprintln!("请确保 Claude CLI 已安装且在 PATH 中");
            ExitCode::from(127)
        }
    }
}
```

**优点**：
- 完整透传退出码（符合 FR-014）
- 参数正确传递（自动处理空格）
- 清晰的错误信息

---

#### ✅ **模式 6: 日志输出到 stderr**
**来源**: spec.md FR-013

```rust
// 使用宏简化 stderr 输出
macro_rules! log_info {
    ($($arg:tt)*) => {
        eprintln!("[INFO] {}", format!($($arg)*))
    };
}

macro_rules! log_warn {
    ($($arg:tt)*) => {
        eprintln!("[WARN] {}", format!($($arg)*))
    };
}

macro_rules! log_error {
    ($($arg:tt)*) => {
        eprintln!("[ERROR] {}", format!($($arg)*))
    };
}

// 使用：
log_info!("正在读取配置文件: {}", path.display());
log_warn!("配置名称重复，使用后缀: {}", new_name);
log_error!("配置文件格式错误: {}", err);
```

**优点**：
- 所有日志输出到 stderr（符合 spec 要求）
- 用户可以通过 `2>/dev/null` 静默日志
- stdout 保留用于功能性输出（补全脚本、配置列表）

---

### 4.2 配置解析模式

#### ✅ **模式 7: 嵌套 JSON 解析**
**来源**: spec.md + serde 文档

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
struct ConfigRoot {
    claude: ClaudeConfig,
}

#[derive(Deserialize)]
struct ClaudeConfig {
    providers: HashMap<String, Provider>,
}

#[derive(Deserialize)]
struct Provider {
    id: String,
    name: String,
    #[serde(rename = "settingsConfig")]
    settings_config: serde_json::Value, // 保留完整 JSON
}

fn extract_configs(config_path: &Path) -> anyhow::Result<Vec<(String, serde_json::Value)>> {
    let content = std::fs::read_to_string(config_path)?;
    let root: ConfigRoot = serde_json::from_str(&content)?;

    let mut configs = Vec::new();
    let mut name_counts: HashMap<String, usize> = HashMap::new();

    for (_, provider) in root.claude.providers {
        let mut name = provider.name.clone();

        // 处理重复名称
        let count = name_counts.entry(name.clone()).or_insert(0);
        *count += 1;
        if *count > 1 {
            name = format!("{}-{}", name, count);
        }

        configs.push((name, provider.settings_config));
    }

    Ok(configs)
}
```

**优点**：
- 使用 `serde_json::Value` 保留完整 JSON 结构（不需要定义完整 schema）
- 自动处理重复名称（符合 spec 要求）
- 错误处理清晰（使用 `anyhow`）

---

#### ✅ **模式 8: 原子文件写入**
**来源**: CCProfileSwitch（防止并发冲突）

```rust
use std::io::Write;

fn write_config_atomically(path: &Path, content: &str) -> anyhow::Result<()> {
    let temp_path = path.with_extension("tmp");

    // 写入临时文件
    let mut file = std::fs::File::create(&temp_path)?;
    file.write_all(content.as_bytes())?;
    file.sync_all()?; // 确保数据写入磁盘

    // 原子重命名
    std::fs::rename(temp_path, path)?;

    Ok(())
}
```

**优点**：
- 原子操作（要么成功，要么失败，不会有部分写入）
- 防止并发写入导致的文件损坏
- 符合 spec 的"无锁设计"（不需要显式锁，依赖文件系统原子操作）

---

### 4.3 用户体验模式

#### ✅ **模式 9: 交互式确认**
**来源**: cctx 的 dialoguer 使用

```rust
use dialoguer::Confirm;

fn confirm_reinit() -> anyhow::Result<bool> {
    Confirm::new()
        .with_prompt("配置已初始化，是否重新初始化？这将删除所有现有配置文件。")
        .default(false)
        .interact()
}

// 在 init 命令中使用
if separated_dir.exists() && !args.yes {
    if !confirm_reinit()? {
        log_info!("取消操作");
        return Ok(());
    }

    // 删除并重建
    std::fs::remove_dir_all(&separated_dir)?;
}
```

**优点**：
- 防止误操作
- 支持 `--yes` 标志跳过确认（CI/CD 友好）
- 清晰的提示信息

---

#### ✅ **模式 10: 彩色输出**
**来源**: cctx 的 colored 使用

```rust
use colored::*;

fn list_configs() -> anyhow::Result<()> {
    let configs = scan_configs()?;

    if configs.is_empty() {
        eprintln!("{}", "未找到配置，请先运行 'ccstart init'".yellow());
        return Ok(());
    }

    for name in configs {
        // 如果名称包含空格或特殊字符，使用双引号包裹
        if name.contains(' ') || name.contains('%') {
            println!("\"{}\"", name.green());
        } else {
            println!("{}", name.green());
        }
    }

    Ok(())
}
```

**优点**：
- 视觉反馈清晰
- 可以通过 `NO_COLOR` 环境变量禁用（符合标准）

---

## 五、推荐技术栈

### 5.1 核心依赖

```toml
[dependencies]
# CLI 框架
clap = { version = "4.5", features = ["derive", "cargo", "env"] }
clap_complete = "4.5"  # Shell 补全

# JSON 处理
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# 文件系统和路径
dirs = "5.0"           # 跨平台用户目录
path-clean = "1.0"     # 路径规范化

# URL 编码（文件名特殊字符处理）
percent-encoding = "2.3"

# 错误处理
anyhow = "1.0"         # 简单的错误传播
# 或者使用 thiserror = "1.0"  # 更结构化的错误定义

# 用户交互（可选）
dialoguer = { version = "0.11", optional = true }
colored = { version = "2.1", optional = true }

[features]
default = ["interactive", "colors"]
interactive = ["dialoguer"]
colors = ["colored"]
```

---

### 5.2 项目结构建议

```
ccstart/
├── src/
│   ├── main.rs              # CLI 入口
│   ├── cli.rs               # Clap 命令定义
│   ├── config/
│   │   ├── mod.rs           # 配置模块
│   │   ├── parser.rs        # JSON 解析逻辑
│   │   ├── storage.rs       # 文件读写
│   │   └── sanitize.rs      # 文件名安全化
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── init.rs          # init 子命令
│   │   ├── update.rs        # update 子命令
│   │   ├── list.rs          # list 子命令
│   │   ├── switch.rs        # 配置切换逻辑
│   │   └── completions.rs   # 补全生成
│   └── utils/
│       ├── mod.rs
│       ├── paths.rs         # 路径处理
│       └── logging.rs       # 日志宏
├── tests/
│   ├── integration_test.rs
│   └── fixtures/            # 测试用的示例配置
├── Cargo.toml
└── README.md
```

---

## 六、风险与注意事项

### 6.1 技术风险

1. **JSON 格式变化**：
   - 风险：cc-switch 的 `config.json` 格式可能变化
   - 缓解：使用宽松的 JSON 解析（`serde_json::Value`），只提取需要的字段
   - 建议：提供 `--version` 显示支持的格式版本

2. **特殊字符处理**：
   - 风险：配置名称包含空格、斜杠等特殊字符
   - 缓解：URL 编码 + 双引号包裹（已在 spec 中明确）
   - 测试：确保测试用例覆盖各种特殊字符

3. **并发写入**：
   - 风险：spec 明确不处理并发
   - 缓解：使用原子文件操作（临时文件 + 重命名）
   - 文档：在 README 中说明限制

---

### 6.2 用户体验风险

1. **Claude CLI 不在 PATH**：
   - 缓解：执行前使用 `which` crate 检查 `claude` 命令是否存在
   - 错误信息：提供明确的安装指引

2. **配置文件权限**：
   - 缓解：在操作前检查 `~/.cc-switch/` 目录权限
   - 错误信息：提示用户使用 `chmod` 修复

3. **配置名称歧义**：
   - 风险：用户可能不清楚"配置名称"和"文件名"的关系
   - 缓解：在 `list` 命令中同时显示名称和文件路径（`--verbose` 模式）

---

## 七、实现优先级建议

### Phase 1: MVP (1-2 天)
- ✅ `ccstart init` - 配置分离
- ✅ `ccstart <name>` - 配置切换
- ✅ 错误处理和日志输出
- ✅ 基本测试

### Phase 2: 增强 (1 天)
- ✅ `ccstart list` - 配置列表
- ✅ `ccstart update` - 配置同步
- ✅ `ccstart completions` - Shell 补全

### Phase 3: 优化 (可选)
- ⚠️ 交互式选择（`dialoguer`）
- ⚠️ 配置验证（JSON schema）
- ⚠️ 彩色输出（`colored`）
- ⚠️ fzf 集成

---

## 八、总结

### ✅ 最终结论

**ccstart 应该自己实现**，理由如下：

1. **核心需求独特**：从混合配置文件中分离配置是其他工具不支持的功能
2. **实现复杂度低**：Rust 生态提供完善的工具库，预计核心代码 < 500 行
3. **可大量借鉴**：已有 7+ 个类似工具提供了清晰的设计模式参考
4. **技术栈成熟**：`clap` + `serde_json` + `dirs` 是经过验证的组合

### 📚 关键借鉴来源

| 方面 | 参考工具 | 借鉴内容 |
|------|----------|----------|
| CLI 架构 | cctx (Rust) | 子命令设计、独立文件存储、状态跟踪 |
| Shell 集成 | CCProfileSwitch (Python) | 命令透传、环境变量设置、初始化向导 |
| 配置管理 | git-profile-manager | 存储与应用分离的模式 |
| 补全生成 | cctx + clap_complete | 自动生成补全脚本 + 动态值补全 |
| 交互体验 | AWS profile switcher | fzf 集成、提示显示当前配置 |

### 🎯 核心设计决策

1. **存储方式**：独立 JSON 文件（`config-<name>.json`）
2. **切换方式**：`claude --settings <path>` 参数（而非修改主配置）
3. **补全方式**：`clap_complete` 生成静态脚本 + 动态读取配置列表
4. **错误处理**：使用 `anyhow`，所有日志输出到 stderr
5. **并发策略**：无锁设计 + 原子文件操作

---

**报告完成时间**: 2025-10-14
**下一步行动**: 开始实现 MVP（Phase 1）
