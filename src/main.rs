mod commands;
mod config;
mod db;
mod error;
mod tui;
mod utils;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{CompleteEnv, Shell};
use std::ffi::OsStr;

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about = "ccstart - Claude Settings 配置管理工具 (SQLite 版)",
    long_about = None,
    after_help = "示例:\n  \
        ccstart list                    # 列出所有配置\n  \
        ccstart packycode               # 使用 packycode 配置启动 Claude\n  \
        ccstart packycode \"help me\"     # 使用配置并传递参数\n  \
        ccstart \"Zhipu GLM\" \"你好\"      # 使用包含空格的配置名称\n  \
        ccstart update                  # 强制刷新所有缓存\n  \
        ccstart completions bash        # 生成 bash 补全脚本\n\n  \
        提示: 推荐启用动态补全 (实时读取配置列表)\n  \
        Bash: echo \"source <(COMPLETE=bash ccstart)\" >> ~/.bashrc\n  \
        Zsh:  echo \"source <(COMPLETE=zsh ccstart)\" >> ~/.zshrc\n  \
        Fish: echo \"COMPLETE=fish ccstart | source\" >> ~/.config/fish/config.fish\n  \
        PowerShell: $env:COMPLETE = 'powershell'; echo \"ccstart | Out-String | Invoke-Expression\" >> $PROFILE; Remove-Item Env:\\\\COMPLETE"
)]
struct Cli {
    /// 当未指定子命令时，作为 `ccstart <name> [args...]` 的 <name>
    #[arg(add = clap_complete::engine::ArgValueCompleter::new(crate::config_name_completer))]
    name: Option<String>,

    /// 透传给底层 `claude` 命令的参数，当使用 `ccstart <name> [args...]` 时生效
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,

    /// 子命令
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// 列出所有可用的配置名称
    List,

    /// 更新配置：强制刷新所有缓存文件
    Update,

    /// 使用 Codex 渠道启动 codex；`ccstart codex list` 列出渠道
    #[command(disable_help_flag = true)]
    Codex {
        /// Codex 渠道名称
        #[arg(add = clap_complete::engine::ArgValueCompleter::new(crate::codex_channel_completer))]
        channel: Option<String>,
        /// 透传给 `codex` 的参数
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// 生成 shell 补全脚本
    Completions {
        /// Shell 类型 (bash, zsh, fish, powershell, elvish)
        shell: Shell,
    },

    /// 显式运行：等价于 `ccstart <name> [args...]`
    Run {
        /// 配置名称
        #[arg(add = clap_complete::engine::ArgValueCompleter::new(crate::config_name_completer))]
        name: String,
        /// 透传给 `claude` 的参数
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}

/// 构建 CLI `Command`（供补全/生成脚本等使用）
pub fn build_cli_command() -> clap::Command { Cli::command() }

/// 检测 argv[0] 是否为 cxstart，是则走 codex 快捷路径
fn is_cxstart() -> bool {
    std::env::args_os()
        .next()
        .is_some_and(|a| is_cxstart_name(&a))
}

fn is_cxstart_name(arg0: &std::ffi::OsStr) -> bool {
    std::path::Path::new(arg0)
        .file_name()
        .is_some_and(|name| name == "cxstart" || name == "cxstart.exe")
}

/// cxstart 快捷入口：将参数视为 codex 子命令
fn run_cxstart() -> error::AppResult<i32> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.first().map(|s| s.as_str()) {
        Some("list") if args.len() == 1 => {
            commands::codex::list_channels()?;
            Ok(0)
        }
        Some(channel) => {
            let passthrough = args[1..].to_vec();
            commands::codex::run(channel, &passthrough)
        }
        None => {
            let db = db::Database::open()?;
            let names = db.providers().list_names("codex")?;

            if names.is_empty() {
                eprintln!("错误: 数据库中没有 Codex 渠道");
                eprintln!("提示: 请先在 cc-switch 中添加 codex 配置");
                return Ok(1);
            }

            match tui::select(&names, "选择 Codex 渠道")? {
                Some(channel) => commands::codex::run(&channel, &[]),
                None => Ok(0),
            }
        }
    }
}

/// 应用主入口：返回进程退出码
fn run_app() -> error::AppResult<i32> {
    if is_cxstart() {
        return run_cxstart();
    }

    // 在最开始拦截 shell 动态补全请求
    CompleteEnv::with_factory(Cli::command).complete();

    let cli = Cli::parse();

    let exit_code = match cli.command {
        Some(Commands::List) => {
            commands::list::list_configs()?;
            0
        }
        Some(Commands::Update) => {
            commands::update::run()?;
            0
        }
        Some(Commands::Codex { channel, args }) => match channel.as_deref() {
            Some("list") if args.is_empty() => {
                commands::codex::list_channels()?;
                0
            }
            Some(channel) => commands::codex::run(channel, &args)?,
            None => {
                let db = db::Database::open()?;
                let names = db.providers().list_names("codex")?;

                if names.is_empty() {
                    eprintln!("错误: 数据库中没有 Codex 渠道");
                    eprintln!("提示: 请先在 cc-switch 中添加 codex 配置");
                    return Ok(1);
                }

                match tui::select(&names, "选择 Codex 渠道")? {
                    Some(channel) => commands::codex::run(&channel, &args)?,
                    None => 0,
                }
            }
        },
        Some(Commands::Completions { shell }) => {
            commands::completions::run(shell)?;
            0
        }
        Some(Commands::Run { name, args }) => commands::run::run(&name, &args)?,
        None => {
            // 无子命令：尝试作为 `ccstart <name> [args...]`
            if let Some(name) = cli.name {
                commands::run::run(&name, &cli.args)?
            } else {
                // 无参数，启动 TUI 选择器
                let db = db::Database::open()?;
                let names = db.providers().list_names("claude")?;

                if names.is_empty() {
                    eprintln!("错误: 数据库中没有 Claude 配置");
                    eprintln!("提示: 请先在 cc-switch 中添加配置");
                    return Ok(1);
                }

                match tui::select(&names, "选择 Claude 配置")? {
                    Some(name) => commands::run::run(&name, &[])?,
                    None => 0,
                }
            }
        }
    };

    Ok(exit_code)
}

fn main() {
    match run_app() {
        Ok(code) => std::process::exit(code),
        Err(e) => {
            eprintln!("错误: {}", e);
            eprintln!("提示: 使用 --help 查看用法");
            std::process::exit(1);
        }
    }
}

/// 动态补全：返回配置名称候选（从 SQLite 查询）
pub fn config_name_completer(current: &OsStr) -> Vec<clap_complete::engine::CompletionCandidate> {
    let mut out = Vec::new();

    let needle = current.to_string_lossy().to_string();
    let lower = needle.to_lowercase();

    // 从 SQLite 查询
    if let Ok(db) = crate::db::Database::open()
        && let Ok(names) = db.providers().list_names("claude")
    {
        for name in names {
            if lower.is_empty() || name.to_lowercase().starts_with(&lower) {
                out.push(clap_complete::engine::CompletionCandidate::new(name));
            }
        }
    }

    out
}

/// 补全匹配逻辑：返回匹配前缀的渠道名（含拼音首字母）
fn complete_channels(prefix: &str, names: &[String]) -> Vec<String> {
    let lower = prefix.to_lowercase();
    let mut out = Vec::new();

    for name in names {
        if lower.is_empty() || name.to_lowercase().starts_with(&lower) {
            out.push(name.clone());
        }
    }

    if !lower.is_empty() && lower.bytes().all(|b| b.is_ascii_alphabetic()) {
        for name in names {
            let initials = crate::utils::pinyin::pinyin_initials(name);
            if initials.starts_with(&lower) && !out.contains(name) {
                out.push(name.clone());
            }
        }
    }

    out
}

/// 动态补全：返回 Codex 渠道候选（从 SQLite 查询，支持拼音首字母前缀）
pub fn codex_channel_completer(current: &OsStr) -> Vec<clap_complete::engine::CompletionCandidate> {
    let needle = current.to_string_lossy().to_string();

    if let Ok(db) = crate::db::Database::open()
        && let Ok(names) = db.providers().list_names("codex")
    {
        complete_channels(&needle, &names)
            .into_iter()
            .map(clap_complete::engine::CompletionCandidate::new)
            .collect()
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;

    #[test]
    fn cxstart_routing() {
        assert!(is_cxstart_name(OsStr::new("cxstart")));
        assert!(is_cxstart_name(OsStr::new("cxstart.exe")));
        assert!(is_cxstart_name(OsStr::new("/usr/local/bin/cxstart")));
        assert!(is_cxstart_name(OsStr::new("./cxstart")));
        assert!(!is_cxstart_name(OsStr::new("ccstart")));
        assert!(!is_cxstart_name(OsStr::new("/bin/ccstart")));
    }

    #[test]
    fn completion_pinyin() {
        let channels = vec!["小丑".into(), "packyapi".into(), "钟阮".into()];
        let results = complete_channels("xc", &channels);
        assert!(results.contains(&"小丑".to_string()));
        assert!(!results.contains(&"packyapi".to_string()));
    }

    #[test]
    fn completion_prefix_normal() {
        let channels = vec!["packyapi".into(), "packycode".into(), "小丑".into()];
        let results = complete_channels("pa", &channels);
        assert!(results.contains(&"packyapi".to_string()));
        assert!(results.contains(&"packycode".to_string()));
        assert!(!results.contains(&"小丑".to_string()));
    }

    #[test]
    fn completion_empty_returns_all() {
        let channels = vec!["packyapi".into(), "小丑".into()];
        let results = complete_channels("", &channels);
        assert_eq!(results.len(), 2);
    }
}
