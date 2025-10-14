mod commands;
mod config;
mod error;
mod utils;

use clap::{Parser, Subcommand, CommandFactory};

#[derive(Debug, Parser)]
#[command(author, version, about = "ccstart - Claude Settings 配置管理工具", long_about = None)]
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

    /// 显式运行：等价于 `ccstart <name> [args...]`
    Run {
        /// 配置名称
        name: String,
        /// 透传给 `claude` 的参数
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}

fn main() -> error::AppResult<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init { force }) => {
            commands::init::run(force)?;
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
