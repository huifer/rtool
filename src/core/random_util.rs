use rand::Rng;

pub struct RandomUtil;

impl RandomUtil {
    /// 生成指定范围内的随机整数。
    ///
    /// # 参数
    ///
    /// * `min`: 整数范围的下限值（包括该值）。
    /// * `max`: 整数范围的上限值（包括该值）。
    ///
    /// # 返回值
    ///
    /// 返回生成的随机整数。
    pub fn random_int(min: i32, max: i32) -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(min..=max)
    }

    /// 生成指定范围内的随机浮点数。
    ///
    /// # 参数
    ///
    /// * `min`: 浮点数范围的下限值（包括该值）。
    /// * `max`: 浮点数范围的上限值（包括该值）。
    ///
    /// # 返回值
    ///
    /// 返回生成的随机浮点数。
    pub fn random_float(min: f64, max: f64) -> f64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(min..=max)
    }

    /// 生成随机布尔值。
    ///
    /// # 返回值
    ///
    /// 返回生成的随机布尔值。
    pub fn random_bool() -> bool {
        let mut rng = rand::thread_rng();
        rng.gen()
    }

    /// 生成随机小写字母。
    ///
    /// # 返回值
    ///
    /// 返回生成的随机小写字母。
    pub fn random_char() -> char {
        let mut rng = rand::thread_rng();
        rng.gen_range(b'a'..=b'z') as char
    }

    /// 生成指定长度的随机字符串，包含小写字母、大写字母和数字。
    ///
    /// # 参数
    ///
    /// * `length`: 字符串的长度。
    ///
    /// # 返回值
    ///
    /// 返回生成的随机字符串。
    pub fn random_string(length: usize) -> String {
        let mut rng = rand::thread_rng();
        let characters: Vec<char> = (0..length)
            .map(|_| {
                let category = rng.gen_range(0..3);
                match category {
                    0 => rng.gen_range(b'a'..=b'z') as char,
                    1 => rng.gen_range(b'A'..=b'Z') as char,
                    2 => rng.gen_range(b'0'..=b'9') as char,
                    _ => unreachable!(),
                }
            })
            .collect();

        characters.into_iter().collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_int() {
        let random_int = RandomUtil::random_int(1, 100);
        assert!(random_int >= 1 && random_int <= 100);
    }

    #[test]
    fn test_random_float() {
        let random_float = RandomUtil::random_float(1.0, 100.0);
        assert!(random_float >= 1.0 && random_float <= 100.0);
    }

    #[test]
    fn test_random_bool() {
        let random_bool = RandomUtil::random_bool();
        assert!(random_bool == true || random_bool == false);
    }

    #[test]
    fn test_random_char() {
        let random_char = RandomUtil::random_char();
        assert!((b'a'..=b'z').contains(&(random_char as u8)));
    }

    #[test]
    fn test_random_string() {
        let random_string = RandomUtil::random_string(10);
        assert_eq!(random_string.len(), 10);
    }
}
