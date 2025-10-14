mod commands;
mod config;
mod error;
mod utils;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{CompleteEnv, Shell};
use std::ffi::OsStr;

#[derive(Debug, Parser)]
#[command(
    author, 
    version, 
    about = "ccstart - Claude Settings 配置管理工具", 
    long_about = None,
    after_help = "示例:\n  \
        ccstart init                    # 初始化配置\n  \
        ccstart list                    # 列出所有配置\n  \
        ccstart packycode               # 使用 packycode 配置启动 Claude\n  \
        ccstart packycode \"help me\"     # 使用配置并传递参数\n  \
        ccstart \"Zhipu GLM\" \"你好\"      # 使用包含空格的配置名称\n  \
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
    /// 初始化：从 ~/.cc-switch/config.json 分离出独立配置
    Init {
        /// 跳过覆盖确认
        #[arg(long)]
        force: bool,
    },

    /// 列出所有可用的配置名称
    List,

    /// 更新配置：重新同步 config.json 的变更到 separated/ 目录
    Update,

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

/// 应用主入口：返回进程退出码
fn run_app() -> error::AppResult<i32> {
    // 在最开始拦截 shell 动态补全请求（由 clap_complete 通过环境变量触发），
    // 若捕获到补全请求，内部将输出候选并直接 exit(0)。
    CompleteEnv::with_factory(Cli::command).complete();

    let cli = Cli::parse();

    let exit_code = match cli.command {
        Some(Commands::Init { force }) => {
            commands::init::run(force)?;
            0
        }
        Some(Commands::List) => {
            commands::list::list_configs()?;
            0
        }
        Some(Commands::Update) => {
            commands::update::run()?;
            0
        }
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
                // 无参数，显示帮助
                Cli::command().print_help().ok();
                println!();
                0
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
            eprintln!("提示: 使用 --help 查看用法，或先运行 'ccstart init' 初始化配置");
            std::process::exit(1);
        }
    }
}

/// 动态补全：返回配置名称候选
pub fn config_name_completer(current: &OsStr) -> Vec<clap_complete::engine::CompletionCandidate> {
    let mut out = Vec::new();

    // 当前输入（尽力解码为 UTF-8）；无法解码则返回所有候选
    let needle = current.to_string_lossy().to_string();
    let lower = needle.to_lowercase();

    if let Ok(list) = crate::config::manager::ConfigManager::list_configs() {
        for name in list {
            // 不区分大小写前缀匹配；空输入返回全部
            if lower.is_empty() || name.to_lowercase().starts_with(&lower) {
                out.push(clap_complete::engine::CompletionCandidate::new(name));
            }
        }
    }

    out
}
