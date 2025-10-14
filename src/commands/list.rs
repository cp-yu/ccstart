use crate::config::manager::ConfigManager;
use crate::error::AppResult;

/// 列出所有可用的配置名称
pub fn list_configs() -> AppResult<()> {
    let configs = ConfigManager::list_configs()?;

    if configs.is_empty() {
        eprintln!("错误: 未找到配置");
        eprintln!("提示: 请先运行 'ccstart init' 初始化配置");
        std::process::exit(1);
    }

    // 输出到 stdout，每行一个配置名称
    // 包含空格或特殊字符的名称使用双引号包裹
    for name in configs {
        if needs_quoting(&name) {
            println!("\"{}\"", name);
        } else {
            println!("{}", name);
        }
    }

    Ok(())
}

/// 判断配置名称是否需要双引号包裹
fn needs_quoting(name: &str) -> bool {
    name.contains(' ')
        || name.contains('\t')
        || name.contains('\n')
        || name.contains('"')
        || name.contains('\'')
        || name.contains('\\')
        || name.contains('*')
        || name.contains('?')
        || name.contains('[')
        || name.contains(']')
        || name.contains('(')
        || name.contains(')')
        || name.contains('{')
        || name.contains('}')
        || name.contains('$')
        || name.contains('&')
        || name.contains('|')
        || name.contains(';')
        || name.contains('<')
        || name.contains('>')
}
