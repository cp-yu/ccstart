use crate::error::AppResult;
use anyhow::Context;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

/// Codex profile 缓存操作结果
#[derive(Debug, Clone, PartialEq)]
pub enum CodexCacheResult {
    Unchanged(PathBuf),
    Created(PathBuf),
    Updated(PathBuf),
}

impl CodexCacheResult {
    #[allow(dead_code)]
    pub fn path(&self) -> &PathBuf {
        match self {
            Self::Unchanged(p) | Self::Created(p) | Self::Updated(p) => p,
        }
    }
}

/// RAII guard：持有 auth.json 锁，drop 时还原原始内容
pub struct AuthGuard {
    auth_path: PathBuf,
    original: Option<Vec<u8>>,
    _lock: fs::File,
}

impl Drop for AuthGuard {
    fn drop(&mut self) {
        match &self.original {
            Some(orig) => { let _ = fs::write(&self.auth_path, orig); }
            None => { let _ = fs::remove_file(&self.auth_path); }
        }
    }
}

/// Codex profile 缓存管理器
pub struct CodexCacheManager {
    codex_dir: PathBuf,
}

impl CodexCacheManager {
    pub fn new() -> AppResult<Self> {
        let codex_dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("无法获取用户主目录"))?
            .join(".codex");
        Ok(Self { codex_dir })
    }

    /// 根据渠道名计算 profile 文件名
    pub fn profile_name(channel: &str) -> String {
        let hash = Self::name_hash(channel);
        format!("ccstart-{}", hash)
    }

    /// 获取 profile 文件路径
    pub fn profile_path(&self, channel: &str) -> PathBuf {
        let name = Self::profile_name(channel);
        self.codex_dir.join(format!("{}.config.toml", name))
    }

    /// 获取 auth.json 独占锁，写入 api_key，返回 RAII guard
    /// guard drop 时还原原始内容并释放锁；并发调用会阻塞等待
    pub fn lock_and_write_auth(&self, api_key: &str) -> AppResult<AuthGuard> {
        fs::create_dir_all(&self.codex_dir)
            .with_context(|| format!("创建 codex 目录失败: {}", self.codex_dir.display()))?;

        let lock_path = self.codex_dir.join(".ccstart-auth.lock");
        let lock_file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&lock_path)
            .with_context(|| "打开 auth 锁文件失败")?;
        lock_file.lock().with_context(|| "获取 auth 文件锁失败")?;

        let auth_path = self.codex_dir.join("auth.json");
        let original = if auth_path.exists() {
            Some(fs::read(&auth_path).with_context(|| "读取原始 auth.json 失败")?)
        } else {
            None
        };

        let content = format!("{{\"OPENAI_API_KEY\":\"{}\"}}\n", api_key);
        fs::write(&auth_path, content.as_bytes()).with_context(|| "写入 auth.json 失败")?;

        Ok(AuthGuard { auth_path, original, _lock: lock_file })
    }

    /// 确保 profile 文件存在且内容最新
    pub fn ensure_cached(&self, channel: &str, toml_content: &str) -> AppResult<CodexCacheResult> {
        let path = self.profile_path(channel);
        let new_hash = Self::content_hash(toml_content.as_bytes());

        if path.exists() {
            let existing = fs::read(&path).with_context(|| "读取 codex profile 文件失败")?;
            let existing_hash = Self::content_hash(&existing);

            if new_hash == existing_hash {
                return Ok(CodexCacheResult::Unchanged(path));
            }

            self.write_atomic(&path, toml_content.as_bytes())?;
            Ok(CodexCacheResult::Updated(path))
        } else {
            self.write_atomic(&path, toml_content.as_bytes())?;
            Ok(CodexCacheResult::Created(path))
        }
    }

    /// 清理不在有效集合中的 ccstart-* profile 文件
    pub fn cleanup_stale(&self, valid_channels: &[String]) -> AppResult<Vec<String>> {
        let valid_hashes: std::collections::HashSet<String> =
            valid_channels.iter().map(|c| Self::name_hash(c)).collect();

        let mut removed = Vec::new();
        if !self.codex_dir.exists() {
            return Ok(removed);
        }

        for entry in fs::read_dir(&self.codex_dir)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if let Some(fname) = path.file_name().and_then(|s| s.to_str())
                && let Some(rest) = fname.strip_prefix("ccstart-")
                && let Some(hash) = rest.strip_suffix(".config.toml")
                && !valid_hashes.contains(hash)
            {
                fs::remove_file(&path).with_context(|| {
                    format!("删除过期 codex profile 失败: {}", path.display())
                })?;
                removed.push(fname.to_string());
            }
        }

        removed.sort();
        Ok(removed)
    }

    /// SHA256(channel_name) 前 8 位 hex
    fn name_hash(channel: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(channel.as_bytes());
        let result = format!("{:x}", hasher.finalize());
        result[..8].to_string()
    }

    fn content_hash(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    fn write_atomic(&self, target: &PathBuf, data: &[u8]) -> AppResult<()> {
        fs::create_dir_all(&self.codex_dir)
            .with_context(|| format!("创建 codex 目录失败: {}", self.codex_dir.display()))?;

        let tmp = target.with_extension("toml.tmp");
        fs::write(&tmp, data).with_context(|| format!("写入临时文件失败: {}", tmp.display()))?;
        fs::rename(&tmp, target)
            .with_context(|| format!("重命名文件失败: {} -> {}", tmp.display(), target.display()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_manager(dir: &std::path::Path) -> CodexCacheManager {
        CodexCacheManager {
            codex_dir: dir.to_path_buf(),
        }
    }

    #[test]
    fn codex_cache_create() {
        let tmp = tempfile::tempdir().unwrap();
        let mgr = temp_manager(tmp.path());
        let toml = "model = \"gpt-4\"\n";

        let result = mgr.ensure_cached("packyapi", toml).unwrap();

        assert!(matches!(result, CodexCacheResult::Created(_)));
        let content = fs::read_to_string(result.path()).unwrap();
        assert_eq!(content, toml);
        assert!(result
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("ccstart-"));
    }

    #[test]
    fn codex_cache_unchanged() {
        let tmp = tempfile::tempdir().unwrap();
        let mgr = temp_manager(tmp.path());
        let toml = "model = \"gpt-4\"\n";

        mgr.ensure_cached("packyapi", toml).unwrap();
        let result = mgr.ensure_cached("packyapi", toml).unwrap();

        assert!(matches!(result, CodexCacheResult::Unchanged(_)));
    }

    #[test]
    fn codex_cache_updated() {
        let tmp = tempfile::tempdir().unwrap();
        let mgr = temp_manager(tmp.path());

        mgr.ensure_cached("packyapi", "model = \"gpt-4\"\n").unwrap();
        let result = mgr.ensure_cached("packyapi", "model = \"gpt-5\"\n").unwrap();

        assert!(matches!(result, CodexCacheResult::Updated(_)));
        let content = fs::read_to_string(result.path()).unwrap();
        assert_eq!(content, "model = \"gpt-5\"\n");
    }

    #[test]
    fn codex_cache_cleanup() {
        let tmp = tempfile::tempdir().unwrap();
        let mgr = temp_manager(tmp.path());

        mgr.ensure_cached("active", "x").unwrap();
        mgr.ensure_cached("stale", "y").unwrap();

        let removed = mgr.cleanup_stale(&["active".to_string()]).unwrap();

        assert_eq!(removed.len(), 1);
        assert!(removed[0].contains("ccstart-"));
        assert!(!mgr.profile_path("stale").exists());
        assert!(mgr.profile_path("active").exists());
    }

    #[test]
    fn profile_name_format() {
        let name = CodexCacheManager::profile_name("小丑");
        assert!(name.starts_with("ccstart-"));
        assert_eq!(name.len(), "ccstart-".len() + 8);
        assert!(name[8..].chars().all(|c| c.is_ascii_hexdigit()));
    }
}
