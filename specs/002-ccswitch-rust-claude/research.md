# Research Phase: ccstart 技术调研报告

**Date**: 2025-10-14
**Branch**: 002-ccswitch-rust-claude

本文档记录了 ccstart 项目所有技术决策的研究过程和理由。

---

## 1. CLI 框架

### Decision
**clap v4+**（启用 `derive` 和 `cargo` features）

### Rationale
1. **成熟度最高**：15.6k GitHub stars，被 461,000+ 项目使用，Rust CLI 框架事实标准
2. **功能完全匹配**：
   - ✅ 原生支持子命令（init, update, list, completions, <name>）
   - ✅ 参数透传（`trailing_var_arg = true` 和 `allow_hyphen_values = true`）
   - ✅ Shell 补全生成（通过 `clap_complete`）
   - ✅ 动态补全（运行时读取配置列表）
3. **开发体验优秀**：Derive API 代码简洁，类型安全，自动生成 --help
4. **社区支持强**：完整文档、丰富示例、活跃 GitHub Discussions
5. **许可证友好**：MIT/Apache 2.0 双许可

### Alternatives Considered
- **structopt**：已被 clap v3+ 合并，不再独立维护 ❌
- **argh**：缺少 Unix 约定支持和 shell 补全功能 ❌
- **pico-args**：无 help 生成、无 derive 支持、无子命令支持 ❌
- **gumdrop**：社区规模小、文档不足、无明显优势 ❌

### Implementation Notes
```toml
[dependencies]
clap = { version = "4.5", features = ["derive", "cargo"] }
clap_complete = "4.5"
```

---

## 2. JSON 处理

### Decision
**serde_json + anyhow**

### Rationale
1. **事实标准**：被 68,519 个 crates 使用，每月下载量超过 2,200 万次
2. **类型安全**：充分利用 Rust 类型系统，编译时保证数据结构正确
3. **嵌套结构支持**：完美处理 `claude.providers[].settingsConfig` 的嵌套 JSON
4. **错误处理友好**：结合 `anyhow` 提供清晰的错误上下文
5. **性能足够**：500-1000 MB/s 反序列化速度，配置文件场景完全够用
6. **可扩展性**：可选使用 `serde_path_to_error` 提供精确的 JSON 路径错误定位

### Alternatives Considered
- **json-rust**：快 1.3-2.7 倍，但缺乏类型安全和文档支持 ❌
- **simd-json**：大文件快，但小文件反而慢，需要可变输入 ❌
- **serde_json_lenient**：宽松解析不符合标准 JSON 规范 ❌

### Implementation Notes
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
```

**数据结构示例**：
```rust
#[derive(Debug, Deserialize)]
struct ConfigFile {
    claude: ClaudeConfig,
}

#[derive(Debug, Deserialize)]
struct ClaudeConfig {
    providers: HashMap<String, Provider>,
}

#[derive(Debug, Deserialize)]
struct Provider {
    #[serde(default)]
    name: String,
    #[serde(rename = "settingsConfig")]
    settings_config: serde_json::Value, // 保持原始 JSON 结构
}
```

---

## 3. Shell 补全生成

### Decision
**clap_complete（动态补全模式）**

### Rationale
1. **官方支持**：clap 的官方补全生成库，与 clap derive API 无缝集成
2. **动态补全**：v4.5+ 支持动态补全（`unstable-dynamic` feature），运行时读取配置列表
3. **空格处理**：正确处理包含空格的配置名称（自动转义或双引号包裹）
4. **多 Shell 支持**：Bash, Zsh, Fish, PowerShell, Elvish
5. **许可证友好**：Apache 2.0 或 MIT

### Alternatives Considered
- **bpaf**：社区较小，文档不如 clap_complete 完善 ❌
- **静态补全**：无法处理运行时生成的配置列表 ❌

### Implementation Notes
```toml
[dependencies]
clap_complete = { version = "4.5", features = ["unstable-dynamic"] }
```

**动态补全实现策略**：
1. 在 `main()` 开始调用 `CompleteEnv::complete()` 拦截补全请求
2. 实现 `ValueCompleter` trait，读取 `~/.cc-switch/separated/` 目录
3. 解析 `config-*.json` 文件名，提取配置名称
4. 返回补全候选项（自动处理引号和转义）

**用户安装方式**：
```bash
# Bash
eval "$(ccstart completions bash)"

# Zsh
eval "$(ccstart completions zsh)"
```

---

## 4. 配置管理工具调研

### Decision
**自己实现 ccstart（无现成工具满足需求）**

### Rationale
1. **核心需求独特**：
   - 从混合配置文件（`config.json`）中提取 `claude.providers[].settingsConfig`
   - 解析嵌套 JSON 结构并分离存储
   - **没有任何现成工具支持这个功能**
2. **实现复杂度可控**：预计核心代码 < 500 行
3. **可借鉴成熟模式**：大量参考 cctx 和 CCProfileSwitch 的设计

### Alternatives Considered

调研了 7+ 个类似工具，但均不满足需求：

| 工具 | 功能 | 为何不适用 |
|------|------|-----------|
| **cctx** | kubectx 风格配置切换 | 假设配置已分离，无分离功能 ❌ |
| **CCProfileSwitch** | OAuth/API key 管理 | 环境变量模式，无 JSON 解析 ❌ |
| **cccs/CC-Switch** | GUI 桌面应用 | Tauri 应用，非 CLI 工具 ❌ |
| **claude-switcher** | 模型切换 | 针对模型，非配置文件 ❌ |

### Key Design Patterns Borrowed

#### 从 cctx 借鉴：
- CLI 架构：子命令 + 位置参数混合
- 配置存储结构：独立的 `separated/` 目录
- 技术栈：`clap` + `serde_json` + `dirs`

#### 从 CCProfileSwitch 借鉴：
- Shell 集成模式
- 初始化向导（重新初始化确认提示）
- 描述性错误信息

#### 从 AWS Profile Switcher 借鉴：
- 配置名称补全模式
- Shell prompt 显示当前配置（可选增强）

---

## 5. 文件名安全编码

### Decision
**URL 编码（Percent-Encoding）+ 自定义字符集**

使用 Rust 的 `percent-encoding` crate

### Rationale
1. **标准化**：基于 RFC 3986 标准
2. **可逆性**：完美支持编码/解码往返
3. **可读性**：只编码必要字符，常规字符保持原样
4. **空格保留**：符合 spec.md 要求（空格保留，用双引号包裹）
5. **跨平台**：兼容 Linux、macOS 文件系统
6. **成熟度**：800M+ crates.io 下载量，官方 Rust URL 工作组维护

### Alternatives Considered
- **urlencoding**：无法自定义编码字符集，会编码空格为 `%20` ❌

### Implementation Notes
```toml
[dependencies]
percent-encoding = "2.3"
```

**需要编码的字符**：
```rust
const FILENAME_UNSAFE: &AsciiSet = &CONTROLS
    .add(b'/') // 路径分隔符
    .add(b':') // Windows 驱动器分隔符
    .add(b'*') // 通配符
    .add(b'?') // 通配符
    .add(b'"') // 引号
    .add(b'<') // 重定向
    .add(b'>') // 重定向
    .add(b'|') // 管道
    .add(b'\\'); // Windows 路径分隔符
    // 注意：空格不编码
```

**编码/解码示例**：
```rust
// 编码
fn encode_config_name(name: &str) -> String {
    utf8_percent_encode(name, FILENAME_UNSAFE).to_string()
}

// 解码
fn decode_config_name(encoded: &str) -> Result<String, std::str::Utf8Error> {
    percent_decode_str(encoded)
        .decode_utf8()
        .map(|s| s.into_owned())
}
```

| 原始配置名称 | 编码后文件名 |
|------------|------------|
| `packycode` | `config-packycode.json` |
| `my config` | `config-my config.json` |
| `my/config` | `config-my%2Fconfig.json` |
| `test:prod` | `config-test%3Aprod.json` |
| `Zhipu GLM` | `config-Zhipu GLM.json` |

---

## 推荐技术栈总结

```toml
[package]
name = "ccstart"
version = "0.1.0"
edition = "2024"

[dependencies]
# CLI 框架
clap = { version = "4.5", features = ["derive", "cargo"] }
clap_complete = { version = "4.5", features = ["unstable-dynamic"] }

# JSON 处理
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# 错误处理
anyhow = "1.0"

# 文件系统
dirs = "5.0"           # 跨平台用户目录
path-clean = "1.0"     # 路径规范化

# URL 编码
percent-encoding = "2.3"

# 可选增强
dialoguer = { version = "0.11", optional = true }  # 交互式确认
colored = { version = "2.1", optional = true }     # 彩色输出
```

---

## Constitution Check 更新

基于研究结果，更新宪章合规检查：

### 轮子复用原则 ✅
- [x] 已调研 Rust 生态中的成熟解决方案
- [x] 选择的库均满足：活跃维护、良好文档、社区支持、许可证兼容
- [x] 配置管理逻辑需要自定义实现（无现成工具），但使用成熟库处理基础功能

**选择的成熟库**：
- CLI 解析：`clap` v4（Rust 标准）
- JSON 处理：`serde_json`（Rust 标准）
- Shell 补全：`clap_complete`（官方插件）
- URL 编码：`percent-encoding`（官方维护）

### Linus三问原则 ✅
- [x] **What**: 管理和快速切换 Claude CLI 配置文件
- [x] **Why**: 简化多配置环境下的使用，ccswitch 的配置管理增强工具
- [x] **Better**: 已评估现有工具（cctx, CCProfileSwitch 等），确认无更好的现成方案，但可借鉴其设计模式

---

## 下一步行动

研究阶段完成，所有 NEEDS CLARIFICATION 项已解决。准备进入 Phase 1: Design & Contracts。

**Phase 1 任务**：
1. 生成 `data-model.md`：定义配置实体和数据结构
2. 生成 `contracts/`：定义 CLI 命令接口规范
3. 生成 `quickstart.md`：快速开始指南
4. 更新 agent context
