use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime, Utc, Weekday};

#[allow(dead_code)]
pub enum DateTimeFormat {
    YmdHMS,
    // "yyyy-mm-dd hh:mm:ss" => "%Y-%m-%d %H:%M:%S"
    YmdHM,
    // "yyyy-mm-dd hh:mm"    => "%Y-%m-%d %H:%M"
    YmdH,
    // "yyyy-mm-dd hh"       => "%Y-%m-%d %H"
    Ymd,
    // "yyyy-mm-dd"          => "%Y-%m-%d"
    Ym,
    // "yyyy-mm"             => "%Y-%m"
    Md,
    // "mm-dd"               => "%m-%d"
    DMYHMS,
    // "dd-mm-yy hh:mm:ss"   => "%d-%m-%y %H:%M:%S"
    YmdHmsSlash,
    // "yyyy/mm/dd hh:mm:ss" => "%Y/%m/%d %H:%M:%S"
    YmdHmSlash,
    // "yyyy/mm/dd hh:mm"    => "%Y/%m/%d %H:%M"
    YmdHSlash,
    // "yyyy/mm/dd hh"       => "%Y/%m/%d %H"
    YmdSlash,
    // "yyyy/mm/dd"          => "%Y/%m/%d"
    YmSlash,
    // "yyyy/mm"             => "%Y/%m"
    MdSlash,
    // "mm/dd"               => "%m/%d"
    DmyhmsSlash,
    // "dd/mm/yy hh:mm:ss"   => "%d/%m/%y %H:%M:%S"
    Y,
    // "yyyy"                => "%Y"
    M,
    // "mm"                  => "%m"
    HMS,
    // "hh:mm:ss"            => "%H:%M:%S"
    MS,              // "mm:ss"               => "%m:%S"
}

impl DateTimeFormat {
    fn to_str(&self) -> &'static str {
        match self {
            Self::YmdHMS => "%Y-%m-%d %H:%M:%S",
            Self::YmdHM => "%Y-%m-%d %H:%M",
            Self::YmdH => "%Y-%m-%d %H",
            Self::Ymd => "%Y-%m-%d",
            Self::Ym => "%Y-%m",
            Self::Md => "%m-%d",
            Self::DMYHMS => "%d-%m-%y %H:%M:%S",
            Self::YmdHmsSlash => "%Y/%m/%d %H:%M:%S",
            Self::YmdHmSlash => "%Y/%m/%d %H:%M",
            Self::YmdHSlash => "%Y/%m/%d %H",
            Self::YmdSlash => "%Y/%m/%d",
            Self::YmSlash => "%Y/%m",
            Self::MdSlash => "%m/%d",
            Self::DmyhmsSlash => "%d/%m/%y %H:%M:%S",
            Self::Y => "%Y",
            Self::M => "%m",
            Self::HMS => "%H:%M:%S",
            Self::MS => "%m:%S",
        }
    }
}

#[allow(dead_code)]
pub enum TimeUnit {
    Years,
    Months,
    Days,
    Hours,
    Minutes,
    Seconds,
    Weeks,
}


pub struct DateUtil {}

#[allow(dead_code)]
impl DateUtil {


    /// 获取结束年的时间
    ///
    /// # 参数
    ///
    /// * `input_datetime`: 输入日期时间
    ///
    /// 返回值: 结束年的时间
    pub fn get_end_of_year(input_datetime: NaiveDateTime) -> NaiveDateTime {
        // 获取年份
        let year = input_datetime.date().year();

        // 创建一个 NaiveDateTime，将月、日、小时、分钟、秒部分设置为12月31日 23:59:59
        let end_of_year = NaiveDateTime::new(NaiveDate::from_ymd_opt(year, 12, 31).unwrap(), chrono::NaiveTime::from_hms_opt(23, 59, 59).unwrap());

        end_of_year
    }

    /// 获取结束月的时间
    ///
    /// # 参数
    ///
    /// * `input_datetime`: 输入日期时间
    ///
    /// 返回值: 结束月的时间
    pub fn get_end_of_month(input_datetime: NaiveDateTime) -> NaiveDateTime {
        // 获取年份和月份
        let year = input_datetime.date().year();
        let month = input_datetime.date().month();

        // 获取该月的天数
        let days_in_month = NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap_or_else(|| NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()).pred_opt().unwrap().day();

        // 创建一个 NaiveDateTime，将日、小时、分钟、秒部分设置为该月最后一天 23:59:59
        let end_of_month = NaiveDateTime::new(NaiveDate::from_ymd_opt(year, month, days_in_month).unwrap(), chrono::NaiveTime::from_hms_opt(23, 59, 59).unwrap());

        end_of_month
    }

    /// 获取结束周的时间
    ///
    /// # 参数
    ///
    /// * `input_datetime`: 输入日期时间
    ///
    /// 返回值: 结束周的时间
    pub fn get_end_of_week(input_datetime: NaiveDateTime) -> NaiveDateTime {
        // 获取输入日期时间的星期几
        let weekday = input_datetime.date().weekday();

        // 计算距离该星期结束的天数
        let days_to_end = match weekday {
            Weekday::Mon => 6,
            Weekday::Tue => 5,
            Weekday::Wed => 4,
            Weekday::Thu => 3,
            Weekday::Fri => 2,
            Weekday::Sat => 1,
            Weekday::Sun => 0,
        };

        // 创建一个 NaiveDateTime，将小时、分钟、秒部分设置为星期结束的时刻
        let end_of_week = input_datetime + Duration::days(days_to_end as i64);
        let end_of_week = NaiveDateTime::new(end_of_week.date(), chrono::NaiveTime::from_hms_opt(23, 59, 59).unwrap());

        end_of_week
    }



    /// 获取开始年的时间
    ///
    /// # 参数
    ///
    /// * `input_datetime`: 输入日期时间
    ///
    /// 返回值: 开始年的时间
    pub fn get_start_of_year(input_datetime: NaiveDateTime) -> NaiveDateTime {
        // 获取年份
        let year = input_datetime.date().year();

        // 创建一个 NaiveDateTime，将月、日、小时、分钟、秒部分设置为1月1日 0:0:0
        let start_of_year = NaiveDateTime::new(NaiveDate::from_ymd_opt(year, 1, 1).unwrap(), chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap());

        start_of_year
    }

    /// 获取开始月的时间
    ///
    /// # 参数
    ///
    /// * `input_datetime`: 输入日期时间
    ///
    /// 返回值: 开始月的时间
    pub fn get_start_of_month(input_datetime: NaiveDateTime) -> NaiveDateTime {
        // 获取年份和月份
        let year = input_datetime.date().year();
        let month = input_datetime.date().month();
        // 创建一个 NaiveDateTime，将日、小时、分钟、秒部分设置为该月1日 0:0:0
        let start_of_month = NaiveDateTime::new(NaiveDate::from_ymd_opt(year, month, 1).unwrap(), chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap());

        start_of_month
    }

    /// 获取开始周的时间
    ///
    /// # 参数
    ///
    /// * `input_datetime`: 输入日期时间
    ///
    /// 返回值: 开始周的时间
    pub fn get_start_of_week(input_datetime: NaiveDateTime) -> NaiveDateTime {
        // 获取输入日期时间的星期几
        let weekday = input_datetime.date().weekday();

        // 计算距离该星期开始的天数
        let days_to_start = match weekday {
            Weekday::Mon => 0,
            Weekday::Tue => 1,
            Weekday::Wed => 2,
            Weekday::Thu => 3,
            Weekday::Fri => 4,
            Weekday::Sat => 5,
            Weekday::Sun => 6,
        };

        // 创建一个 NaiveDateTime，将小时、分钟、秒部分设置为星期开始的时刻
        let start_of_week = input_datetime - Duration::days(days_to_start as i64);
        let start_of_week = NaiveDateTime::new(start_of_week.date(), chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap());

        start_of_week
    }

    /// 获取一天中的起始时间
    ///
    /// # 参数
    ///
    /// * `input_date`: 输入日期（不包含时间部分）
    ///
    /// 返回值: 一天中的起始时间
    pub fn get_start_of_day_for_day(input_date: NaiveDate) -> NaiveDateTime {
        // 创建一个 NaiveDateTime，将小时、分钟、秒部分设置为零
        let start_of_day = NaiveDateTime::new(input_date, chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap());

        start_of_day
    }


    /// 获取一天的结束时间
    ///
    /// # 参数
    ///
    /// * `input_date`: 输入日期（不包含时间部分）
    ///
    /// 返回值: 一天的结束时间
    pub fn get_end_of_day_for_day(input_date: NaiveDate) -> NaiveDateTime {
        // 创建一个 NaiveDateTime，将小时、分钟、秒部分设置为23:59:59
        let end_of_day = NaiveDateTime::new(input_date, chrono::NaiveTime::from_hms_opt(23, 59, 59).unwrap());

        end_of_day
    }

    /// 获取一天的起始时间
    ///
    /// # 参数
    ///
    /// * `input_datetime`: 输入日期时间
    ///
    /// 返回值: 一天的起始时间
    pub fn get_start_of_day(input_datetime: NaiveDateTime) -> NaiveDateTime {
        // 获取日期部分
        let date_part = input_datetime.date();

        // 创建一个 NaiveDateTime，将小时、分钟、秒部分设置为0:0:0
        let start_of_day = NaiveDateTime::new(date_part, chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap());

        start_of_day
    }

    /// 获取一天的结束时间
    ///
    /// # 参数
    ///
    /// * `input_datetime`: 输入日期时间
    ///
    /// 返回值: 一天的结束时间
    pub fn get_end_of_day(input_datetime: NaiveDateTime) -> NaiveDateTime {
        // 获取日期部分
        let date_part = input_datetime.date();

        // 创建一个 NaiveDateTime，将小时、分钟、秒部分设置为23:59:59
        let end_of_day = NaiveDateTime::new(date_part, chrono::NaiveTime::from_hms_opt(23, 59, 59).unwrap());

        end_of_day
    }





    /// 对 NaiveDateTime 进行时间加减法
    ///
    /// # 参数
    ///
    /// * `input_time`: 输入时间
    /// * `step`: 步长，可以是正数或负数
    /// * `unit`: 时间单位，使用 TimeUnit 枚举表示
    ///
    /// 返回值: 计算后的时间
    pub fn manipulate_time(input_time: NaiveDateTime, step: i64, unit: TimeUnit) -> NaiveDateTime {
        // 定义时间间隔
        let duration = match unit {
            TimeUnit::Years => Duration::days(step * 365),
            TimeUnit::Months => Duration::days(step * 30),
            TimeUnit::Days => Duration::days(step),
            TimeUnit::Hours => Duration::hours(step),
            TimeUnit::Minutes => Duration::minutes(step),
            TimeUnit::Seconds => Duration::seconds(step),
            TimeUnit::Weeks => Duration::weeks(step),
        };

        // 进行时间加减法操作
        input_time + duration
    }
    /// 解析时间字符串
    ///
    /// # 参数
    ///
    /// * `input`: 待解析的时间字符串
    /// * `format_str`: 时间格式
    ///
    /// 返回值: 解析后的时间，以 NaiveDateTime 格式表示
    ///
    pub fn parse_time(input: String, format_str: DateTimeFormat) -> Option<NaiveDateTime> {
        let parsed_time = NaiveDateTime::parse_from_str(input.as_str(), format_str.to_str());

        return match parsed_time {
            Ok(parsed_time) => { Option::from(parsed_time) }

            Err(err) => {
                println!("时间转换异常 ， {}", err);
                Option::None
            }
        }
    }


    /// 获取当前时间
    ///
    /// # 参数
    ///
    /// * `format_str`: 目标格式
    ///
    /// 返回值: String 当前时间的字符串表示
    ///
    pub fn get_now_time_string(format_str: DateTimeFormat) -> String {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let formatted_time = chrono::DateTime::<chrono::Local>::from(UNIX_EPOCH + std::time::Duration::from_secs(current_time))
            .format(format_str.to_str())
            .to_string();
        return formatted_time;
    }


    pub fn now() -> NaiveDateTime {
        Local::now().naive_local()
    }

    /// 获取当前 UTC 时间
    ///
    /// # 参数
    ///
    /// * `format_str`: 目标格式
    ///
    /// 返回值: String 当前 UTC 时间的字符串表示
    ///
    pub fn get_now_time_utc_string(format_str: DateTimeFormat) -> String {
        let current_time = Utc::now();
        current_time.format(format_str.to_str()).to_string()
    }
}


#[cfg(test)]
mod tests {
    use crate::date::date_util::DateTimeFormat;
    use crate::date::date_util::DateUtil;
    use crate::date::date_util::TimeUnit;

    #[test]
    fn test_date_util() {
        // 示例用法
        println!("当前日期：{}", DateUtil::get_now_time_string(DateTimeFormat::YmdHMS));
        let s = DateUtil::get_now_time_utc_string(DateTimeFormat::YmdHMS);
        println!("当前日期：{}", s);

        let time = DateUtil::parse_time(s, DateTimeFormat::YmdHMS);
        println!("当前日期：{}", time.unwrap());


        let time1 = DateUtil::manipulate_time(DateUtil::now(), -1, TimeUnit::Days);
        println!("计算后日期：{}", time1);

        // 获取当前日期时间
        let current_datetime = chrono::Utc::now().naive_utc();

        // 获取开始年的时间
        let start_of_year = DateUtil::get_start_of_year(current_datetime);

        // 获取开始月的时间
        let start_of_month = DateUtil::get_start_of_month(current_datetime);

        // 获取开始周的时间
        let start_of_week = DateUtil::get_start_of_week(current_datetime);
        // 获取结束年的时间
        let end_of_year = DateUtil::get_end_of_year(current_datetime);

        // 获取结束月的时间
        let end_of_month = DateUtil::get_end_of_month(current_datetime);

        // 获取结束周的时间
        let end_of_week = DateUtil::get_end_of_week(current_datetime);

        // 打印结果
        println!("当前日期时间：{}", current_datetime);
        println!("开始年的时间：{}", start_of_year);
        println!("开始月的时间：{}", start_of_month);
        println!("开始周的时间：{}", start_of_week);
        println!("结束年的时间：{}", end_of_year);
        println!("结束月的时间：{}", end_of_month);
        println!("结束周的时间：{}", end_of_week);
    }
}