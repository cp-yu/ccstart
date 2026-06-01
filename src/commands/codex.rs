use crate::db::Database;
use crate::error::AppResult;
use anyhow::Context;
#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
use std::process::Command;
use toml::Value;

#[derive(Debug)]
struct CodexConfig {
    api_key: String,
    model_provider: String,
    model: String,
    base_url: String,
}

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
    let provider = match db.providers().get_by_name("codex", channel)? {
        Some(p) => p,
        None => {
            eprintln!("错误: 未找到 Codex 渠道 '{}'", channel);
            if let Ok(names) = db.providers().list_names("codex") {
                if names.is_empty() {
                    eprintln!("提示: 数据库中没有 Codex 渠道，请先在 cc-switch 中添加。");
                } else {
                    eprintln!("提示: 可用 Codex 渠道如下：");
                    for n in names {
                        eprintln!("  - {}", n);
                    }
                }
            }
            return Ok(1);
        }
    };

    let config = parse_settings_config(&provider.settings_config)?;

    let mut cmd = Command::new("codex");
    cmd.env("OPENAI_API_KEY", config.api_key)
        .arg("-c")
        .arg(format!(
            "model_providers.{}.base_url={}",
            config.model_provider,
            toml_string(&config.base_url)
        ))
        .arg("-c")
        .arg(format!("model={}", toml_string(&config.model)));

    for a in args {
        cmd.arg(a);
    }

    let status = cmd
        .status()
        .with_context(|| "执行 'codex' 命令失败，请确认已安装并在 PATH 中")?;

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

fn parse_settings_config(settings: &serde_json::Value) -> AppResult<CodexConfig> {
    let api_key = settings
        .pointer("/auth/OPENAI_API_KEY")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("settings_config 缺少 auth.OPENAI_API_KEY"))?
        .to_owned();

    let config_str = settings
        .get("config")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("settings_config 缺少 config"))?;

    let config: Value =
        toml::from_str(config_str).with_context(|| "解析 settings_config.config TOML 失败")?;
    let model_provider = get_str(&config, &["model_provider"])?.to_owned();
    let model = get_str(&config, &["model"])?.to_owned();
    let base_url = get_str(&config, &["model_providers", &model_provider, "base_url"])?.to_owned();

    Ok(CodexConfig {
        api_key,
        model_provider,
        model,
        base_url,
    })
}

fn get_str<'a>(value: &'a Value, path: &[&str]) -> AppResult<&'a str> {
    let mut current = value;
    for key in path {
        current = current
            .get(*key)
            .ok_or_else(|| anyhow::anyhow!("settings_config.config 缺少 {}", path.join(".")))?;
    }
    current
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("settings_config.config 字段 {} 不是字符串", path.join(".")))
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

fn toml_string(value: &str) -> String {
    Value::String(value.to_owned()).to_string()
}

#[cfg(test)]
mod tests {
    use super::parse_settings_config;
    use serde_json::json;

    #[test]
    fn parses_standard_config() {
        let settings = json!({
            "auth": {
                "OPENAI_API_KEY": "key"
            },
            "config": r#"
model_provider = "custom"
model = "gpt-5.5"

[model_providers.custom]
base_url = "https://example.com/v1"
"#
        });

        let config = parse_settings_config(&settings).unwrap();

        assert_eq!(config.api_key, "key");
        assert_eq!(config.model_provider, "custom");
        assert_eq!(config.model, "gpt-5.5");
        assert_eq!(config.base_url, "https://example.com/v1");
    }

    #[test]
    fn reports_missing_base_url() {
        let settings = json!({
            "auth": {
                "OPENAI_API_KEY": "key"
            },
            "config": r#"
model_provider = "custom"
model = "gpt-5.5"

[model_providers.custom]
"#
        });

        let err = parse_settings_config(&settings).unwrap_err().to_string();

        assert!(err.contains("model_providers.custom.base_url"));
    }
}
