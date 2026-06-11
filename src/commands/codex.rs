use crate::commands::codex_resolve::{ResolveResult, resolve_channel};
use crate::config::codex_cache::CodexCacheManager;
use crate::db::Database;
use crate::error::AppResult;
use anyhow::Context;
#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
use std::process::Command;

const AUTH_ENV_KEY: &str = "OPENAI_API_KEY";

pub fn list_channels() -> AppResult<()> {
    let db = Database::open()?;
    let names = db.providers().list_names("codex")?;

    if names.is_empty() {
        eprintln!("错误: 数据库中没有 Codex 渠道");
        eprintln!("提示: 请先在 cc-switch 中添加 codex 配置");
        std::process::exit(1);
    }

    for name in names {
        if needs_quoting(&name) {
            println!("\"{}\"", name);
        } else {
            println!("{}", name);
        }
    }

    Ok(())
}

pub fn run(channel: &str, args: &[String]) -> AppResult<i32> {
    let db = Database::open()?;
    let names = db.providers().list_names("codex")?;

    let resolved_name = match resolve_channel(channel, &names) {
        ResolveResult::Exact(name) | ResolveResult::PinyinUnique(name) => name,
        ResolveResult::PinyinCollision(candidates) => {
            eprintln!("错误: 拼音 '{}' 匹配到多个渠道：", channel);
            for c in &candidates {
                eprintln!("  - {}", c);
            }
            return Ok(1);
        }
        ResolveResult::NotFound => {
            eprintln!("错误: 未找到 Codex 渠道 '{}'", channel);
            if names.is_empty() {
                eprintln!("提示: 数据库中没有 Codex 渠道，请先在 cc-switch 中添加。");
            } else {
                eprintln!("提示: 可用 Codex 渠道如下：");
                for n in &names {
                    eprintln!("  - {}", n);
                }
            }
            return Ok(1);
        }
    };

    let provider = db
        .providers()
        .get_by_name("codex", &resolved_name)?
        .ok_or_else(|| anyhow::anyhow!("渠道 '{}' 查询失败", resolved_name))?;

    let settings = &provider.settings_config;

    let api_key = settings
        .pointer("/auth/OPENAI_API_KEY")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("settings_config 缺少 auth.OPENAI_API_KEY"))?;

    let toml_content = settings
        .get("config")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("settings_config 缺少 config"))?;

    let cache = CodexCacheManager::new()?;
    cache.ensure_cached(&resolved_name, toml_content)?;
    let auth_guard = cache.lock_and_write_auth(api_key)?;
    let profile_name = CodexCacheManager::profile_name(&resolved_name);

    let mut cmd = Command::new("codex");
    cmd.env(AUTH_ENV_KEY, api_key)
        .arg("-p")
        .arg(&profile_name);
    for a in args {
        cmd.arg(a);
    }

    let mut child = cmd
        .spawn()
        .with_context(|| "执行 'codex' 命令失败，请确认已安装并在 PATH 中")?;

    // codex 启动后约 1s 内完成 auth.json 读取，随后还原文件并释放锁
    std::thread::sleep(std::time::Duration::from_secs(1));
    drop(auth_guard);

    let status = child.wait().with_context(|| "等待 codex 进程失败")?;

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

fn needs_quoting(name: &str) -> bool {
    name.contains(' ')
        || name.contains('\t')
        || name.contains('\n')
        || name.contains('"')
        || name.contains('\'')
        || name.contains('\\')
        || name.contains('*')
        || name.contains('?')
        || name.contains('[')
        || name.contains(']')
        || name.contains('(')
        || name.contains(')')
        || name.contains('{')
        || name.contains('}')
        || name.contains('$')
        || name.contains('&')
        || name.contains('|')
        || name.contains(';')
        || name.contains('<')
        || name.contains('>')
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn codex_dispatch_profile() {
        let mut cmd = Command::new("codex");
        cmd.env(AUTH_ENV_KEY, "test-key")
            .arg("-p")
            .arg("ccstart-abcd1234");
        cmd.arg("help me");

        let args: Vec<String> = cmd
            .get_args()
            .map(|a| a.to_string_lossy().into_owned())
            .collect();

        assert_eq!(args[0], "-p");
        assert_eq!(args[1], "ccstart-abcd1234");
        assert_eq!(args[2], "help me");
        assert!(!args.contains(&"-c".to_owned()));
    }

    #[test]
    fn codex_dispatch_env() {
        let mut cmd = Command::new("codex");
        cmd.env(AUTH_ENV_KEY, "secret-key")
            .arg("-p")
            .arg("ccstart-abcd1234");

        let env_val = cmd
            .get_envs()
            .find(|(k, _)| k.to_string_lossy() == AUTH_ENV_KEY)
            .unwrap()
            .1;

        assert_eq!(env_val, Some("secret-key".as_ref()));
    }
}
