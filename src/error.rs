use std::fmt;

/// Domain-specific configuration errors
#[allow(dead_code)]
#[derive(Debug)]
pub enum ConfigError {
    InvalidStructure(String),
    MissingField(String),
    DuplicateName(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::InvalidStructure(s) => write!(f, "无效的配置结构: {}", s),
            ConfigError::MissingField(s) => write!(f, "缺少必要字段: {}", s),
            ConfigError::DuplicateName(s) => write!(f, "重复的配置名称: {}", s),
        }
    }
}

/// I/O 错误占位符（如需分类使用）
#[allow(dead_code)]
#[derive(Debug)]
pub enum IoError {
    PathNotFound(String),
    PermissionDenied(String),
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IoError::PathNotFound(s) => write!(f, "路径不存在: {}", s),
            IoError::PermissionDenied(s) => write!(f, "权限不足: {}", s),
        }
    }
}

/// JSON 解析错误占位符（如需分类使用）
#[allow(dead_code)]
#[derive(Debug)]
pub enum JsonError {
    ParseError(String),
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonError::ParseError(s) => write!(f, "JSON 解析错误: {}", s),
        }
    }
}

pub type AppResult<T> = Result<T, anyhow::Error>;
