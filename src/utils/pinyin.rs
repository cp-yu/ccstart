use pinyin::ToPinyin;

/// Extract pinyin initials from a string.
/// Chinese characters → lowercase first letter of pinyin.
/// ASCII characters → lowercase.
/// Other characters → ignored.
pub fn pinyin_initials(name: &str) -> String {
    let mut out = String::new();
    for ch in name.chars() {
        if ch.is_ascii() {
            if ch.is_ascii_alphabetic() {
                out.push(ch.to_ascii_lowercase());
            }
        } else if let Some(py) = ch.to_pinyin() {
            let plain = py.plain();
            if let Some(first) = plain.chars().next() {
                out.push(first);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pinyin_initials_chinese() {
        assert_eq!(pinyin_initials("小丑"), "xc");
        assert_eq!(pinyin_initials("钟阮"), "zr");
    }

    #[test]
    fn pinyin_initials_ascii() {
        assert_eq!(pinyin_initials("packyapi"), "packyapi");
    }

    #[test]
    fn pinyin_initials_mixed() {
        assert_eq!(pinyin_initials("test测试"), "testcs");
    }

    #[test]
    fn pinyin_initials_empty() {
        assert_eq!(pinyin_initials(""), "");
    }

    #[test]
    fn pinyin_initials_digits_ignored() {
        assert_eq!(pinyin_initials("test123"), "test");
    }
}
