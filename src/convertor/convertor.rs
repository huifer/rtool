use base64::decode;

#[allow(dead_code)]
pub struct Convertor;

impl Convertor {
    /// 将十六进制格式的颜色转换为RGB格式
    ///
    /// # 参数
    ///
    /// * `hex`: &str - 十六进制颜色代码（例如，"#RRGGBB"）
    ///
    /// 返回值：Result<(u8, u8, u8), String> - 表示RGB分量的元组或错误消息
    pub fn color_hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), String> {
        if hex.len() != 7 || !hex.starts_with('#') {
            return Err("颜色转换异常".to_string()); // 返回无效颜色格式的错误消息
        }

        let r = u8::from_str_radix(&hex[1..3], 16).map_err(|_| "红色分量无效")?;
        let g = u8::from_str_radix(&hex[3..5], 16).map_err(|_| "绿色分量无效")?;
        let b = u8::from_str_radix(&hex[5..7], 16).map_err(|_| "蓝色分量无效")?;

        Ok((r, g, b))
    }

    /// 将RGB格式转换为十六进制颜色代码
    ///
    /// # 参数
    ///
    /// * `rgb`: (u8, u8, u8) - 表示RGB分量的元组
    ///
    /// 返回值：String - 十六进制颜色代码
    pub fn color_rgb_to_hex(rgb: (u8, u8, u8)) -> String {
        format!("#{:02X}{:02X}{:02X}", rgb.0, rgb.1, rgb.2)
    }

    /// 将字节数组转换为Base64编码
    ///
    /// # 参数
    ///
    /// * `data`: &[u8] - 待编码的字节数组
    ///
    /// 返回值：String - Base64编码的结果
    pub fn bytes_to_base64(data: &[u8]) -> String {
        let mut encoded = Vec::new();
        data.iter().copied().for_each(|byte| {
            encoded.push(byte as u8);
        });
        base64::encode(&encoded)
    }

    /// 将Base64编码的字符串转换为字节数组
    ///
    /// # 参数
    ///
    /// * `data`: &str - Base64编码的字符串
    ///
    /// 返回值：String - 解码后的字节数组的字符串表示
    pub fn base64_to_bytes(data: &str) -> String {
        let decoded_bytes = decode(data.as_bytes()).unwrap();
        String::from_utf8(decoded_bytes).unwrap()
    }

    /// 将字符串转换为十六进制表示
    ///
    /// # 参数
    ///
    /// * `input`: &str - 待转换的字符串
    ///
    /// 返回值：String - 十六进制表示的结果
    pub fn string_to_hex(input: &str) -> String {
        input.chars().map(|c| format!("{:02X}", c as u8)).collect()
    }

    /// 将十六进制表示转换为字符串
    ///
    /// # 参数
    ///
    /// * `input`: &str - 十六进制表示的字符串
    ///
    /// 返回值：Result<String, std::num::ParseIntError> - 解码后的字符串或错误
    pub fn hex_to_string(input: &str) -> Result<String, std::num::ParseIntError> {
        let bytes: Result<Vec<u8>, _> = (0..input.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&input[i..i + 2], 16))
            .collect();

        Ok(String::from_utf8_lossy(&bytes?).to_string())
    }

    /// 将字符串转换为字节数组
    ///
    /// # 参数
    ///
    /// * `input`: &str - 待转换的字符串
    ///
    /// 返回值：Vec<u8> - 字节数组
    pub fn string_to_bytes(input: &str) -> Vec<u8> {
        input.bytes().collect()
    }

    /// 将字节数组转换为字符串
    ///
    /// # 参数
    ///
    /// * `input`: &[u8] - 待转换的字节数组
    ///
    /// 返回值：String - 字符串表示
    pub fn bytes_to_string(input: &[u8]) -> String {
        String::from_utf8_lossy(input).to_string()
    }

    /// 十进制转换为十六进制
    ///
    /// # 参数
    ///
    /// * `decimal`: u64 - 十进制数
    ///
    /// 返回值：String - 十六进制表示
    pub fn decimal_to_hex(decimal: u64) -> String {
        format!("{:X}", decimal)
    }

    /// 十六进制转换为十进制
    ///
    /// # 参数
    ///
    /// * `hex_string`: &str - 十六进制表示的字符串
    ///
    /// 返回值：Result<u64, std::num::ParseIntError> - 十进制数或错误
    pub fn hex_to_decimal(hex_string: &str) -> Result<u64, std::num::ParseIntError> {
        u64::from_str_radix(hex_string, 16)
    }

    /// 十六进制转换为八进制
    ///
    /// # 参数
    ///
    /// * `hex_string`: &str - 十六进制表示的字符串
    ///
    /// 返回值：Result<String, std::num::ParseIntError> - 八进制表示或错误
    pub fn hex_to_octal(hex_string: &str) -> Result<String, std::num::ParseIntError> {
        let decimal = u64::from_str_radix(hex_string, 16)?;
        Ok(format!("{:o}", decimal))
    }

    /// 八进制转换为十六进制
    ///
    /// # 参数
    ///
    /// * `octal_string`: &str - 八进制表示的字符串
    ///
    /// 返回值：Result<String, std::num::ParseIntError> - 十六进制表示或错误
    pub fn octal_to_hex(octal_string: &str) -> Result<String, std::num::ParseIntError> {
        let decimal = u64::from_str_radix(octal_string, 8)?;
        Ok(format!("{:X}", decimal))
    }

    /// 十六进制转换为二进制
    ///
    /// # 参数
    ///
    /// * `hex_string`: &str - 十六进制表示的字符串
    ///
    /// 返回值：Result<String, std::num::ParseIntError> - 二进制表示或错误
    pub fn hex_to_binary(hex_string: &str) -> Result<String, std::num::ParseIntError> {
        let decimal = u64::from_str_radix(hex_string, 16)?;
        Ok(format!("{:b}", decimal))
    }

    /// 二进制转换为十六进制
    ///
    /// # 参数
    ///
    /// * `binary_string`: &str - 二进制表示的字符串
    ///
    /// 返回值：Result<String, std::num::ParseIntError> - 十六进制表示或错误
    pub fn binary_to_hex(binary_string: &str) -> Result<String, std::num::ParseIntError> {
        let decimal = u64::from_str_radix(binary_string, 2)?;
        Ok(format!("{:X}", decimal))
    }

    /// 半角字符转换为全角字符
    ///
    /// # 参数
    ///
    /// * `input`: &str - 待转换的字符串
    ///
    /// 返回值：String - 转换后的全角字符串
    pub fn halfwidth_to_fullwidth(input: &str) -> String {
        input.chars().map(|c| {
            match c {
                ' ' => '\u{3000}', // 将半角空格转换为全角空格
                _ if c >= '!' && c <= '~' => char::from_u32(c as u32 + 0xFF00 - 0x20).unwrap_or(c),
                _ => c,
            }
        }).collect()
    }

    /// 全角字符转换为半角字符
    ///
    /// # 参数
    ///
    /// * `input`: &str - 待转换的字符串
    ///
    /// 返回值：String - 转换后的半角字符串
    pub fn fullwidth_to_halfwidth(input: &str) -> String {
        input.chars().map(|c| {
            match c {
                '\u{3000}' => ' ', // 将全角空格转换为半角空格
                _ if c >= '\u{FF01}' && c <= '\u{FF5E}' => char::from_u32(c as u32 - 0xFF00 + 0x20).unwrap_or(c),
                _ => c,
            }
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::convertor::convertor::Convertor;

    #[test]
    fn test_color() {
        // check color
        let color = "#000000";
        if let Ok((r, g, b)) = Convertor::color_hex_to_rgb(color) {
            assert_eq!(r, 0);
            assert_eq!(g, 0);
            assert_eq!(b, 0);
        } else {
            assert!(false);
        }

        let hex = Convertor::color_rgb_to_hex((0, 0, 0));
        assert_eq!(hex, "#000000");


        let color2 = "#0000001";
        let result = Convertor::color_hex_to_rgb(color2);
        assert_eq!(result.err().unwrap(), "颜色转换异常");
    }

    #[test]
    fn test_base64() {
        // check string base64
        let string = "hello world";
        let base64 = Convertor::bytes_to_base64(string.as_bytes());
        assert_eq!(base64, "aGVsbG8gd29ybGQ=");

        let string = Convertor::base64_to_bytes("aGVsbG8gd29ybGQ=");

        assert_eq!(string, "hello world");
    }

    #[test]
    fn test_hex() {
        //     字符串和16进制之间的转换验证
        let hex = "aklj1";
        let string = Convertor::string_to_hex(hex);
        assert_eq!(string, "616b6c6a31".to_uppercase());

        let string1 = Convertor::hex_to_string(string.as_str()).unwrap();
        assert_eq!(string1, hex);
        //     16进制和10进制
        let hex = "616b6c6a31";
        let decimal = Convertor::hex_to_decimal(hex).unwrap();
        assert_eq!(decimal, 418414094897);
        let hex1 = Convertor::decimal_to_hex(decimal);
        assert_eq!(hex1, hex.to_uppercase());
        //     16进制和8进制
        let octal = Convertor::hex_to_octal(hex).unwrap();
        assert_eq!(octal, "6055333065061");
        let string2 = Convertor::octal_to_hex(octal.as_str()).unwrap();
        assert_eq!(string2, hex.to_uppercase());
        //     16进制和2进制
        let binary = Convertor::hex_to_binary(hex).unwrap();
        assert_eq!(binary, "110000101101011011011000110101000110001");
        let string3 = Convertor::binary_to_hex(binary.as_str()).unwrap();
        assert_eq!(string3, hex.to_uppercase());
    }

    #[test]
    fn test_string_byte() {
        let s = "hello";
        let vec = Convertor::string_to_bytes(s);
        let bytes_vec: Vec<u8> = vec![104, 101, 108, 108, 111];

        assert_eq!(vec, bytes_vec);
        let string = Convertor::bytes_to_string(&[104, 101, 108, 108, 111]);
        assert_eq!(string, s);
    }

    #[test]
    fn halfwidth_fullwidth() {
        let string = Convertor::halfwidth_to_fullwidth("Hello, World!");
        assert_eq!(string, "Ｈｅｌｌｏ，　Ｗｏｒｌｄ！");
        assert_eq!(Convertor::fullwidth_to_halfwidth("Ｈｅｌｌｏ，　Ｗｏｒｌｄ！"), "Hello, World!");
    }
}