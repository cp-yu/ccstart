# cc-switch SQLite 数据库使用详解

本文档详细描述项目中 SQLite 的使用方式，可作为其他项目的参考模板。

## 1. 依赖配置

```toml
# Cargo.toml
[dependencies]
rusqlite = { version = "0.31", features = ["bundled", "backup"] }
```

- `bundled`: 内置 SQLite，无需系统安装
- `backup`: 支持 Backup API（用于快照/导入导出）

## 2. 数据库初始化

```rust
use rusqlite::Connection;
use std::sync::Mutex;

pub struct Database {
    conn: Mutex<Connection>,  // Mutex 包装实现线程安全
}

impl Database {
    /// 文件数据库（生产环境）
    pub fn init() -> Result<Self, AppError> {
        let db_path = get_app_config_dir().join("cc-switch.db");

        // 确保目录存在
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(&db_path)?;
        conn.execute("PRAGMA foreign_keys = ON;", [])?;  // 启用外键

        let db = Self { conn: Mutex::new(conn) };
        db.create_tables()?;
        db.apply_schema_migrations()?;
        Ok(db)
    }

    /// 内存数据库（测试用）
    pub fn memory() -> Result<Self, AppError> {
        let conn = Connection::open_in_memory()?;
        conn.execute("PRAGMA foreign_keys = ON;", [])?;
        let db = Self { conn: Mutex::new(conn) };
        db.create_tables()?;
        Ok(db)
    }
}
```

## 3. 线程安全访问宏

```rust
/// 安全获取 Mutex 锁
macro_rules! lock_conn {
    ($mutex:expr) => {
        $mutex
            .lock()
            .map_err(|e| AppError::Database(format!("Mutex lock failed: {}", e)))?
    };
}

// 使用示例
pub fn query_count(&self) -> Result<i64, AppError> {
    let conn = lock_conn!(self.conn);
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM table", [], |row| row.get(0))?;
    Ok(count)
}
```

## 4. Schema 版本管理

```rust
pub(crate) const SCHEMA_VERSION: i32 = 1;

impl Database {
    fn get_user_version(conn: &Connection) -> Result<i32, AppError> {
        conn.query_row("PRAGMA user_version;", [], |row| row.get(0))
    }

    fn set_user_version(conn: &Connection, version: i32) -> Result<(), AppError> {
        conn.execute(&format!("PRAGMA user_version = {version};"), [])
    }

    /// 迁移入口
    fn apply_schema_migrations(&self) -> Result<(), AppError> {
        let conn = lock_conn!(self.conn);
        conn.execute("SAVEPOINT schema_migration;", [])?;

        let mut version = Self::get_user_version(&conn)?;

        // 版本过新则拒绝
        if version > SCHEMA_VERSION {
            conn.execute("ROLLBACK TO schema_migration;", []).ok();
            return Err(AppError::Database("数据库版本过新".into()));
        }

        // 逐版本迁移
        while version < SCHEMA_VERSION {
            match version {
                0 => {
                    Self::migrate_v0_to_v1(&conn)?;
                    Self::set_user_version(&conn, 1)?;
                }
                _ => return Err(AppError::Database("未知版本".into())),
            }
            version = Self::get_user_version(&conn)?;
        }

        conn.execute("RELEASE schema_migration;", [])?;
        Ok(())
    }
}
```

## 5. 安全添加列（迁移辅助）

```rust
fn add_column_if_missing(
    conn: &Connection,
    table: &str,
    column: &str,
    definition: &str,
) -> Result<bool, AppError> {
    if Self::has_column(conn, table, column)? {
        return Ok(false);
    }

    let sql = format!("ALTER TABLE \"{table}\" ADD COLUMN \"{column}\" {definition};");
    conn.execute(&sql, [])?;
    Ok(true)
}

fn has_column(conn: &Connection, table: &str, column: &str) -> Result<bool, AppError> {
    let sql = format!("PRAGMA table_info(\"{table}\");");
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let name: String = row.get(1)?;
        if name.eq_ignore_ascii_case(column) {
            return Ok(true);
        }
    }
    Ok(false)
}
```

## 6. CRUD 操作模式

```rust
use rusqlite::params;

impl Database {
    /// 查询列表
    pub fn get_all(&self, app_type: &str) -> Result<Vec<Item>, AppError> {
        let conn = lock_conn!(self.conn);
        let mut stmt = conn.prepare(
            "SELECT id, name, config FROM items WHERE app_type = ?1 ORDER BY id"
        )?;

        let iter = stmt.query_map(params![app_type], |row| {
            Ok(Item {
                id: row.get(0)?,
                name: row.get(1)?,
                config: serde_json::from_str(&row.get::<_, String>(2)?).unwrap_or_default(),
            })
        })?;

        iter.collect::<Result<Vec<_>, _>>()
            .map_err(|e| AppError::Database(e.to_string()))
    }

    /// 事务写入
    pub fn save(&self, item: &Item) -> Result<(), AppError> {
        let mut conn = lock_conn!(self.conn);
        let tx = conn.transaction()?;

        tx.execute(
            "INSERT OR REPLACE INTO items (id, name, config) VALUES (?1, ?2, ?3)",
            params![item.id, item.name, serde_json::to_string(&item.config)?],
        )?;

        tx.commit()?;
        Ok(())
    }

    /// 删除
    pub fn delete(&self, id: &str) -> Result<(), AppError> {
        let conn = lock_conn!(self.conn);
        conn.execute("DELETE FROM items WHERE id = ?1", params![id])?;
        Ok(())
    }
}
```

## 7. 备份与恢复

```rust
use rusqlite::backup::Backup;

impl Database {
    /// 创建内存快照
    fn snapshot_to_memory(&self) -> Result<Connection, AppError> {
        let conn = lock_conn!(self.conn);
        let mut snapshot = Connection::open_in_memory()?;

        let backup = Backup::new(&conn, &mut snapshot)?;
        backup.step(-1)?;  // -1 表示一次性复制全部

        Ok(snapshot)
    }

    /// 从快照恢复
    fn restore_from(&self, source: &Connection) -> Result<(), AppError> {
        let mut main_conn = lock_conn!(self.conn);
        let backup = Backup::new(source, &mut main_conn)?;
        backup.step(-1)?;
        Ok(())
    }
}
```

## 8. 完整表结构

```sql
-- 1. providers (AI 提供商)
CREATE TABLE providers (
    id TEXT NOT NULL,
    app_type TEXT NOT NULL,           -- 'claude' | 'codex' | 'gemini'
    name TEXT NOT NULL,
    settings_config TEXT NOT NULL,    -- JSON: API Key 等配置
    website_url TEXT,
    category TEXT,
    created_at INTEGER,               -- Unix timestamp (ms)
    sort_index INTEGER,
    notes TEXT,
    icon TEXT,
    icon_color TEXT,
    meta TEXT NOT NULL DEFAULT '{}',  -- JSON: 扩展元数据
    is_current BOOLEAN NOT NULL DEFAULT 0,
    PRIMARY KEY (id, app_type)
);

-- 2. provider_endpoints (提供商端点)
CREATE TABLE provider_endpoints (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    provider_id TEXT NOT NULL,
    app_type TEXT NOT NULL,
    url TEXT NOT NULL,
    added_at INTEGER,
    FOREIGN KEY (provider_id, app_type)
        REFERENCES providers(id, app_type) ON DELETE CASCADE
);

-- 3. mcp_servers (MCP 服务器)
CREATE TABLE mcp_servers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    server_config TEXT NOT NULL,      -- JSON: 服务器配置
    description TEXT,
    homepage TEXT,
    docs TEXT,
    tags TEXT NOT NULL DEFAULT '[]',  -- JSON Array
    enabled_claude BOOLEAN NOT NULL DEFAULT 0,
    enabled_codex BOOLEAN NOT NULL DEFAULT 0,
    enabled_gemini BOOLEAN NOT NULL DEFAULT 0
);

-- 4. prompts (提示词)
CREATE TABLE prompts (
    id TEXT NOT NULL,
    app_type TEXT NOT NULL,
    name TEXT NOT NULL,
    content TEXT NOT NULL,
    description TEXT,
    enabled BOOLEAN NOT NULL DEFAULT 1,
    created_at INTEGER,
    updated_at INTEGER,
    PRIMARY KEY (id, app_type)
);

-- 5. skills (技能)
CREATE TABLE skills (
    key TEXT PRIMARY KEY,
    installed BOOLEAN NOT NULL DEFAULT 0,
    installed_at INTEGER NOT NULL DEFAULT 0
);

-- 6. skill_repos (技能仓库)
CREATE TABLE skill_repos (
    owner TEXT NOT NULL,
    name TEXT NOT NULL,
    branch TEXT NOT NULL DEFAULT 'main',
    enabled BOOLEAN NOT NULL DEFAULT 1,
    PRIMARY KEY (owner, name)
);

-- 7. settings (通用设置)
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT                        -- JSON 或纯文本
);
```

## 9. 外部访问

数据库位置: `~/.cc-switch/cc-switch.db`

```bash
# 命令行访问
sqlite3 ~/.cc-switch/cc-switch.db

# 常用命令
.tables                              # 列出所有表
.schema providers                    # 查看表结构
.mode column                         # 列模式显示
.headers on                          # 显示列头

# 查询示例
SELECT id, name FROM providers WHERE app_type = 'claude';
SELECT * FROM mcp_servers WHERE enabled_claude = 1;
SELECT key, value FROM settings;

# 导出
.output backup.sql
.dump
.output stdout
```

## 10. 关键设计要点

| 要点 | 说明 |
|------|------|
| 线程安全 | `Mutex<Connection>` 包装，配合 `lock_conn!` 宏 |
| 外键约束 | `PRAGMA foreign_keys = ON` 启用级联删除 |
| 版本管理 | `PRAGMA user_version` 追踪 Schema 版本 |
| 迁移策略 | SAVEPOINT 事务 + 逐版本迁移 + 回滚保护 |
| JSON 存储 | 复杂配置用 `TEXT` 存 JSON，读取时 `serde_json::from_str` |
| 复合主键 | `(id, app_type)` 支持多应用类型隔离 |
| 备份机制 | `rusqlite::backup::Backup` API 实现原子快照 |

## 11. 源码位置

```
src-tauri/src/database/
├── mod.rs        - Database 结构体 + 初始化
├── schema.rs     - 表结构定义 + Schema 迁移
├── backup.rs     - SQL 导入导出 + 快照备份
├── migration.rs  - JSON → SQLite 数据迁移
└── dao/          - 数据访问对象
    ├── providers.rs
    ├── mcp.rs
    ├── prompts.rs
    ├── skills.rs
    └── settings.rs
```

---

## 12. ccstart 集成方案

ccstart 作为 cc-switch 的 CLI 客户端，采用 **只读访问 + Read-Through Cache** 模式集成 SQLite。

### 12.1 架构设计

```
                    ┌──────────────────┐
                    │    SQLite DB     │
                    │  (cc-switch.db)  │
                    └────────┬─────────┘
                             │
                    [只读查询 providers]
                             │
                             v
                    ┌──────────────────┐
                    │   内容哈希比较    │
                    │  (SHA256)        │
                    └────────┬─────────┘
                             │
              ┌──────────────┴──────────────┐
              │                             │
        [哈希相同]                    [哈希不同]
              │                             │
              v                             v
    ┌──────────────────┐         ┌──────────────────┐
    │  使用缓存文件     │         │ 重新生成缓存      │
    └──────────────────┘         └──────────────────┘
              │                             │
              └──────────────┬──────────────┘
                             │
                             v
                    ┌──────────────────┐
                    │ claude --settings│
                    │     <path>       │
                    └──────────────────┘
```

### 12.2 核心模块

```
src/
├── db/
│   ├── mod.rs        - Database 只读连接
│   └── provider.rs   - Provider DAO (查询)
├── config/
│   └── cache.rs      - 缓存管理 (哈希比较 + 原子写入)
└── commands/
    ├── run.rs        - Read-Through Cache 执行
    ├── list.rs       - 直接查询 SQLite
    └── update.rs     - 强制刷新所有缓存
```

### 12.3 只读访问模式

```rust
use rusqlite::{Connection, OpenFlags};

impl Database {
    pub fn connect(&self) -> AppResult<Connection> {
        // 只读模式打开，不会修改上游数据库
        Connection::open_with_flags(&self.path, OpenFlags::SQLITE_OPEN_READ_ONLY)
    }
}
```

### 12.4 缓存策略

ccstart 使用 SHA256 哈希比较决定是否重新生成缓存：

```rust
pub fn ensure_cached(&self, provider: &Provider) -> AppResult<PathBuf> {
    let path = self.get_cache_path(&provider.name);
    let content = serde_json::to_vec_pretty(&provider.settings_config)?;
    let new_hash = sha256(&content);

    let should_write = if path.exists() {
        let existing = fs::read(&path)?;
        sha256(&existing) != new_hash
    } else {
        true
    };

    if should_write {
        self.write_atomic(&path, &content)?;
    }

    Ok(path)
}
```

### 12.5 命令对照

| 命令 | 数据源 | 说明 |
|------|--------|------|
| `ccstart list` | SQLite | 直接查询 `providers` 表 |
| `ccstart <name>` | SQLite + 缓存 | 查询 → 哈希比较 → 按需写缓存 → 执行 |
| `ccstart update` | SQLite | 强制刷新所有缓存文件 |
| `ccstart completions` | SQLite | 动态查询补全候选 |

### 12.6 关键设计决策

| 决策 | 选择 | 原因 |
|------|------|------|
| 访问模式 | 只读 | 不修改上游数据，安全 |
| 缓存策略 | 内容哈希 | 最小化 I/O，只在变化时写入 |
| 缓存路径 | `~/.cc-switch/separated/` | 与旧版兼容 |
| `init` 命令 | 已移除 | SQLite 模式无需初始化 |
| `update` 命令 | 保留 | 用于强制刷新/调试 |
