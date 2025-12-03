# CLI Interface Contract: ccstart

**Date**: 2025-10-14
**Branch**: 002-ccswitch-rust-claude
**Version**: 1.0.0

本文档定义 ccstart 命令行工具的完整接口规范。

---

## 概述

```bash
ccstart - Claude CLI 配置管理工具

USAGE:
    ccstart [OPTIONS] [SUBCOMMAND]
    ccstart <NAME> [ARGS...]

SUBCOMMANDS:
    init            初始化配置分离
    update          更新分离的配置文件
    list            列出所有可用配置
    completions     生成 shell 补全脚本
    help            显示帮助信息

OPTIONS:
    -h, --help       显示帮助信息
    -V, --version    显示版本信息

ARGUMENTS:
    <NAME>           配置名称（快速切换模式）
    <ARGS>...        传递给 Claude CLI 的额外参数
```

---

## 命令详细规范

### 1. `ccstart init`

**功能**：从 `~/.cc-switch/config.json` 读取混合配置，拆分成独立的配置文件。

#### 语法

```bash
ccstart init [--force]
```

#### 参数

| 参数 | 类型 | 必需 | 默认值 | 描述 |
|------|------|------|--------|------|
| `--force`, `-f` | Flag | 否 | false | 强制重新初始化，不提示确认 |

#### 行为

1. **检查源配置文件**：
   - 验证 `~/.cc-switch/config.json` 是否存在
   - 解析 JSON 格式

2. **检查目标目录**：
   - 如果 `~/.cc-switch/separated/` 已存在：
     - 无 `--force` 标志：提示用户确认是否重新初始化
     - 有 `--force` 标志：直接删除并重建

3. **提取配置**：
   - 从 `claude.providers` 中提取每个 provider
   - 处理重复的配置名称（添加数字后缀）
   - 编码配置名称（URL 编码特殊字符，保留空格）

4. **写入文件**：
   - 为每个 provider 创建 `config-<name>.json` 文件
   - 写入 settingsConfig 内容

#### 输出

**成功**：
```
[INFO] 正在读取配置文件: ~/.cc-switch/config.json
[INFO] 找到 5 个 provider 配置
[INFO] 正在分离配置...
✓ 已提取: packycode -> ~/.cc-switch/separated/config-packycode.json
✓ 已提取: Zhipu GLM -> ~/.cc-switch/separated/config-Zhipu GLM.json
✓ 已提取: work -> ~/.cc-switch/separated/config-work.json
✓ 已提取: default -> ~/.cc-switch/separated/config-default.json
[WARN] 检测到重复的配置名称 'default'，使用后缀: default-2
✓ 已提取: default-2 -> ~/.cc-switch/separated/config-default-2.json
[INFO] 配置初始化完成！已生成 5 个配置文件
```

**重新初始化确认**（无 `--force` 时）：
```
[WARN] 配置已初始化，是否重新初始化？这将删除所有现有配置文件。(y/N): _
```

#### 退出码

| 退出码 | 含义 |
|--------|------|
| 0 | 成功 |
| 1 | 配置文件不存在或格式错误 |
| 2 | 用户取消操作 |
| 3 | 文件系统错误（权限不足等） |

#### 错误场景

```bash
# 配置文件不存在
$ ccstart init
错误: 配置文件不存在: ~/.cc-switch/config.json
提示: 请确保已安装 ccswitch 并创建了配置文件。
退出码: 1

# JSON 格式错误
$ ccstart init
错误: JSON 格式错误位于第 10 行第 5 列
提示: 检查该位置附近是否有多余的逗号、缺少引号或括号不匹配
退出码: 1

# 权限不足
$ ccstart init
错误: 无法创建目录: ~/.cc-switch/separated (权限被拒绝)
提示: 检查文件系统权限
退出码: 3
```

---

### 2. `ccstart update`

**功能**：重新读取 `config.json` 并同步到 `separated/` 目录。

#### 语法

```bash
ccstart update
```

#### 参数

无参数。

#### 行为

1. **读取源配置**：
   - 读取 `~/.cc-switch/config.json`
   - 解析 providers

2. **对比差异**：
   - 新增的 provider → 创建新文件
   - 已存在的 provider → 更新文件内容
   - 删除的 provider → 删除对应文件

3. **同步操作**：
   - 应用所有变更
   - 报告操作结果

#### 输出

**成功**：
```
[INFO] 正在读取配置文件: ~/.cc-switch/config.json
[INFO] 正在同步配置...
✓ 新增: new-config -> ~/.cc-switch/separated/config-new-config.json
✓ 更新: packycode -> ~/.cc-switch/separated/config-packycode.json
✓ 删除: old-config (配置已从 config.json 中移除)
[INFO] 配置更新完成！新增 1 个，更新 1 个，删除 1 个
```

#### 退出码

| 退出码 | 含义 |
|--------|------|
| 0 | 成功 |
| 1 | 配置文件不存在或格式错误 |
| 3 | 文件系统错误 |

---

### 3. `ccstart list`

**功能**：列出所有可用的配置名称。

#### 语法

```bash
ccstart list
```

#### 参数

无参数。

#### 行为

1. **读取目录**：
   - 扫描 `~/.cc-switch/separated/` 目录
   - 查找所有 `config-*.json` 文件

2. **提取名称**：
   - 去除 `config-` 前缀和 `.json` 后缀
   - 解码 URL 编码的字符

3. **输出列表**：
   - 每行一个配置名称
   - 包含空格或特殊字符的名称使用双引号包裹

#### 输出

**成功**：
```
packycode
"Zhipu GLM"
work
default
default-2
```

**目录不存在或为空**：
```
错误: 未找到配置，请先运行 'ccstart init'
退出码: 1
```

#### 退出码

| 退出码 | 含义 |
|--------|------|
| 0 | 成功 |
| 1 | separated/ 目录不存在或为空 |

---

### 4. `ccstart completions <SHELL>`

**功能**：生成 shell 补全脚本。

#### 语法

```bash
ccstart completions <SHELL>
```

#### 参数

| 参数 | 类型 | 必需 | 可选值 | 描述 |
|------|------|------|--------|------|
| `<SHELL>` | Enum | 是 | bash, zsh, fish, powershell | Shell 类型 |

#### 行为

1. **生成补全脚本**：
   - 根据指定的 shell 类型生成对应的补全脚本
   - 包含动态补全逻辑（读取配置列表）

2. **输出到 stdout**：
   - 完整的补全脚本内容
   - 用户可重定向到文件

#### 输出

**示例（Bash）**：
```bash
$ ccstart completions bash
_ccstart() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"

    # ... 完整的补全脚本 ...
}

complete -F _ccstart ccstart
```

**使用方法**：
```bash
# 推荐（动态补全，实时读取配置列表）
# Bash
echo "source <(COMPLETE=bash ccstart)" >> ~/.bashrc

# Zsh
echo "source <(COMPLETE=zsh ccstart)" >> ~/.zshrc

# Fish
echo "COMPLETE=fish ccstart | source" >> ~/.config/fish/config.fish

# PowerShell
$env:COMPLETE = "powershell"; echo "ccstart | Out-String | Invoke-Expression" >> $PROFILE; Remove-Item Env:\COMPLETE

# 备选（静态脚本，功能稳定但名称列表非实时）
ccstart completions bash > ~/.bash_completion.d/ccstart
ccstart completions zsh > ~/.zsh/completions/_ccstart
ccstart completions fish > ~/.config/fish/completions/ccstart.fish
ccstart completions powershell | Out-File -Encoding utf8 -FilePath $PROFILE
```

#### 退出码

| 退出码 | 含义 |
|--------|------|
| 0 | 成功 |
| 1 | 不支持的 shell 类型 |

---

### 5. `ccstart <NAME> [ARGS...]`

**功能**：使用指定配置启动 Claude CLI，并传递额外参数。

#### 语法

```bash
ccstart <NAME> [ARGS...]
```

#### 参数

| 参数 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `<NAME>` | String | 是 | 配置名称（包含空格时需双引号） |
| `[ARGS...]` | String[] | 否 | 传递给 Claude CLI 的额外参数 |

#### 行为

1. **查找配置**：
   - 编码配置名称
   - 查找 `~/.cc-switch/separated/config-<encoded-name>.json`

2. **验证配置**：
   - 检查文件是否存在
   - 验证 JSON 格式

3. **调用 Claude CLI**：
   ```bash
   claude --settings <配置文件路径> [额外参数]
   ```

4. **透传结果**：
   - 透传 Claude CLI 的 stdout
   - 透传 Claude CLI 的 stderr
   - 透传 Claude CLI 的退出码

#### 输出

**成功**：
```bash
$ ccstart packycode "help me debug"
# Claude CLI 的输出...
```

**配置不存在**：
```
错误: 配置 'nonexistent' 不存在

可用配置:
  - packycode
  - "Zhipu GLM"
  - work
  - default

提示: 使用 'ccstart list' 查看所有配置
退出码: 1
```

**Claude CLI 不存在**：
```
错误: 无法执行 claude 命令: 未找到命令
提示: 请确保 Claude CLI 已安装并在 PATH 中
退出码: 127
```

#### 退出码

| 退出码 | 含义 |
|--------|------|
| 0-255 | 透传 Claude CLI 的退出码 |
| 1 | 配置不存在 |
| 127 | Claude CLI 未安装 |

---

### 6. `ccstart --help`

**功能**：显示帮助信息。

#### 语法

```bash
ccstart --help
ccstart -h
ccstart help [SUBCOMMAND]
```

#### 输出

```
ccstart 0.2.0
Claude CLI 配置管理工具

USAGE:
    ccstart [OPTIONS] [SUBCOMMAND]
    ccstart <NAME> [ARGS...]

OPTIONS:
    -h, --help       显示帮助信息
    -V, --version    显示版本信息

SUBCOMMANDS:
    init            初始化配置分离
    update          更新分离的配置文件
    list            列出所有可用配置
    completions     生成 shell 补全脚本
    help            显示帮助信息

EXAMPLES:
    # 初始化配置
    ccstart init

    # 列出所有配置
    ccstart list

    # 使用指定配置启动 Claude
    ccstart packycode "help me debug"

    # 生成 bash 补全脚本
    ccstart completions bash > ~/.bash_completion.d/ccstart

更多信息请访问: https://github.com/user/ccstart
```

---

### 7. `ccstart --version`

**功能**：显示版本信息。

#### 语法

```bash
ccstart --version
ccstart -V
```

#### 输出

```
ccstart 0.2.0
```

---

## 全局规则

### 日志输出

**所有日志输出到 stderr**：
- `[INFO]`：一般信息
- `[WARN]`：警告信息
- `[ERROR]`：错误信息

**stdout 仅用于功能性输出**：
- `ccstart list` 的配置列表
- `ccstart completions` 的补全脚本

**用户可静默日志**：
```bash
ccstart list 2>/dev/null
```

### 错误处理

**错误信息格式**：
```
错误: <错误描述>
提示: <解决方案>
退出码: <退出码>
```

**退出码范围**：
- 0：成功
- 1：一般错误（配置不存在、格式错误等）
- 2：用户取消操作
- 3：文件系统错误（权限不足等）
- 127：Claude CLI 未安装
- 其他：透传 Claude CLI 的退出码

### 配置名称处理

**Shell 使用规则**：
- 不包含空格和特殊字符：直接使用
  ```bash
  ccstart packycode
  ```

- 包含空格或特殊字符：必须使用双引号
  ```bash
  ccstart "Zhipu GLM"
  ccstart "my/config"
  ```

### 路径规范

**配置文件路径**：
- 源配置：`~/.cc-switch/config.json`
- 分离配置目录：`~/.cc-switch/separated/`
- 分离配置文件：`~/.cc-switch/separated/config-<name>.json`

**路径展开**：
- `~` 展开为用户主目录
- 支持 Linux 和 macOS

---

## 兼容性

### 平台支持

| 平台 | 支持状态 | 备注 |
|------|---------|------|
| Linux | ✅ 完全支持 | 主要目标平台 |
| macOS | ✅ 完全支持 | 主要目标平台 |
| Windows | ❌ 不支持 | spec.md 未涵盖 |

### Shell 支持

| Shell | 补全支持 | 备注 |
|-------|---------|------|
| Bash | ✅ | 主要目标 |
| Zsh | ✅ | 主要目标 |
| Fish | ✅ | 通过 clap_complete |
| PowerShell | ✅ | 通过 clap_complete |

---

## 性能要求

| 操作 | 性能目标 | 来源 |
|------|---------|------|
| `ccstart init` | < 5秒 | spec.md SC-001 |
| `ccstart <name>` | < 1秒（启动） | spec.md SC-002 |
| `ccstart list` | < 0.5秒 | spec.md SC-003 |
| `ccstart update` | < 3秒 | spec.md SC-006 |

---

## 测试用例

### 单元测试

```rust
// 配置名称编码
#[test]
fn test_encode_config_name() {
    assert_eq!(encode_config_name("simple"), "simple");
    assert_eq!(encode_config_name("my config"), "my config");
    assert_eq!(encode_config_name("my/config"), "my%2Fconfig");
}

// 配置名称解码
#[test]
fn test_decode_config_name() {
    assert_eq!(decode_config_name("simple").unwrap(), "simple");
    assert_eq!(decode_config_name("my config").unwrap(), "my config");
    assert_eq!(decode_config_name("my%2Fconfig").unwrap(), "my/config");
}

// 重复名称处理
#[test]
fn test_handle_duplicate_names() {
    let mut counts = HashMap::new();
    assert_eq!(handle_duplicate_name("default", &mut counts), "default");
    assert_eq!(handle_duplicate_name("default", &mut counts), "default-2");
    assert_eq!(handle_duplicate_name("default", &mut counts), "default-3");
}
```

### 集成测试

```bash
# 测试初始化
ccstart init
assert_file_exists ~/.cc-switch/separated/config-packycode.json

# 测试列表
output=$(ccstart list 2>/dev/null)
assert_contains "$output" "packycode"

# 测试配置切换
ccstart packycode --version
assert_exit_code 0

# 测试不存在的配置
ccstart nonexistent
assert_exit_code 1
```

---

## 参考

- **spec.md**: 功能规格定义
- **data-model.md**: 数据模型定义
- **clap documentation**: https://docs.rs/clap/
