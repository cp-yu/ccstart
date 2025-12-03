use crate::db::Database;
use crate::error::AppResult;

/// 列出所有可用的配置名称（从 SQLite 查询）
pub fn list_configs() -> AppResult<()> {
    let db = Database::open()?;
    let names = db.providers().list_names()?;

    if names.is_empty() {
        eprintln!("错误: 数据库中没有 Claude 配置");
        eprintln!("提示: 请先在 cc-switch 中添加配置");
        std::process::exit(1);
    }

    for name in names {
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
