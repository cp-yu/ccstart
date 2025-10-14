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

## Code Style
Rust 2024 edition: Follow standard conventions

## Recent Changes
- 002-ccswitch-rust-claude: Added Rust 2024 edition

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
