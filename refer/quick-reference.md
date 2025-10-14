# ccstart 调研结论速查表

## 🎯 核心结论

**应该自己实现 ccstart**，因为：
- ✅ 核心需求（配置分离）无现成工具
- ✅ 实现复杂度低（预计 < 500 行核心代码）
- ✅ 可大量借鉴已有工具的设计模式

---

## 📦 推荐技术栈

```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
clap_complete = "4.5"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
dirs = "5.0"
percent-encoding = "2.3"
anyhow = "1.0"

# 可选增强
dialoguer = { version = "0.11", optional = true }
colored = { version = "2.1", optional = true }
```

---

## 🔍 关键参考工具

### 1. **cctx** (Rust) ⭐⭐⭐⭐⭐
- GitHub: https://github.com/nwiizo/cctx
- **借鉴**：
  - ✅ 独立 JSON 文件存储
  - ✅ `.cctx-state.json` 状态跟踪
  - ✅ `clap` + `serde_json` + `dirs` 技术栈
  - ✅ `dialoguer` 交互式选择
  - ✅ `clap_complete` 补全生成

### 2. **CCProfileSwitch** (Python) ⭐⭐⭐⭐
- GitHub: https://github.com/biostochastics/CCProfileSwitch
- **借鉴**：
  - ✅ Shell 包装脚本（`cpswitch`）
  - ✅ 环境变量透传模式
  - ✅ 初始化向导设计
  - ✅ 描述性错误信息

### 3. **AWS Profile Switcher** ⭐⭐⭐
- **借鉴**：
  - ✅ `fzf` 集成
  - ✅ Prompt 显示当前配置
  - ✅ 环境变量切换模式

---

## 🛠️ 核心设计模式

### 1. CLI 架构
```rust
ccstart                     # 列出所有配置
ccstart <name> [args...]   # 切换并调用 claude
ccstart init [--yes]       # 初始化配置分离
ccstart update             # 更新配置
ccstart list               # 列出配置名称
ccstart completions <shell> # 生成补全脚本
```

### 2. 文件结构
```
~/.cc-switch/
├── config.json                    # 源混合配置（只读）
├── separated/                     # 分离后的配置
│   ├── config-packycode.json
│   ├── config-work.json
│   └── config-personal.json
└── .ccstart-state.json           # 状态跟踪（可选）
```

### 3. 配置切换实现
```rust
// 调用方式
claude --settings ~/.cc-switch/separated/config-{name}.json "$@"

// 透传退出码
std::process::Command::new("claude")
    .arg("--settings")
    .arg(config_path)
    .args(user_args)
    .status()
```

### 4. 配置名称安全化
```rust
// 空格保留，特殊字符 URL 编码
"my config" -> "config-my config.json"
"client/project" -> "config-client%2Fproject.json"
```

### 5. Shell 补全
```rust
use clap_complete::{generate, Shell};

// 静态补全脚本
generate(shell, &mut cmd, "ccstart", &mut io::stdout());

// 动态配置名称补全
fn complete_config_names() -> Vec<String> {
    // 扫描 ~/.cc-switch/separated/ 目录
}
```

---

## 📋 实现 Checklist

### Phase 1: MVP (P1 - 必须)
- [ ] `ccstart init` - 配置分离
  - [ ] 读取 `config.json`
  - [ ] 解析 `claude.providers[].settingsConfig`
  - [ ] 保存为独立文件（处理重复名称）
  - [ ] 重新初始化确认
- [ ] `ccstart <name> [args...]` - 配置切换
  - [ ] 检查配置文件存在
  - [ ] 调用 `claude --settings ...`
  - [ ] 透传退出码
  - [ ] 错误提示（配置不存在时列出可用配置）

### Phase 2: 增强 (P2 - 重要)
- [ ] `ccstart list` - 配置列表
- [ ] `ccstart update` - 配置同步
- [ ] `ccstart completions <shell>` - 补全脚本

### Phase 3: 可选优化
- [ ] 交互式选择（`dialoguer`）
- [ ] 彩色输出（`colored`）
- [ ] fzf 集成
- [ ] JSON schema 验证

---

## ⚠️ 注意事项

### 必须遵守的 Spec 要求
1. ✅ **日志输出到 stderr**（用户可通过 `2>/dev/null` 静默）
2. ✅ **透传 Claude CLI 退出码**
3. ✅ **空格保留在文件名中，其他特殊字符 URL 编码**
4. ✅ **命令行参数用双引号包裹包含空格的配置名**
5. ✅ **无并发控制**（依赖原子文件操作）

### 测试用例必须覆盖
- [ ] 重复配置名称处理（"default" -> "default-2"）
- [ ] 特殊字符文件名（空格、斜杠、冒号等）
- [ ] 配置文件不存在错误
- [ ] `config.json` 格式错误
- [ ] Claude CLI 不在 PATH
- [ ] 权限错误

---

## 🔗 参考链接

- **调研完整报告**: [research-report.md](./research-report.md)
- **项目规格**: [../specs/002-ccswitch-rust-claude/spec.md](../specs/002-ccswitch-rust-claude/spec.md)
- **cctx 源码**: https://github.com/nwiizo/cctx
- **clap 文档**: https://docs.rs/clap/
- **serde_json 文档**: https://docs.rs/serde_json/

---

**更新时间**: 2025-10-14
