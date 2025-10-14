use anyhow::anyhow;
use percent_encoding::{percent_decode_str, percent_encode, AsciiSet, CONTROLS};

// 需要进行百分号编码的字符集（保留空格、字母、数字、连字符、下划线）
const FORBIDDEN: &AsciiSet = &CONTROLS
    .add(b'/')
    .add(b':')
    .add(b'*')
    .add(b'?')
    .add(b'"')
    .add(b'<')
    .add(b'>')
    .add(b'|')
    .add(b'\\');

/// 将配置名称编码为安全的文件名片段
pub fn encode_config_name(name: &str) -> String {
    percent_encode(name.as_bytes(), FORBIDDEN).to_string()
}

/// 解码配置名称中的百分号编码
pub fn decode_config_name(encoded: &str) -> anyhow::Result<String> {
    Ok(percent_decode_str(encoded)
        .decode_utf8()
        .map_err(|e| anyhow!("解码配置名称失败: {}", e))?
        .to_string())
}

