use crate::config::manager::ConfigManager;
use crate::config::parser::parse_config_file;
use crate::error::AppResult;
use crate::utils::fs::{get_config_paths, ensure_dir_exists};
use anyhow::Context;
use std::io::{self, Write};

pub fn run(force: bool) -> AppResult<()> {
    let paths = get_config_paths()?;
    ensure_dir_exists(&paths.base_dir)?;

    // 检查 separated 目录是否存在
    if paths.separated_dir.exists() && !force {
        eprintln!(
            "[WARN] 检测到已存在的分离目录: {}",
            paths.separated_dir.display()
        );
        eprint!("是否继续并覆盖已存在的文件？(y/N): ");
        io::stderr().flush().ok();
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();
        if !(input == "y" || input == "yes") {
            eprintln!("[INFO] 已取消初始化。");
            return Ok(());
        }
    }

    eprintln!("[INFO] 读取配置: {}", paths.config_json.display());
    let collection = parse_config_file(&paths.config_json)
        .with_context(|| format!("无法解析配置文件: {}", paths.config_json.display()))?;

    eprintln!("[INFO] 提取 provider 设置...");
    let items = ConfigManager::extract_providers(&collection);
    eprintln!("[INFO] 需写入 {} 个配置", items.len());

    let written = ConfigManager::write_separated_configs(&items)?;
    for p in written {
        eprintln!("[INFO] 写入: {}", p.display());
    }

    eprintln!("[INFO] 初始化完成。");
    Ok(())
}

