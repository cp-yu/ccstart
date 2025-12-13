use crate::config::cache::CacheManager;
use crate::config::diff::CacheResult;
use crate::db::Database;
use crate::error::AppResult;

/// 更新配置：同步所有缓存文件并显示变更详情
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

    // 3. 同步缓存并显示变更详情
    let cache = CacheManager::new()?;
    let mut created_count = 0;
    let mut updated_count = 0;
    let mut unchanged_count = 0;

    for provider in &providers {
        let result = cache.ensure_cached(provider)?;
        match result {
            CacheResult::Created(path) => {
                eprintln!("✓ 新增: {} -> {}", provider.name, path.display());
                created_count += 1;
            }
            CacheResult::Updated { path, changed_fields } => {
                let fields = changed_fields.join(", ");
                eprintln!("✓ 更新: {} ({}) -> {}", provider.name, fields, path.display());
                updated_count += 1;
            }
            CacheResult::Unchanged(_) => {
                unchanged_count += 1;
            }
        }
    }

    // 4. 清理过期缓存
    let removed = cache.cleanup_stale(&valid_names)?;
    for name in &removed {
        eprintln!("✓ 删除: {}", name);
    }

    eprintln!(
        "[INFO] 配置同步完成！新增 {}，更新 {}，未变 {}，删除 {}",
        created_count,
        updated_count,
        unchanged_count,
        removed.len()
    );

    Ok(())
}
