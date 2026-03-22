/// RPG Maker 特殊代码解析器
/// 用于识别和保护 RPG Maker 中的特殊代码，如 \V[1]、\N[1]、\C[2] 等

/// 解析结果
#[derive(Debug, Clone, PartialEq)]
pub struct ParseResult {
    /// 原始文本
    pub original: String,
    /// 处理后的文本（特殊代码被标记）
    pub processed: String,
    /// 特殊代码映射
    pub special_codes: Vec<(usize, String)>, // (位置, 特殊代码)
}

/// 解析 RPG Maker 特殊代码
/// 
/// # 示例
/// ```
/// let text = "Hello \V[1], your name is \N[1]!";
/// let result = rpg_parser::parse_special_codes(text);
/// assert_eq!(result.original, text);
/// assert_eq!(result.special_codes.len(), 2);
/// ```
pub fn parse_special_codes(text: &str) -> ParseResult {
    let mut result = ParseResult {
        original: text.to_string(),
        processed: String::new(),
        special_codes: Vec::new(),
    };

    let mut i = 0;
    let chars: Vec<char> = text.chars().collect();

    while i < chars.len() {
        if chars[i] == '\\' && i + 1 < chars.len() {
            let start = i;
            let mut code = String::from('\\');
            i += 1;

            // 处理特殊代码
            if i < chars.len() {
                let code_char = chars[i];
                code.push(code_char);
                i += 1;

                // 处理带参数的代码，如 \V[1]
                if i < chars.len() && chars[i] == '[' {
                    code.push('[');
                    i += 1;
                    while i < chars.len() && chars[i] != ']' {
                        code.push(chars[i]);
                        i += 1;
                    }
                    if i < chars.len() {
                        code.push(']');
                        i += 1;
                    }
                }
            }

            // 记录特殊代码
            result.special_codes.push((result.processed.len(), code.clone()));
            // 用占位符替换特殊代码，避免翻译
            result.processed.push_str(&format!("[SPECIAL_CODE_{}]", result.special_codes.len() - 1));
        } else {
            result.processed.push(chars[i]);
            i += 1;
        }
    }

    result
}

/// 恢复特殊代码
/// 
/// # 示例
/// ```
/// let text = "Hello [SPECIAL_CODE_0], your name is [SPECIAL_CODE_1]!";
/// let special_codes = vec![(7, "\\V[1]".to_string()), (25, "\\N[1]".to_string())];
/// let result = rpg_parser::restore_special_codes(text, &special_codes);
/// assert_eq!(result, "Hello \\V[1], your name is \\N[1]!");
/// ```
pub fn restore_special_codes(text: &str, special_codes: &[(usize, String)]) -> String {
    let mut result = String::new();
    let mut last_pos = 0;

    // 按位置排序特殊代码
    let mut sorted_codes: Vec<(usize, &String)> = special_codes
        .iter()
        .map(|(pos, code)| (*pos, code))
        .collect();
    sorted_codes.sort_by(|a, b| a.0.cmp(&b.0));

    for (pos, code) in sorted_codes {
        if pos >= last_pos {
            // 添加普通文本
            if pos > last_pos {
                result.push_str(&text[last_pos..pos]);
            }
            // 添加特殊代码
            result.push_str(code);
            last_pos = pos + format!("[SPECIAL_CODE_{}]", special_codes.iter().position(|(p, _)| *p == pos).unwrap()).len();
        }
    }

    // 添加剩余的普通文本
    if last_pos < text.len() {
        result.push_str(&text[last_pos..]);
    }

    result
}

/// 检测文本是否包含 RPG Maker 特殊代码
/// 
/// # 示例
/// ```
/// let text1 = "Hello \V[1]";
/// let text2 = "Hello world";
/// assert_eq!(rpg_parser::contains_special_codes(text1), true);
/// assert_eq!(rpg_parser::contains_special_codes(text2), false);
/// ```
pub fn contains_special_codes(text: &str) -> bool {
    text.contains('\\') && {
        let chars: Vec<char> = text.chars().collect();
        for i in 0..chars.len() {
            if chars[i] == '\\' && i + 1 < chars.len() {
                let code_char = chars[i + 1];
                // 常见的 RPG Maker 特殊代码前缀
                if "VNCWITFBSMP\".contains(code_char) {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_special_codes() {
        let text = "Hello \\V[1], your name is \\N[1]!";
        let result = parse_special_codes(text);
        assert_eq!(result.original, text);
        assert!(result.processed.contains("[SPECIAL_CODE_0]"));
        assert!(result.processed.contains("[SPECIAL_CODE_1]"));
        assert_eq!(result.special_codes.len(), 2);
    }

    #[test]
    fn test_restore_special_codes() {
        let text = "Hello [SPECIAL_CODE_0], your name is [SPECIAL_CODE_1]!";
        let special_codes = vec![(7, "\\V[1]".to_string()), (25, "\\N[1]".to_string())];
        let result = restore_special_codes(text, &special_codes);
        assert_eq!(result, "Hello \\V[1], your name is \\N[1]!");
    }

    #[test]
    fn test_contains_special_codes() {
        let text1 = "Hello \\V[1]";
        let text2 = "Hello world";
        assert_eq!(contains_special_codes(text1), true);
        assert_eq!(contains_special_codes(text2), false);
    }

    #[test]
    fn test_complex_special_codes() {
        let text = "\\C[2]Hello\\C[0] \\V[1] \\N[1] \\I[1]";
        let result = parse_special_codes(text);
        assert_eq!(result.special_codes.len(), 4);
        let restored = restore_special_codes(&result.processed, &result.special_codes);
        assert_eq!(restored, text);
    }
}
