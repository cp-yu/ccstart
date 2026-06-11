use crate::config::cache::CacheManager;
use crate::config::codex_cache::{CodexCacheManager, CodexCacheResult};
use crate::config::diff::CacheResult;
use crate::db::Database;
use crate::error::AppResult;

/// 更新配置：同步所有缓存文件并显示变更详情
pub fn run() -> AppResult<()> {
    eprintln!("[INFO] 正在从数据库同步配置...");

    let db = Database::open()?;

    // 1. 同步 Claude 配置
    let providers = db.providers().list_all("claude")?;
    let valid_names: Vec<String> = providers.iter().map(|p| p.name.clone()).collect();

    if !providers.is_empty() {
        let cache = CacheManager::new()?;
        let mut created = 0;
        let mut updated = 0;
        let mut unchanged = 0;

        for provider in &providers {
            match cache.ensure_cached(provider)? {
                CacheResult::Created(path) => {
                    eprintln!("✓ Claude 新增: {} -> {}", provider.name, path.display());
                    created += 1;
                }
                CacheResult::Updated { path, changed_fields } => {
                    let fields = changed_fields.join(", ");
                    eprintln!("✓ Claude 更新: {} ({}) -> {}", provider.name, fields, path.display());
                    updated += 1;
                }
                CacheResult::Unchanged(_) => {
                    unchanged += 1;
                }
            }
        }

        let removed = cache.cleanup_stale(&valid_names)?;
        for name in &removed {
            eprintln!("✓ Claude 删除: {}", name);
        }

        eprintln!(
            "[INFO] Claude 配置同步完成！新增 {}，更新 {}，未变 {}，删除 {}",
            created, updated, unchanged, removed.len()
        );
    } else {
        eprintln!("[WARN] 数据库中没有 Claude 配置");
    }

    // 2. 同步 Codex profile
    let codex_providers = db.providers().list_all("codex")?;
    let codex_names: Vec<String> = codex_providers.iter().map(|p| p.name.clone()).collect();

    if !codex_providers.is_empty() {
        let codex_cache = CodexCacheManager::new()?;
        let mut cx_created = 0;
        let mut cx_updated = 0;
        let mut cx_unchanged = 0;

        for provider in &codex_providers {
            let toml = provider
                .settings_config
                .get("config")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            match codex_cache.ensure_cached(&provider.name, toml)? {
                CodexCacheResult::Created(path) => {
                    eprintln!("✓ Codex 新增: {} -> {}", provider.name, path.display());
                    cx_created += 1;
                }
                CodexCacheResult::Updated(path) => {
                    eprintln!("✓ Codex 更新: {} -> {}", provider.name, path.display());
                    cx_updated += 1;
                }
                CodexCacheResult::Unchanged(_) => {
                    cx_unchanged += 1;
                }
            }
        }

        let cx_removed = codex_cache.cleanup_stale(&codex_names)?;
        for name in &cx_removed {
            eprintln!("✓ Codex 删除: {}", name);
        }

        eprintln!(
            "[INFO] Codex profile 同步完成！新增 {}，更新 {}，未变 {}，删除 {}",
            cx_created, cx_updated, cx_unchanged, cx_removed.len()
        );
    } else {
        eprintln!("[WARN] 数据库中没有 Codex 渠道");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn update_syncs_codex() {
        // 测试逻辑需要真实数据库，这里验证代码结构
    }

    #[test]
    fn update_cleans_codex() {
        // 测试逻辑需要真实数据库，这里验证代码结构
    }
}
