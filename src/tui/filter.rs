use crate::utils::pinyin::pinyin_initials;

pub fn filter_items<'a>(items: &'a [String], query: &str) -> Vec<&'a String> {
    if query.is_empty() {
        return items.iter().collect();
    }

    let lower = query.to_lowercase();
    let mut result = Vec::new();
    let is_ascii_alpha = lower.bytes().all(|b| b.is_ascii_alphabetic());

    for item in items {
        let item_lower = item.to_lowercase();
        if item_lower.contains(&lower) {
            result.push(item);
        } else if is_ascii_alpha {
            let initials = pinyin_initials(item);
            if initials.contains(&lower) {
                result.push(item);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substring_match() {
        let items = vec!["packycode".into(), "packyapi".into(), "小丑".into()];
        let result = filter_items(&items, "pack");
        assert_eq!(result.len(), 2);
        assert!(result.contains(&&"packycode".to_string()));
        assert!(result.contains(&&"packyapi".to_string()));
    }

    #[test]
    fn test_pinyin_initials_match() {
        let items = vec!["小丑".into(), "packyapi".into(), "钟阮".into()];
        let result = filter_items(&items, "xc");
        assert_eq!(result.len(), 1);
        assert!(result.contains(&&"小丑".to_string()));
    }

    #[test]
    fn test_empty_query_returns_all() {
        let items = vec!["packycode".into(), "小丑".into(), "钟阮".into()];
        let result = filter_items(&items, "");
        assert_eq!(result.len(), 3);
    }
}
