use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::sha1::Sha1;

pub struct SecureUtil;

impl SecureUtil {
    /// 使用 MD5 哈希算法计算输入字符串的哈希值。
    ///
    /// # 参数
    ///
    /// * `input`: &str - 待计算哈希值的字符串
    ///
    /// # 返回值
    ///
    /// 返回一个包含输入字符串 MD5 哈希值的 String。
    ///
    pub fn md5(input: &str) -> String {
        let mut md5 = Md5::new();
        md5.input(input.as_ref());
        md5.result_str()
    }
    /// 使用 SHA-1 哈希算法计算输入字符串的哈希值。
    ///
    /// # 参数
    ///
    /// * `input`: &str - 待计算哈希值的字符串
    ///
    /// # 返回值
    ///
    /// 返回一个包含输入字符串 SHA-1 哈希值的 String。
    ///
    pub fn sha1(input: &str) -> String {
        let mut sha1 = Sha1::new();
        sha1.input_str(input);
        sha1.result_str()
    }
}


#[cfg(test)]
mod tests {
    use crate::secure::secure::SecureUtil;

    #[test]
    fn test_md5() {
        let input = "Hello, World!";
        let expected_output = "65a8e27d8879283831b664bd8b7f0ad4";
        assert_eq!(SecureUtil::md5(input), expected_output);
    }

    #[test]
    fn test_sha1() {
        let input = "Hello, World!";
        let expected_output = "0a0a9f2a6772942557ab5355d76af442f8f65e01";
        assert_eq!(SecureUtil::sha1(input), expected_output);
    }
}
