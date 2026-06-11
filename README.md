# ccstart — Claude Settings 配置管理工具

> 需要配合 https://github.com/farion1231/cc-switch 使用！

ccstart 是一个用 Rust 编写的命令行工具，用于管理并快速切换 Claude CLI 和 Codex 的设置文件。它从 cc-switch 的 SQLite 数据库 (`~/.cc-switch/cc-switch.db`) 读取配置，采用 Read-Through Cache 模式按需生成缓存文件，并提供快捷命令按名称启动。

**特性：**
- **Claude 快速切换**：`ccstart <name> [args...]`
- **Codex Profile 模式**：`ccstart codex <channel>` 或 `cxstart <channel>`，使用 `-p` 保留完整 TOML 配置
- **拼音首字母匹配**：支持中文渠道名拼音匹配（如 `cxstart xc` 匹配 "小丑"）
- **自动更新**：运行时自动检测数据库变化并更新缓存，显示变更字段
- **智能同步**：`ccstart update` 同步 Claude 配置和 Codex profile，显示详细统计
- **Shell 自动补全**：动态补全支持拼音前缀匹配


## 安装

### 方式一：下载预编译二进制（推荐）

前往 GitHub Releases 页面，下载对应平台的二进制并放入 `PATH`：

- Linux x86_64：`ccstart-linux-x64` 和 `cxstart-linux-x64`
- Windows x86_64：`ccstart-windows-x64.exe` 和 `cxstart-windows-x64.exe`

Linux/macOS：
```bash
chmod +x ccstart-linux-x64 cxstart-linux-x64
sudo mv ccstart-linux-x64 /usr/local/bin/ccstart
sudo mv cxstart-linux-x64 /usr/local/bin/cxstart
```

Windows（PowerShell）：
```powershell
Move-Item .\ccstart-windows-x64.exe $Env:USERPROFILE\bin\ccstart.exe
Move-Item .\cxstart-windows-x64.exe $Env:USERPROFILE\bin\cxstart.exe
```

> 说明：Windows 版本为实验性构建；Linux/macOS 为主要目标平台。

### 方式二：从源码构建

要求：已安装 Rust（stable）
```bash
# 克隆仓库并进入目录
cargo build --release
# 二进制位于 target/release/ccstart
```


## 快速开始

### Claude 配置管理

1) 查看可用配置
```bash
ccstart list
```

2) 使用指定配置启动 Claude
```bash
ccstart packycode "help me debug"
ccstart "Zhipu GLM" "你好"  # 名称包含空格需用引号
```

### Codex 渠道管理

1) 查看可用 Codex 渠道
```bash
ccstart codex list
# 或使用快捷方式
cxstart list
```

2) 启动 Codex（支持拼音首字母匹配）
```bash
# 精确匹配
ccstart codex 小丑
cxstart 小丑

# 拼音首字母匹配（输入为纯 ASCII 字母时自动触发）
ccstart codex xc    # 匹配 "小丑"
cxstart xc          # 快捷方式

# 透传参数
cxstart packyapi "help me code"
```

3) 配置变更时自动更新
```bash
# ccstart 会自动检测 Claude 配置变化
ccstart packycode
# ✓ 更新: packycode (apiKey, model) -> ~/.cc-switch/separated/config-packycode.json

# Codex profile 会自动更新
cxstart 小丑
# ✓ Codex 更新: 小丑 -> ~/.codex/ccstart-a1b2c3d4.config.toml
```


## 命令概览

```bash
# Claude 配置管理
ccstart <NAME> [ARGS...]           # 使用指定配置启动 Claude
ccstart list                       # 列出所有 Claude 配置
ccstart update                     # 同步所有缓存

# Codex 渠道管理
ccstart codex <CHANNEL> [ARGS...]  # 启动 Codex（支持拼音匹配）
ccstart codex list                 # 列出所有 Codex 渠道
cxstart <CHANNEL> [ARGS...]        # 快捷方式，等价于 ccstart codex

# 工具命令
ccstart completions <SHELL>        # 生成 shell 补全脚本
ccstart help                       # 显示帮助信息
```

### Codex Profile 模式

- `ccstart codex <channel>` 使用 `-p` profile 模式启动，保留完整 TOML 配置：
  ```bash
  OPENAI_API_KEY=xxx codex -p ccstart-<hash> [args...]
  ```
  Profile 文件自动生成到 `~/.codex/ccstart-<hash>.config.toml`，包含 `mcp_servers`、`projects`、`reasoning_effort` 等完整配置。

- 拼音首字母匹配规则：
  - 精确匹配优先
  - 输入为纯 ASCII 字母时触发拼音匹配
  - 唯一匹配自动使用，多个匹配列出候选

### 配置同步

- `ccstart update` 同步所有配置：
  ```bash
  ccstart update
  # ✓ Claude 更新: openai (apiKey, model) -> ~/.cc-switch/separated/config-openai.json
  # ✓ Codex 新增: 小丑 -> ~/.codex/ccstart-a1b2c3d4.config.toml
  # [INFO] Claude 配置同步完成！新增 0，更新 1，未变 3，删除 0
  # [INFO] Codex profile 同步完成！新增 1，更新 0，未变 2，删除 0
  ```


## Shell 自动补全

推荐使用"动态补全"（实时读取配置列表，支持拼音匹配）：

- Bash
```bash
echo "source <(COMPLETE=bash ccstart)" >> ~/.bashrc
source ~/.bashrc
```

- Zsh
```bash
echo "source <(COMPLETE=zsh ccstart)" >> ~/.zshrc
source ~/.zshrc
```

- Fish
```bash
echo "COMPLETE=fish ccstart | source" >> ~/.config/fish/config.fish
```

- PowerShell
```powershell
$env:COMPLETE = "powershell"; echo "ccstart | Out-String | Invoke-Expression" >> $PROFILE; Remove-Item Env:\COMPLETE
```

### cxstart 补全配置（Bash）

动态补全会自动支持 `ccstart codex`，但 `cxstart` 需要额外配置：

```bash
# 创建 cxstart 补全脚本
cat > ~/.local/share/bash-completion/completions/cxstart << 'EOF'
_cxstart() {
    local cur prev
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    
    if [[ ${COMP_CWORD} -eq 1 ]]; then
        if [[ "$cur" == "l"* ]]; then
            COMPREPLY=( $(compgen -W "list" -- "$cur") )
        else
            local channels=$(ccstart codex list 2>/dev/null | tr -d '"')
            COMPREPLY=( $(compgen -W "$channels" -- "$cur") )
        fi
    fi
}
complete -F _cxstart cxstart
EOF

# 加载补全
source ~/.local/share/bash-completion/completions/cxstart
```

也可生成静态脚本（非实时，不推荐）：
```bash
ccstart completions bash > ~/.bash_completion.d/ccstart
```


## 配置与文件位置

### Claude 配置
- 数据库：`~/.cc-switch/cc-switch.db`（由 cc-switch 管理，只读访问）
- 缓存目录：`~/.cc-switch/separated/`
- 缓存文件：`~/.cc-switch/separated/config-<name>.json`

### Codex Profile
- 数据库：`~/.cc-switch/cc-switch.db`（读取 `app_type='codex'` 的 provider）
- Profile 目录：`~/.codex/`
- Profile 文件：`~/.codex/ccstart-<hash>.config.toml`（hash 为渠道名 SHA256 前 8 位）
- 内容：完整的 TOML 配置（包含 model、mcp_servers、projects 等）

### 编码规则
- Claude 配置名：保留空格，不安全字符（`/ : * ? " < > | \`）采用 URL 百分号编码
- Codex profile：使用 hash 避免中文文件名问题


## 常见问题（FAQ）

### 通用问题

- **数据库不存在**
  - 错误：`数据库不存在: ~/.cc-switch/cc-switch.db`
  - 解决：先安装并配置 cc-switch

- **配置/渠道名称不存在**
  - 程序会列出可用列表，检查名称、大小写或使用引号包裹含空格的名称

- **`claude` 或 `codex` 命令不存在**
  - 错误：`无法执行 xxx 命令`
  - 解决：确保对应 CLI 已安装并在 `PATH` 中

### Codex 相关

- **拼音匹配不工作**
  - 确认输入为纯 ASCII 字母（如 `xc`，不能是 `xc1` 或 `x c`）
  - 使用 `ccstart codex list` 查看可用渠道的实际拼音首字母

- **拼音匹配多个候选**
  - 输出：`错误: 拼音 'xc' 匹配到多个渠道：小丑, 新城`
  - 解决：使用完整渠道名或更长的拼音前缀

- **Profile 文件找不到**
  - Profile 文件会在首次运行时自动生成
  - 手动同步：`ccstart update`
  - 检查：`ls ~/.codex/ccstart-*.config.toml`

- **Codex 版本要求**
  - Profile 模式需要 Codex ≥0.134.0
  - 检查版本：`codex --version`


## 平台支持

| 平台    | 状态       | 备注               |
|---------|------------|--------------------|
| Linux   | ✅ 完全支持 | 主要目标平台       |
| macOS   | ✅ 完全支持 | 主要目标平台       |
| Windows | ⚠️ 实验性  | 构建产物可用，未广泛测试 |


## 开发

```bash
# 代码检查
cargo clippy --all-targets --all-features -- -D warnings

# 格式化
cargo fmt --all
```
