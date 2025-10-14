use crate::error::AppResult;
use dirs::home_dir;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ConfigPaths {
    pub base_dir: PathBuf,
    pub config_json: PathBuf,
    pub separated_dir: PathBuf,
}

/// 将以 `~` 开头的路径展开为用户主目录
pub fn expand_tilde(path: &str) -> AppResult<PathBuf> {
    if let Some(rest) = path.strip_prefix("~/") {
        let home = home_dir().ok_or_else(|| anyhow::anyhow!("无法获取用户主目录"))?;
        Ok(home.join(rest))
    } else if path == "~" {
        let home = home_dir().ok_or_else(|| anyhow::anyhow!("无法获取用户主目录"))?;
        Ok(home)
    } else {
        Ok(PathBuf::from(path))
    }
}

/// 确保目录存在（不存在则创建）
pub fn ensure_dir_exists(dir: &Path) -> AppResult<()> {
    if !dir.exists() {
        fs::create_dir_all(dir)?;
    }
    Ok(())
}

/// 返回 ccstart 使用的标准路径
pub fn get_config_paths() -> AppResult<ConfigPaths> {
    let base_dir = expand_tilde("~/.cc-switch")?;
    let config_json = base_dir.join("config.json");
    let separated_dir = base_dir.join("separated");
    Ok(ConfigPaths {
        base_dir,
        config_json,
        separated_dir,
    })
}

