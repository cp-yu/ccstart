use crate::utils::pinyin::pinyin_initials;

pub enum ResolveResult {
    Exact(String),
    PinyinUnique(String),
    PinyinCollision(Vec<String>),
    NotFound,
}

pub fn resolve_channel(input: &str, channels: &[String]) -> ResolveResult {
    if channels.iter().any(|n| n == input) {
        return ResolveResult::Exact(input.to_owned());
    }

    if !input.bytes().all(|b| b.is_ascii_alphabetic()) || input.is_empty() {
        return ResolveResult::NotFound;
    }

    let lower = input.to_ascii_lowercase();
    let matches: Vec<String> = channels
        .iter()
        .filter(|name| pinyin_initials(name).starts_with(&lower))
        .cloned()
        .collect();

    match matches.len() {
        0 => ResolveResult::NotFound,
        1 => ResolveResult::PinyinUnique(matches.into_iter().next().unwrap()),
        _ => ResolveResult::PinyinCollision(matches),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_exact() {
        let channels = vec!["小丑".into(), "packyapi".into()];
        match resolve_channel("小丑", &channels) {
            ResolveResult::Exact(name) => assert_eq!(name, "小丑"),
            _ => panic!("expected Exact"),
        }
    }

    #[test]
    fn resolve_pinyin_unique() {
        let channels = vec!["小丑".into(), "钟阮".into(), "packyapi".into()];
        match resolve_channel("xc", &channels) {
            ResolveResult::PinyinUnique(name) => assert_eq!(name, "小丑"),
            _ => panic!("expected PinyinUnique"),
        }
    }

    #[test]
    fn resolve_pinyin_collision() {
        let channels = vec!["小丑".into(), "新城".into(), "packyapi".into()];
        match resolve_channel("xc", &channels) {
            ResolveResult::PinyinCollision(names) => {
                assert!(names.contains(&"小丑".to_owned()));
                assert!(names.contains(&"新城".to_owned()));
            }
            _ => panic!("expected PinyinCollision"),
        }
    }

    #[test]
    fn resolve_not_found() {
        let channels = vec!["小丑".into(), "packyapi".into()];
        assert!(matches!(resolve_channel("zz", &channels), ResolveResult::NotFound));
    }

    #[test]
    fn resolve_non_ascii_no_pinyin_fallback() {
        let channels = vec!["小丑".into(), "packyapi".into()];
        assert!(matches!(resolve_channel("不存在", &channels), ResolveResult::NotFound));
    }
}
