mod provider;

pub use provider::{Provider, ProviderDao};

use crate::error::AppResult;
use anyhow::Context;
use rusqlite::{Connection, OpenFlags};
use std::path::PathBuf;

/// 数据库访问结构
pub struct Database {
    path: PathBuf,
}

impl Database {
    /// 打开数据库（只读模式）
    pub fn open() -> AppResult<Self> {
        let path = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("无法获取用户主目录"))?
            .join(".cc-switch/cc-switch.db");

        if !path.exists() {
            anyhow::bail!(
                "数据库不存在: {}\n提示: 请先运行 cc-switch 创建数据库",
                path.display()
            );
        }

        Ok(Self { path })
    }

    /// 创建只读连接
    pub fn connect(&self) -> AppResult<Connection> {
        Connection::open_with_flags(&self.path, OpenFlags::SQLITE_OPEN_READ_ONLY)
            .with_context(|| format!("无法打开数据库: {}", self.path.display()))
    }

    /// 获取 Provider DAO
    pub fn providers(&self) -> ProviderDao<'_> {
        ProviderDao::new(self)
    }
}
