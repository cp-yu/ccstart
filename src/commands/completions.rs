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
    eprintln!("[推荐] 动态补全（实时读取配置）安装示例：");
    eprintln!("  - Bash: echo \"source <(COMPLETE=bash ccstart)\" >> ~/.bashrc");
    eprintln!("  - Zsh:  echo \"source <(COMPLETE=zsh ccstart)\" >> ~/.zshrc");
    eprintln!("  - Fish: echo \"COMPLETE=fish ccstart | source\" >> ~/.config/fish/config.fish");
    eprintln!("  - PowerShell: $env:COMPLETE = 'powershell'; echo \"ccstart | Out-String | Invoke-Expression\" >> $PROFILE; Remove-Item Env:\\COMPLETE");
    eprintln!("[备选] 静态脚本（非实时）：ccstart completions <shell> > <path>");

    Ok(())
}
