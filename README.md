# ccstart — Claude Settings 配置管理工具

> 需要配合 https://github.com/farion1231/cc-switch 使用！

ccstart 是一个用 Rust 编写的命令行工具，用于管理并快速切换 Claude CLI 的设置文件。它从 cc-switch 的 SQLite 数据库 (`~/.cc-switch/cc-switch.db`) 读取配置，采用 Read-Through Cache 模式按需生成缓存文件到 `~/.cc-switch/separated/` 目录，并提供快捷命令按名称启动 Claude。

**特性：**
- 快速切换：`ccstart <name> [args...]`
- 自动更新：运行时自动检测数据库变化并更新缓存，显示变更字段
- 智能同步：`ccstart update` 显示新增/更新/未变/删除的详细统计
- 查看配置：`ccstart list`
- Shell 自动补全：`ccstart completions <shell>` 或推荐启用动态补全


## 安装

### 方式一：下载预编译二进制（推荐）

前往 GitHub Releases 页面，下载对应平台的二进制并放入 `PATH`：

- Linux x86_64：`ccstart-linux-x64`
- Windows x86_64：`ccstart-windows-x64.exe`

Linux/macOS：
```bash
chmod +x ccstart-linux-x64
sudo mv ccstart-linux-x64 /usr/local/bin/ccstart
```

Windows（PowerShell）：
```powershell
# 假设已下载到当前目录
Move-Item .\ccstart-windows-x64.exe $Env:USERPROFILE\bin\ccstart.exe
# 将 %USERPROFILE%\bin 加入 PATH 后新开终端生效
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

1) 查看可用配置
```bash
ccstart list
# 可能输出（每行一个）：
# packycode
# "Zhipu GLM"
# work
# default
```

2) 使用指定配置启动 Claude
```bash
# 直接使用名称（无空格/特殊字符）
ccstart packycode "help me debug"

# 名称包含空格/特殊字符时需用双引号
ccstart "Zhipu GLM" "你好"
```

3) 配置变更时自动更新
```bash
# 当 cc-switch 修改了配置后，直接运行即可
# ccstart 会自动检测变化并显示变更的字段
ccstart packycode
# [INFO] 配置已更新: packycode (apiKey, model 已变更)
# [INFO] 使用配置: ~/.cc-switch/separated/config-packycode.json
```


## 命令概览

```bash
ccstart [OPTIONS] [SUBCOMMAND]
ccstart <NAME> [ARGS...]

SUBCOMMANDS:
  update         智能同步所有缓存（显示新增/更新/未变/删除统计）
  list           列出所有可用配置
  completions    生成 shell 补全脚本（静态）
  help           显示帮助信息
```

- `ccstart <name> [args...]` 会执行：
  ```bash
  claude --settings ~/.cc-switch/separated/config-<encoded-name>.json [args...]
  ```
  并透传 Claude 的退出码。运行时自动检测数据库变化并按需更新缓存。

- `ccstart update` 同步所有配置：
  ```bash
  ccstart update
  # [INFO] 正在从数据库同步配置...
  # ✓ 更新: openai (apiKey, model) -> ~/.cc-switch/separated/config-openai.json
  # ✓ 新增: newconfig -> ~/.cc-switch/separated/config-newconfig.json
  # [INFO] 配置同步完成！新增 1，更新 1，未变 3，删除 0
  ```


## Shell 自动补全

推荐使用"动态补全"（实时读取配置列表）：

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

也可生成静态脚本（非实时）：
```bash
ccstart completions bash > ~/.bash_completion.d/ccstart
ccstart completions zsh > ~/.zsh/completions/_ccstart
ccstart completions fish > ~/.config/fish/completions/ccstart.fish
```


## 配置与文件位置

- 数据库：`~/.cc-switch/cc-switch.db`（由 cc-switch 管理，只读访问）
- 缓存目录：`~/.cc-switch/separated/`
- 缓存文件：`~/.cc-switch/separated/config-<name>.json`
- 名称编码：保留空格，其他不安全字符（如 `/ : * ? " < > | \`）采用 URL 百分号编码


## 常见问题（FAQ）

- 数据库不存在
  - 输出：`错误: 数据库不存在: ~/.cc-switch/cc-switch.db`，请先安装并配置 cc-switch

- 配置名称不存在
  - 程序会给出"可用配置列表"；请检查名称、大小写或使用引号包裹含空格的名称

- `claude` 命令不存在
  - 输出：`错误: 无法执行 claude 命令: 未找到命令`，请确保 Claude CLI 已安装并在 `PATH` 中


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
