mod commands;
mod config;
mod error;
mod utils;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;

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
        ccstart completions bash        # 生成 bash 补全脚本"
)]
struct Cli {
    /// 当未指定子命令时，作为 `ccstart <name> [args...]` 的 <name>
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
        name: String,
        /// 透传给 `claude` 的参数
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}

/// 构建 CLI `Command`（供补全/生成脚本等使用）
pub fn build_cli_command() -> clap::Command { Cli::command() }

fn main() -> error::AppResult<()> {
    // 在最开始拦截 shell 动态补全请求（由 clap_complete 通过环境变量触发）
    // 若捕获到补全请求，将直接输出补全结果并退出。
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init { force }) => {
            commands::init::run(force)?;
        }
        Some(Commands::List) => {
            commands::list::list_configs()?;
        }
        Some(Commands::Update) => {
            commands::update::run()?;
        }
        Some(Commands::Completions { shell }) => {
            commands::completions::run(shell)?;
        }
        Some(Commands::Run { name, args }) => {
            let code = commands::run::run(&name, &args)?;
            std::process::exit(code);
        }
        None => {
            // 无子命令：尝试作为 `ccstart <name> [args...]`
            if let Some(name) = cli.name {
                let code = commands::run::run(&name, &cli.args)?;
                std::process::exit(code);
            } else {
                // 无参数，显示帮助
                Cli::command().print_help().ok();
                println!();
            }
        }
    }

    Ok(())
}
