pub struct Desensitized {}

impl Desensitized {
    /// 对输入字符串进行脱敏处理。
    ///
    /// # 参数
    ///
    /// - `input`: 待脱敏的字符串。
    /// - `start_index`: 脱敏保留的起始索引，从0开始。
    /// - `end_index`: 脱敏保留的结束索引，从字符串末尾往前数。
    ///
    /// # 返回值
    ///
    /// 返回经过脱敏处理后的字符串，其中保留了起始索引和末尾索引的字符，其他字符用 '*' 代替。
    ///
    pub fn desensitize(input: &str, start_index: usize, end_index: usize) -> String {
        let mut result = String::new();

        for (i, c) in input.chars().enumerate() {
            if i < start_index || i >= input.len() - end_index {
                result.push(c);
            } else {
                result.push('*');
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Desensitized;

    #[test]
    fn test_desensitize() {
        let result1 = Desensitized::desensitize("hello world", 2, 3);
        assert_eq!(result1, "he******rld");

        let result2 = Desensitized::desensitize("example123", 0, 4);
        assert_eq!(result2, "******e123");

        let result3 = Desensitized::desensitize("confidential", 5, 5);
        assert_eq!(result3, "confi**ntial");
    }
}
