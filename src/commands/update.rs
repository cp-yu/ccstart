use crate::config::cache::CacheManager;
use crate::db::Database;
use crate::error::AppResult;

/// 更新配置：强制刷新所有缓存文件
pub fn run() -> AppResult<()> {
    eprintln!("[INFO] 正在从数据库同步配置...");

    // 1. 打开数据库
    let db = Database::open()?;

    // 2. 获取所有 provider
    let providers = db.providers().list_all()?;
    let valid_names: Vec<String> = providers.iter().map(|p| p.name.clone()).collect();

    if providers.is_empty() {
        eprintln!("[WARN] 数据库中没有 Claude 配置");
        return Ok(());
    }

    // 3. 强制写入所有缓存
    let cache = CacheManager::new()?;
    let mut write_count = 0;

    for provider in &providers {
        let path = cache.force_write(provider)?;
        eprintln!("✓ 写入: {} -> {}", provider.name, path.display());
        write_count += 1;
    }

    // 4. 清理过期缓存
    let removed = cache.cleanup_stale(&valid_names)?;
    for name in &removed {
        eprintln!("✓ 删除: {}", name);
    }

    eprintln!(
        "[INFO] 配置更新完成！写入 {} 个，删除 {} 个",
        write_count,
        removed.len()
    );

    Ok(())
}
