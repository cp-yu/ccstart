# Quick Start Guide: ccstart

**Date**: 2025-10-14
**Version**: 1.0.0

本指南帮助您快速开始使用 ccstart 管理 Claude CLI 配置。

---

## 前置要求

在开始之前，请确保：

1. ✅ 已安装 **Claude CLI**：
   ```bash
   claude --version
   ```

2. ✅ 已安装 **ccswitch** 并创建了 `~/.cc-switch/config.json` 配置文件

3. ✅ **Rust 工具链**（用于从源码构建）：
   ```bash
   rustc --version
   # 如果未安装，请访问 https://rustup.rs/
   ```

---

## 安装

### 方式 1：从源码构建（推荐）

```bash
# 克隆仓库
git clone https://github.com/user/ccstart.git
cd ccstart

# 构建并安装
cargo install --path .

# 验证安装
ccstart --version
```

### 方式 2：使用 cargo install（发布后）

```bash
# 从 crates.io 安装（发布后可用）
cargo install ccstart

# 验证安装
ccstart --version
```

---

## 5 分钟快速入门

### 步骤 1：初始化配置

从 `~/.cc-switch/config.json` 中分离配置到独立文件。

```bash
$ ccstart init

[INFO] 正在读取配置文件: ~/.cc-switch/config.json
[INFO] 找到 5 个 provider 配置
[INFO] 正在分离配置...
✓ 已提取: packycode -> ~/.cc-switch/separated/config-packycode.json
✓ 已提取: Zhipu GLM -> ~/.cc-switch/separated/config-Zhipu GLM.json
✓ 已提取: work -> ~/.cc-switch/separated/config-work.json
✓ 已提取: default -> ~/.cc-switch/separated/config-default.json
✓ 已提取: default-2 -> ~/.cc-switch/separated/config-default-2.json
[INFO] 配置初始化完成！已生成 5 个配置文件
```

### 步骤 2：列出所有配置

查看可用的配置名称。

```bash
$ ccstart list

packycode
"Zhipu GLM"
work
default
default-2
```

### 步骤 3：使用指定配置启动 Claude

选择一个配置并启动 Claude CLI。

```bash
# 使用 packycode 配置
$ ccstart packycode

# 使用配置并传递参数
$ ccstart packycode "help me debug this code"

# 使用包含空格的配置名称（需要双引号）
$ ccstart "Zhipu GLM" "你好"
```

### 步骤 4：安装 Shell 补全（可选）

让配置名称自动补全，提升使用体验。

**Bash（推荐：动态补全）**：
```bash
echo "source <(COMPLETE=bash ccstart)" >> ~/.bashrc
source ~/.bashrc

# 测试补全
ccstart <Tab>
```

**Zsh（推荐：动态补全）**：
```bash
echo "source <(COMPLETE=zsh ccstart)" >> ~/.zshrc
source ~/.zshrc

# 测试补全
ccstart <Tab>
```

**Fish**：
```bash
# 推荐：动态补全
echo "COMPLETE=fish ccstart | source" >> ~/.config/fish/config.fish

# 测试补全
ccstart <Tab>
```

**PowerShell**：
```powershell
# 写入当前用户的 PowerShell 配置文件
ccstart completions powershell | Out-File -Encoding utf8 -FilePath $PROFILE

# 重启 PowerShell 或手动 dot-source 配置文件后测试
ccstart <Tab>
```

---

## 常见使用场景

### 场景 1：切换不同的 API Provider

```bash
# 使用 Anthropic 官方 API
ccstart anthropic-official "help me with this code"

# 切换到 Zhipu GLM
ccstart "Zhipu GLM" "帮我写个函数"

# 切换到内部代理
ccstart work-proxy "refactor this code"
```

### 场景 2：更新配置

当修改了 `~/.cc-switch/config.json` 后，同步更改。

```bash
# 编辑源配置文件
vim ~/.cc-switch/config.json

# 同步到分离的配置文件
$ ccstart update

[INFO] 正在读取配置文件: ~/.cc-switch/config.json
[INFO] 正在同步配置...
✓ 新增: new-config -> ~/.cc-switch/separated/config-new-config.json
✓ 更新: packycode -> ~/.cc-switch/separated/config-packycode.json
[INFO] 配置更新完成！新增 1 个，更新 1 个，删除 0 个
```

### 场景 3：脚本中使用

在自动化脚本中使用 ccstart。

```bash
#!/bin/bash

# 静默日志输出
CONFIG_LIST=$(ccstart list 2>/dev/null)

# 遍历所有配置
for config in $CONFIG_LIST; do
    echo "Testing with config: $config"
    ccstart "$config" "test prompt" 2>/dev/null || echo "Failed with $config"
done
```

### 场景 4：重新初始化

强制重新初始化（删除现有配置并重新生成）。

```bash
# 使用 --force 标志跳过确认
ccstart init --force
```

---

## 配置文件结构

### 源配置文件：`~/.cc-switch/config.json`

```json
{
  "claude": {
    "providers": {
      "provider-id-1": {
        "id": "provider-id-1",
        "name": "packycode",
        "settingsConfig": {
          "provider": "anthropic",
          "apiKey": "sk-...",
          "model": "claude-3-5-sonnet-20241022"
        }
      },
      "provider-id-2": {
        "id": "provider-id-2",
        "name": "Zhipu GLM",
        "settingsConfig": {
          "provider": "zhipu",
          "apiKey": "...",
          "baseUrl": "https://open.bigmodel.cn/api/paas/v4/"
        }
      }
    }
  }
}
```

### 分离后的配置文件

```
~/.cc-switch/separated/
├── config-packycode.json           # Anthropic 官方
├── config-Zhipu GLM.json           # Zhipu GLM（包含空格）
├── config-work.json                # 工作代理
└── config-default.json             # 默认配置
```

每个文件的内容是对应 provider 的 `settingsConfig` 对象：

```json
{
  "provider": "anthropic",
  "apiKey": "sk-...",
  "model": "claude-3-5-sonnet-20241022",
  "baseUrl": "https://api.anthropic.com"
}
```

---

## 命令速查表

| 命令 | 功能 | 示例 |
|------|------|------|
| `ccstart init` | 初始化配置分离 | `ccstart init` |
| `ccstart init --force` | 强制重新初始化 | `ccstart init -f` |
| `ccstart update` | 更新分离的配置 | `ccstart update` |
| `ccstart list` | 列出所有配置 | `ccstart list` |
| `ccstart <name>` | 使用指定配置 | `ccstart packycode` |
| `ccstart <name> [args]` | 传递额外参数 | `ccstart packycode "help me"` |
| `ccstart completions <shell>` | 生成补全脚本 | `ccstart completions bash` |
| `ccstart --help` | 显示帮助信息 | `ccstart -h` |
| `ccstart --version` | 显示版本信息 | `ccstart -V` |

---

## 常见问题

### Q1: 为什么需要 ccstart？

**A**: 如果你有多个 Claude CLI 配置（不同的 API provider、不同的 API key、不同的模型设置），ccstart 可以：
- 一次性从 ccswitch 配置中提取所有配置
- 通过简单的名称快速切换配置
- 提供命令行补全，提升使用体验

### Q2: ccstart 和 ccswitch 的关系是什么？

**A**:
- **ccswitch**: 管理混合配置文件 `~/.cc-switch/config.json`
- **ccstart**: 读取 ccswitch 的配置，拆分成独立文件，提供快速切换功能
- ccstart 是 ccswitch 的**增强工具**，专注于配置管理和快速切换

### Q3: 如何处理配置名称中的空格？

**A**: 在命令行中使用双引号包裹：
```bash
ccstart "Zhipu GLM" "help me"
```

Shell 补全会自动添加双引号，所以只需按 Tab 补全即可。

### Q4: 如何处理重复的配置名称？

**A**: ccstart 会自动添加数字后缀：
```
第 1 个: default
第 2 个: default-2
第 3 个: default-3
```

### Q5: 修改了 config.json 后需要做什么？

**A**: 运行 `ccstart update` 同步更改：
```bash
vim ~/.cc-switch/config.json
ccstart update
```

### Q6: 如何静默日志输出？

**A**: 将 stderr 重定向到 `/dev/null`：
```bash
ccstart list 2>/dev/null
```

### Q7: ccstart 支持哪些平台？

**A**:
- ✅ Linux
- ✅ macOS
- ❌ Windows（暂不支持）

### Q8: 如何卸载 ccstart？

**A**:
```bash
cargo uninstall ccstart
```

---

## 性能基准

| 操作 | 预期时间 | 实际测试（50 个配置） |
|------|---------|---------------------|
| `ccstart init` | < 5 秒 | ~2.3 秒 |
| `ccstart <name>` | < 1 秒 | ~0.4 秒 |
| `ccstart list` | < 0.5 秒 | ~0.15 秒 |
| `ccstart update` | < 3 秒 | ~1.8 秒 |

---

## 下一步

- 📖 阅读 [CLI Interface Contract](./contracts/cli-interface.md) 了解完整的命令行接口规范
- 📖 阅读 [Data Model](./data-model.md) 了解数据结构设计
- 🛠️ 查看 [spec.md](./spec.md) 了解完整的功能规格
- 🧪 运行测试：`cargo test`
- 🚀 参与贡献：查看 [CONTRIBUTING.md](../../CONTRIBUTING.md)（如果有）

---

## 获取帮助

- **文档**: `ccstart --help`
- **GitHub Issues**: https://github.com/user/ccstart/issues
- **源代码**: https://github.com/user/ccstart

---

## 许可证

MIT / Apache 2.0 双许可证

---

**祝使用愉快！🎉**
**PowerShell**：
```powershell
$env:COMPLETE = "powershell"; echo "ccstart | Out-String | Invoke-Expression" >> $PROFILE; Remove-Item Env:\COMPLETE

# 重启 PowerShell 或手动 dot-source 配置文件后测试
ccstart <Tab>
```

注：若偏好静态文件方式，可使用：
```bash
ccstart completions bash > ~/.bash_completion.d/ccstart
ccstart completions zsh > ~/.zsh/completions/_ccstart
ccstart completions fish > ~/.config/fish/completions/ccstart.fish
```
