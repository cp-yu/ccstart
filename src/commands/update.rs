use crate::config::manager::ConfigManager;
use crate::config::parser::parse_config_file;
use crate::error::AppResult;
use crate::utils::fs::get_config_paths;
use anyhow::Context;
use std::collections::HashSet;
use std::fs;

/// 更新配置：重新同步 config.json 的变更到 separated/ 目录
pub fn run() -> AppResult<()> {
    let paths = get_config_paths()?;

    eprintln!("[INFO] 正在读取配置文件: {}", paths.config_json.display());
    let collection = parse_config_file(&paths.config_json)
        .with_context(|| format!("读取配置文件失败: {}", paths.config_json.display()))?;

    eprintln!("[INFO] 正在同步配置...");

    // 提取新的配置列表
    let new_items = ConfigManager::extract_providers(&collection);
    let new_names: HashSet<String> = new_items.iter().map(|(_, uniq, _, _)| uniq.clone()).collect();

    // 读取现有配置列表
    let existing_configs = ConfigManager::list_configs()?;
    let existing_names: HashSet<String> = existing_configs.into_iter().collect();

    // 计算差异
    let added: Vec<_> = new_names.difference(&existing_names).collect();
    let removed: Vec<_> = existing_names.difference(&new_names).collect();
    let updated: Vec<_> = new_names.intersection(&existing_names).collect();

    let mut add_count = 0;
    let mut update_count = 0;
    let mut delete_count = 0;

    // 写入新增和更新的配置
    let written = ConfigManager::write_separated_configs(&new_items)?;
    for (path, (_, uniq, _, _)) in written.iter().zip(new_items.iter()) {
        if added.contains(&uniq) {
            eprintln!("✓ 新增: {} -> {}", uniq, path.display());
            add_count += 1;
        } else if updated.contains(&uniq) {
            eprintln!("✓ 更新: {} -> {}", uniq, path.display());
            update_count += 1;
        }
    }

    // 删除已移除的配置
    for name in removed {
        let file_path = ConfigManager::find_config_file(name);
        if let Ok(path) = file_path {
            fs::remove_file(&path)
                .with_context(|| format!("删除配置文件失败: {}", path.display()))?;
            eprintln!("✓ 删除: {} -> {}", name, path.display());
            delete_count += 1;
        }
    }

    eprintln!(
        "[INFO] 配置更新完成！新增 {} 个，更新 {} 个，删除 {} 个",
        add_count, update_count, delete_count
    );

    Ok(())
}
