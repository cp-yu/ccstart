use crate::config::manager::ConfigManager;
use crate::error::AppResult;
use anyhow::Context;
#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
use std::process::Command;

pub fn run(name: &str, args: &[String]) -> AppResult<i32> {
    let settings_path = match ConfigManager::find_config_file(name) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("错误: 未找到配置 '{}': {}", name, e);
            if let Ok(list) = ConfigManager::list_configs() {
                if list.is_empty() {
                    eprintln!("提示: 先运行 'ccstart init' 生成分离配置。");
                } else {
                    eprintln!("提示: 可用配置如下：");
                    for n in list {
                        eprintln!("  - {}", n);
                    }
                }
            } else {
                eprintln!("提示: 先运行 'ccstart init' 生成分离配置，或检查名称是否正确。");
            }
            return Ok(1);
        }
    };

    eprintln!("[INFO] 使用配置: {}", settings_path.display());

    let mut cmd = Command::new("claude");
    cmd.arg("--settings").arg(&settings_path);
    for a in args {
        cmd.arg(a);
    }

    let status = cmd
        .status()
        .with_context(|| "执行 'claude' 命令失败，请确认已安装并在 PATH 中")?;

    if let Some(code) = status.code() {
        Ok(code)
    } else {
        // 子进程未返回常规退出码
        #[cfg(unix)]
        {
            // Unix: 可能被信号终止
            let sig = status.signal().unwrap_or_default();
            eprintln!("[WARN] 进程被信号终止: {}", sig);
            Ok(128 + sig)
        }
        #[cfg(not(unix))]
        {
            // Windows: 无信号概念，按失败处理
            eprintln!("[WARN] 子进程未返回退出码（非 Unix 平台），按失败处理");
            Ok(1)
        }
    }
}
