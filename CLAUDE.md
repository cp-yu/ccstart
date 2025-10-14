# ccstart Development Guidelines

Auto-generated from all feature plans. Last updated: 2025-10-14

## Active Technologies
- Rust 2024 edition (002-ccswitch-rust-claude)

## Project Structure
```
src/
tests/
```

## Commands

Rust (ccstart):
- Build: `cargo build --release`
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`
- Run: `cargo run -- <args...>`

ccstart CLI:
- 初始化配置：`ccstart init [--force]`
- 列出配置：`ccstart list`
- 使用配置运行 Claude：`ccstart <name> [args...]`
- 生成补全脚本（静态）：`ccstart completions <bash|zsh|fish|powershell>`

补全安装（推荐：动态补全，实时读取配置列表）
- Bash: `echo "source <(COMPLETE=bash ccstart)" >> ~/.bashrc`
- Zsh: `echo "source <(COMPLETE=zsh ccstart)" >> ~/.zshrc`
- Fish: `echo "COMPLETE=fish ccstart | source" >> ~/.config/fish/config.fish`
- PowerShell: `$env:COMPLETE = "powershell"; echo "ccstart | Out-String | Invoke-Expression" >> $PROFILE; Remove-Item Env:\COMPLETE`

补全安装（备选：静态脚本，非实时）
- Bash: `ccstart completions bash > ~/.bash_completion.d/ccstart`
- Zsh: `ccstart completions zsh > ~/.zsh/completions/_ccstart`
- Fish: `ccstart completions fish > ~/.config/fish/completions/ccstart.fish`
- PowerShell: `ccstart completions powershell | Out-File -Encoding utf8 -FilePath $PROFILE`

## Release

发布到 GitHub Release：
- 创建版本标签并推送，例如：
  - `git tag v0.1.0 && git push origin v0.1.0`
- GitHub Actions 工作流 `.github/workflows/release.yml` 将为以下平台构建发布产物：
  - Linux x86_64 (`ccstart-linux-x64`)
  - Windows x86_64 (`ccstart-windows-x64.exe`)
- 工作流会上传构建产物与对应的 `SHA256` 校验文件，并自动创建 Release（推送标签时触发）。
- 也可通过 `Actions -> Release -> Run workflow` 手动触发（`workflow_dispatch`）。

## Code Style
Rust 2024 edition: Follow standard conventions

## Recent Changes
- 002-ccswitch-rust-claude: Added Rust 2024 edition

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
