use crate::config::cache::CacheManager;
use crate::config::diff::CacheResult;
use crate::db::Database;
use crate::error::AppResult;
use anyhow::Context;
#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
use std::process::Command;

pub fn run(name: &str, args: &[String]) -> AppResult<i32> {
    // 1. 打开数据库
    let db = Database::open()?;

    // 2. 查询 provider
    let provider = match db.providers().get_by_name(name)? {
        Some(p) => p,
        None => {
            eprintln!("错误: 未找到配置 '{}'", name);
            if let Ok(names) = db.providers().list_names() {
                if names.is_empty() {
                    eprintln!("提示: 数据库中没有 Claude 配置，请先在 cc-switch 中添加。");
                } else {
                    eprintln!("提示: 可用配置如下：");
                    for n in names {
                        eprintln!("  - {}", n);
                    }
                }
            }
            return Ok(1);
        }
    };

    // 3. 确保缓存文件存在（懒加载 + 哈希比较）
    let cache = CacheManager::new()?;
    let result = cache.ensure_cached(&provider)?;

    // 4. 显示缓存状态反馈
    match &result {
        CacheResult::Created(path) => {
            eprintln!("[INFO] 新配置已缓存: {}", provider.name);
            eprintln!("[INFO] 使用配置: {}", path.display());
        }
        CacheResult::Updated { path, changed_fields } => {
            let fields = changed_fields.join(", ");
            eprintln!("[INFO] 配置已更新: {} ({} 已变更)", provider.name, fields);
            eprintln!("[INFO] 使用配置: {}", path.display());
        }
        CacheResult::Unchanged(path) => {
            eprintln!("[INFO] 使用配置: {}", path.display());
        }
    }

    // 5. 执行 claude
    let settings_path = result.path();
    let mut cmd = Command::new("claude");
    cmd.arg("--settings").arg(settings_path);
    for a in args {
        cmd.arg(a);
    }

    let status = cmd
        .status()
        .with_context(|| "执行 'claude' 命令失败，请确认已安装并在 PATH 中")?;

    if let Some(code) = status.code() {
        Ok(code)
    } else {
        #[cfg(unix)]
        {
            let sig = status.signal().unwrap_or_default();
            eprintln!("[WARN] 进程被信号终止: {}", sig);
            Ok(128 + sig)
        }
        #[cfg(not(unix))]
        {
            eprintln!("[WARN] 子进程未返回退出码，按失败处理");
            Ok(1)
        }
    }
}
