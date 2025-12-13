//! JSON 差异比较模块：用于检测配置字段变更

use serde_json::Value;

/// 缓存操作结果
#[derive(Debug, Clone)]
pub enum CacheResult {
    /// 缓存未变化
    Unchanged(std::path::PathBuf),
    /// 新创建缓存
    Created(std::path::PathBuf),
    /// 缓存已更新，包含变更的字段列表
    Updated {
        path: std::path::PathBuf,
        changed_fields: Vec<String>,
    },
}

impl CacheResult {
    /// 获取缓存文件路径
    pub fn path(&self) -> &std::path::PathBuf {
        match self {
            CacheResult::Unchanged(p) | CacheResult::Created(p) | CacheResult::Updated { path: p, .. } => p,
        }
    }
}

/// 比较两个 JSON Value，返回变更的顶层字段列表
///
/// 只返回顶层字段名，嵌套变更归属于其顶层父字段
pub fn diff_top_level_fields(old: &Value, new: &Value) -> Vec<String> {
    let mut changed = Vec::new();

    match (old, new) {
        (Value::Object(old_map), Value::Object(new_map)) => {
            // 检查新增和修改的字段
            for (key, new_val) in new_map {
                match old_map.get(key) {
                    Some(old_val) if old_val != new_val => {
                        changed.push(key.clone());
                    }
                    None => {
                        changed.push(format!("{}(+)", key));
                    }
                    _ => {}
                }
            }
            // 检查删除的字段
            for key in old_map.keys() {
                if !new_map.contains_key(key) {
                    changed.push(format!("{}(-)", key));
                }
            }
        }
        _ => {
            // 非对象类型，整体变更
            if old != new {
                changed.push("(root)".to_string());
            }
        }
    }

    changed.sort();
    changed
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_diff_no_change() {
        let old = json!({"a": 1, "b": "hello"});
        let new = json!({"a": 1, "b": "hello"});
        assert!(diff_top_level_fields(&old, &new).is_empty());
    }

    #[test]
    fn test_diff_modified_field() {
        let old = json!({"a": 1, "b": "hello"});
        let new = json!({"a": 2, "b": "hello"});
        let result = diff_top_level_fields(&old, &new);
        assert_eq!(result, vec!["a"]);
    }

    #[test]
    fn test_diff_added_field() {
        let old = json!({"a": 1});
        let new = json!({"a": 1, "b": 2});
        let result = diff_top_level_fields(&old, &new);
        assert_eq!(result, vec!["b(+)"]);
    }

    #[test]
    fn test_diff_removed_field() {
        let old = json!({"a": 1, "b": 2});
        let new = json!({"a": 1});
        let result = diff_top_level_fields(&old, &new);
        assert_eq!(result, vec!["b(-)"]);
    }

    #[test]
    fn test_diff_nested_change() {
        let old = json!({"config": {"key": "old"}});
        let new = json!({"config": {"key": "new"}});
        let result = diff_top_level_fields(&old, &new);
        assert_eq!(result, vec!["config"]);
    }

    #[test]
    fn test_diff_multiple_changes() {
        let old = json!({"a": 1, "b": 2, "c": 3});
        let new = json!({"a": 10, "b": 2, "d": 4});
        let result = diff_top_level_fields(&old, &new);
        assert_eq!(result, vec!["a", "c(-)", "d(+)"]);
    }

    #[test]
    fn test_diff_non_object() {
        let old = json!([1, 2, 3]);
        let new = json!([1, 2, 4]);
        let result = diff_top_level_fields(&old, &new);
        assert_eq!(result, vec!["(root)"]);
    }
}
