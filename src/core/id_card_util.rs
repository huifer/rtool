use chrono::{Datelike, NaiveDate, Utc};
use regex::Regex;

#[derive(Debug)]
#[allow(dead_code)]
pub struct IdCardInfo {
    birthday: NaiveDate,
    // 存储身份证持有人的生日日期
    age: u32,
    // 存储身份证持有人的年龄
    gender: &'static str, // 存储身份证持有人的性别，"男"或"女"
}

pub struct IdCardUtil {}

impl IdCardUtil {
    /// 从身份证号码中提取信息并返回一个包含生日、年龄和性别的结构体
    ///
    /// # 参数
    ///
    /// - `id_card`：身份证号码字符串
    ///
    /// # 返回值
    ///
    /// 如果成功提取信息，则返回`Some(IdCardInfo)`，否则返回`None`
    pub fn extract_id_card_info(id_card: &str) -> Option<IdCardInfo> {
        // 定义身份证号码的正则表达式
        let id_card_regex = Regex::new(r"^(\d{6})(\d{4})(\d{2})(\d{2})(\d{3})([0-9Xx])$").unwrap();

        // 匹配正则表达式
        if let Some(captures) = id_card_regex.captures(id_card) {
            // 提取生日信息
            let birth_year_str = captures.get(2)?.as_str();
            let birth_month_str = captures.get(3)?.as_str();
            let birth_day_str = captures.get(4)?.as_str();

            // 格式化生日字符串为YYYYMMDD的形式
            let birthday_str = format!("{}{}{}", birth_year_str, birth_month_str, birth_day_str);
            let birthday = NaiveDate::parse_from_str(&birthday_str, "%Y%m%d").ok()?;

            // 提取性别信息
            let gender_digit: u32 = captures.get(3)?.as_str().parse().ok()?;
            let gender = if gender_digit % 2 == 0 { "女" } else { "男" };

            // 计算年龄
            let birth_year: i32 = captures.get(2)?.as_str().parse().ok()?;
            let current_year = Utc::now().year();
            let age = current_year - birth_year as i32;

            let id_card_info = IdCardInfo {
                birthday,
                age: age as u32,
                gender,
            };

            return Some(id_card_info);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// 测试从身份证号码中提取信息的函数
    fn test_extract_id_card_info() {
        // 准备测试数据
        let id_card = "150526200001010797";

        // 调用提取信息的函数
        let result = IdCardUtil::extract_id_card_info(id_card);

        // 断言：确保成功提取信息并且返回的信息正确
        assert!(result.is_some());
        let id_card_info = result.unwrap();
        // assert_eq!(id_card_info.age, 23); // 根据当前年份和生日计算的年龄
        assert_eq!(id_card_info.gender, "男");
    }
}
