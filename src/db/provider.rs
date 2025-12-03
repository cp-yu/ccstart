use crate::error::AppResult;
use anyhow::Context;
use rusqlite::params;
use serde_json::Value;

use super::Database;

/// Provider 数据模型
#[derive(Debug, Clone)]
pub struct Provider {
    #[allow(dead_code)]
    pub id: String,
    pub name: String,
    pub settings_config: Value,
}

/// Provider 数据访问对象
pub struct ProviderDao<'a> {
    db: &'a Database,
}

impl<'a> ProviderDao<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    /// 获取所有 Claude provider
    pub fn list_all(&self) -> AppResult<Vec<Provider>> {
        let conn = self.db.connect()?;
        let mut stmt = conn
            .prepare(
                "SELECT id, name, settings_config
                 FROM providers
                 WHERE app_type = 'claude'
                 ORDER BY sort_index, name",
            )
            .with_context(|| "准备查询语句失败")?;

        let iter = stmt
            .query_map([], |row| {
                let settings_str: String = row.get(2)?;
                Ok(Provider {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    settings_config: serde_json::from_str(&settings_str).unwrap_or(Value::Null),
                })
            })
            .with_context(|| "执行查询失败")?;

        iter.collect::<Result<Vec<_>, _>>()
            .with_context(|| "读取查询结果失败")
    }

    /// 根据名称获取 Provider
    pub fn get_by_name(&self, name: &str) -> AppResult<Option<Provider>> {
        let conn = self.db.connect()?;
        let mut stmt = conn
            .prepare(
                "SELECT id, name, settings_config
                 FROM providers
                 WHERE app_type = 'claude' AND name = ?1",
            )
            .with_context(|| "准备查询语句失败")?;

        let result = stmt.query_row(params![name], |row| {
            let settings_str: String = row.get(2)?;
            Ok(Provider {
                id: row.get(0)?,
                name: row.get(1)?,
                settings_config: serde_json::from_str(&settings_str).unwrap_or(Value::Null),
            })
        });

        match result {
            Ok(p) => Ok(Some(p)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e).with_context(|| format!("查询 provider '{}' 失败", name)),
        }
    }

    /// 获取所有 Claude provider 名称列表
    pub fn list_names(&self) -> AppResult<Vec<String>> {
        let conn = self.db.connect()?;
        let mut stmt = conn
            .prepare(
                "SELECT name FROM providers
                 WHERE app_type = 'claude'
                 ORDER BY sort_index, name",
            )
            .with_context(|| "准备查询语句失败")?;

        let names: Vec<String> = stmt
            .query_map([], |row| row.get(0))
            .with_context(|| "执行查询失败")?
            .collect::<Result<_, _>>()
            .with_context(|| "读取查询结果失败")?;

        Ok(names)
    }
}
