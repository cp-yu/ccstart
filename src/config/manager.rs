use crate::error::AppResult;
use crate::utils::{
    encoding::{decode_config_name, encode_config_name},
    fs::{ensure_dir_exists, get_config_paths},
};
use crate::config::parser::{ConfigCollection, Provider};
use anyhow::Context;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub struct ConfigManager;

impl ConfigManager {
    /// 处理重复名称：对于重复的名称添加数字后缀（-2, -3, ...）
    pub fn handle_duplicate_names(names: &[String]) -> Vec<String> {
        let mut counts: HashMap<String, usize> = HashMap::new();
        let mut result = Vec::with_capacity(names.len());
        for name in names {
            let count = counts.entry(name.clone()).or_insert(0);
            *count += 1;
            if *count == 1 {
                result.push(name.clone());
            } else {
                result.push(format!("{}-{}", name, *count));
            }
        }
        result
    }

    /// 从 ConfigCollection 中提取 provider 设置，并返回 (原始名称, 唯一名称, 编码后文件名, JSON 值)
    pub fn extract_providers(collection: &ConfigCollection) -> Vec<(String, String, String, Value)> {
        // 使用 BTreeMap 确保稳定顺序
        let mut items: Vec<(String, Provider)> = collection
            .claude
            .providers.values().map(|v| (Self::display_name(v), v.clone()))
            .collect();
        // 稳定排序以保证确定性
        items.sort_by(|a, b| a.0.cmp(&b.0));

        let base_names: Vec<String> = items.iter().map(|(n, _)| n.clone()).collect();
        let unique = Self::handle_duplicate_names(&base_names);

        items
            .into_iter()
            .zip(unique)
            .map(|((orig_name, prov), uniq)| {
                let encoded = encode_config_name(&uniq);
                (orig_name, uniq, encoded, prov.settings_config)
            })
            .collect()
    }

    fn display_name(provider: &Provider) -> String {
        if !provider.name.trim().is_empty() {
            provider.name.trim().to_string()
        } else if !provider.id.trim().is_empty() {
            provider.id.trim().to_string()
        } else {
            "default".to_string()
        }
    }

    /// 将分离后的配置写入 `~/.cc-switch/separated/config-<encoded>.json`
    pub fn write_separated_configs(items: &[(String, String, String, Value)]) -> AppResult<Vec<PathBuf>> {
        let paths = get_config_paths()?;
        ensure_dir_exists(&paths.separated_dir)?;

        let mut written = Vec::with_capacity(items.len());
        for (_orig, _uniq, encoded, json) in items {
            let file_name = format!("config-{}.json", encoded);
            let target = paths.separated_dir.join(file_name);

            // 原子写：写入临时文件再重命名
            let tmp_target = target.with_extension("json.tmp");
            let data = serde_json::to_vec_pretty(json)?;
            {
                let mut file = fs::File::create(&tmp_target)
                    .with_context(|| format!("创建临时文件失败: {}", tmp_target.display()))?;
                file.write_all(&data)
                    .with_context(|| format!("写入临时文件失败: {}", tmp_target.display()))?;
                file.sync_all().ok();
            }
            fs::rename(&tmp_target, &target)
                .with_context(|| format!("重命名文件失败: {} -> {}", tmp_target.display(), target.display()))?;

            written.push(target);
        }
        Ok(written)
    }

    /// 查找指定名称的配置文件路径（编码后）
    pub fn find_config_file(name: &str) -> AppResult<PathBuf> {
        let paths = get_config_paths()?;
        let encoded = crate::utils::encoding::encode_config_name(name);
        let file = paths.separated_dir.join(format!("config-{}.json", encoded));
        if file.exists() {
            Ok(file)
        } else {
            anyhow::bail!("配置 '{}' 不存在: {}", name, file.display());
        }
    }

    /// 列出 separated/ 目录下所有可用的配置名称（解码后）
    pub fn list_configs() -> AppResult<Vec<String>> {
        let paths = get_config_paths()?;
        let mut results = Vec::new();
        if !paths.separated_dir.exists() {
            return Ok(results);
        }
        for entry in std::fs::read_dir(&paths.separated_dir)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if let Some(fname) = path.file_name().and_then(|s| s.to_str())
                && let Some(rest) = fname.strip_prefix("config-")
                    && let Some(encoded) = rest.strip_suffix(".json")
                        && let Ok(decoded) = decode_config_name(encoded) {
                            results.push(decoded);
                        }
        }
        results.sort();
        Ok(results)
    }
}
