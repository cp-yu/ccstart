use crate::db::Provider;
use crate::error::AppResult;
use crate::utils::encoding::encode_config_name;
use anyhow::Context;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

/// 缓存管理器：处理配置文件的懒加载和哈希比较
pub struct CacheManager {
    cache_dir: PathBuf,
}

impl CacheManager {
    pub fn new() -> AppResult<Self> {
        let cache_dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("无法获取用户主目录"))?
            .join(".cc-switch/separated");

        Ok(Self { cache_dir })
    }

    /// 获取缓存文件路径
    pub fn get_cache_path(&self, name: &str) -> PathBuf {
        let encoded = encode_config_name(name);
        self.cache_dir.join(format!("config-{}.json", encoded))
    }

    /// 确保缓存文件存在且是最新的，返回文件路径
    /// 使用内容哈希比较，只在内容变化时才重新写入
    pub fn ensure_cached(&self, provider: &Provider) -> AppResult<PathBuf> {
        let path = self.get_cache_path(&provider.name);
        let content = serde_json::to_vec_pretty(&provider.settings_config)
            .with_context(|| "序列化配置失败")?;
        let new_hash = Self::hash_content(&content);

        let should_write = if path.exists() {
            let existing = fs::read(&path).with_context(|| "读取缓存文件失败")?;
            let existing_hash = Self::hash_content(&existing);
            new_hash != existing_hash
        } else {
            true
        };

        if should_write {
            self.write_atomic(&path, &content)?;
        }

        Ok(path)
    }

    /// 强制写入缓存文件（用于 update 命令）
    pub fn force_write(&self, provider: &Provider) -> AppResult<PathBuf> {
        let path = self.get_cache_path(&provider.name);
        let content = serde_json::to_vec_pretty(&provider.settings_config)
            .with_context(|| "序列化配置失败")?;
        self.write_atomic(&path, &content)?;
        Ok(path)
    }

    /// 删除指定名称的缓存文件
    pub fn remove_cache(&self, name: &str) -> AppResult<()> {
        let path = self.get_cache_path(name);
        if path.exists() {
            fs::remove_file(&path).with_context(|| format!("删除缓存文件失败: {}", path.display()))?;
        }
        Ok(())
    }

    /// 列出所有缓存的配置名称
    pub fn list_cached_names(&self) -> AppResult<Vec<String>> {
        let mut results = Vec::new();
        if !self.cache_dir.exists() {
            return Ok(results);
        }

        for entry in fs::read_dir(&self.cache_dir)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if let Some(fname) = path.file_name().and_then(|s| s.to_str())
                && let Some(rest) = fname.strip_prefix("config-")
                && let Some(encoded) = rest.strip_suffix(".json")
                && let Ok(decoded) = crate::utils::encoding::decode_config_name(encoded)
            {
                results.push(decoded);
            }
        }
        results.sort();
        Ok(results)
    }

    /// 清理不在数据库中的缓存文件
    pub fn cleanup_stale(&self, valid_names: &[String]) -> AppResult<Vec<String>> {
        let cached = self.list_cached_names()?;
        let valid_set: std::collections::HashSet<_> = valid_names.iter().collect();
        let mut removed = Vec::new();

        for name in cached {
            if !valid_set.contains(&name) {
                self.remove_cache(&name)?;
                removed.push(name);
            }
        }

        Ok(removed)
    }

    /// 计算内容哈希
    fn hash_content(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /// 原子写入：先写临时文件再重命名
    fn write_atomic(&self, target: &PathBuf, data: &[u8]) -> AppResult<()> {
        fs::create_dir_all(&self.cache_dir)
            .with_context(|| format!("创建缓存目录失败: {}", self.cache_dir.display()))?;

        let tmp = target.with_extension("json.tmp");
        fs::write(&tmp, data).with_context(|| format!("写入临时文件失败: {}", tmp.display()))?;
        fs::rename(&tmp, target)
            .with_context(|| format!("重命名文件失败: {} -> {}", tmp.display(), target.display()))?;

        Ok(())
    }
}
