use crate::error::AppResult;
use clap_complete::{generate, Shell};
use std::io;

/// 生成 shell 补全脚本
pub fn run(shell: Shell) -> AppResult<()> {
    // 使用顶层提供的构建函数，避免直接依赖私有类型
    let mut cmd = crate::build_cli_command();
    let bin_name = cmd.get_name().to_string();
    
    generate(shell, &mut cmd, bin_name, &mut io::stdout());
    // 在 stderr 给出安装说明，避免污染 stdout 的脚本内容
    eprintln!("[INFO] 已生成 {:?} 补全脚本到 stdout", shell);
    match shell {
        Shell::Bash => {
            eprintln!("安装示例 (二选一)：");
            eprintln!("  - 持久化: ccstart completions bash > ~/.bash_completion.d/ccstart");
            eprintln!("  - 动态加载: echo 'eval \"$(ccstart completions bash)\"' >> ~/.bashrc");
        }
        Shell::Zsh => {
            eprintln!("安装示例 (二选一)：");
            eprintln!("  - 写入 rc: ccstart completions zsh > ~/.zshrc");
            eprintln!("  - 建议方式: ccstart completions zsh > ~/.zsh/completions/_ccstart && source ~/.zshrc");
        }
        Shell::Fish => {
            eprintln!("安装示例：ccstart completions fish > ~/.config/fish/completions/ccstart.fish");
        }
        Shell::PowerShell => {
            eprintln!("安装示例：ccstart completions powershell | Out-File -Encoding utf8 -FilePath $PROFILE");
        }
        _ => {
            eprintln!("提示：将 stdout 内容保存到相应 shell 的补全加载路径");
        }
    }

    Ok(())
}
