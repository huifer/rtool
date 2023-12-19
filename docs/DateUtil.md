# DateUtil 模块

`DateUtil` 模块提供了一些有关日期和时间的常用工具，包括获取特定时间点、时间范围的函数以及对时间进行加减法、时间字符串解析等功能。

## 获取特定时间点

### `get_end_of_year` 函数

#### 作用

获取输入日期时间所在年的结束时间。

#### 函数签名

```rust
pub fn get_end_of_year(input_datetime: NaiveDateTime) -> NaiveDateTime
```

#### 参数

- `input_datetime`: `NaiveDateTime` - 输入日期时间

#### 返回值

- 返回一个包含输入日期时间所在年结束时间的 `NaiveDateTime`。

#### 例子

```rust
use date_util::{DateUtil, TimeUnit};

let input_datetime = NaiveDateTime::from_str("2023-05-15 12:34:56", "%Y-%m-%d %H:%M:%S").unwrap();
let end_of_year = DateUtil::get_end_of_year(input_datetime);

println!("End of Year: {}", end_of_year);
```

### `get_end_of_month` 函数

#### 作用

获取输入日期时间所在月的结束时间。

#### 函数签名

```rust
pub fn get_end_of_month(input_datetime: NaiveDateTime) -> NaiveDateTime
```

#### 参数

- `input_datetime`: `NaiveDateTime` - 输入日期时间

#### 返回值

- 返回一个包含输入日期时间所在月结束时间的 `NaiveDateTime`。

#### 例子

```rust
use date_util::{DateUtil, TimeUnit};

let input_datetime = NaiveDateTime::from_str("2023-05-15 12:34:56", "%Y-%m-%d %H:%M:%S").unwrap();
let end_of_month = DateUtil::get_end_of_month(input_datetime);

println!("End of Month: {}", end_of_month);
```

### `get_end_of_week` 函数

#### 作用

获取输入日期时间所在周的结束时间。

#### 函数签名

```rust
pub fn get_end_of_week(input_datetime: NaiveDateTime) -> NaiveDateTime
```

#### 参数

- `input_datetime`: `NaiveDateTime` - 输入日期时间

#### 返回值

- 返回一个包含输入日期时间所在周结束时间的 `NaiveDateTime`。

#### 例子

```rust
use date_util::{DateUtil, TimeUnit};

let input_datetime = NaiveDateTime::from_str("2023-05-15 12:34:56", "%Y-%m-%d %H:%M:%S").unwrap();
let end_of_week = DateUtil::get_end_of_week(input_datetime);

println!("End of Week: {}", end_of_week);
```

## 获取特定时间范围

### `get_start_of_year` 函数

#### 作用

获取输入日期时间所在年的开始时间。

#### 函数签名

```rust
pub fn get_start_of_year(input_datetime: NaiveDateTime) -> NaiveDateTime
```

#### 参数

- `input_datetime`: `NaiveDateTime` - 输入日期时间

#### 返回值

- 返回一个包含输入日期时间所在年开始时间的 `NaiveDateTime`。

#### 例子

```rust
use date_util::{DateUtil, TimeUnit};

let input_datetime = NaiveDateTime::from_str("2023-05-15 12:34:56", "%Y-%m-%d %H:%M:%S").unwrap();
let start_of_year = DateUtil::get_start_of_year(input_datetime);

println!("Start of Year: {}", start_of_year);
```

### `get_start_of_month` 函数

#### 作用

获取输入日期时间所在月的开始时间。

#### 函数签名

```rust
pub fn get_start_of_month(input_datetime: NaiveDateTime) -> NaiveDateTime
```

#### 参数

- `input_datetime`: `NaiveDateTime` - 输入日期时间

#### 返回值

- 返回一个包含输入日期时间所在月开始时间的 `NaiveDateTime`。

#### 例子

```rust
use date_util::{DateUtil, TimeUnit};

let input_datetime = NaiveDateTime::from_str("2023-05-15 12:34:56", "%Y-%m-%d %H:%M:%S").unwrap();
let start_of_month = DateUtil::get_start_of_month(input_datetime);

println!("Start of Month: {}", start_of_month);
```

### `get_start_of_week` 函数

#### 作用

获取输入日期时间所在周的开始时间。

#### 函数签名

```rust
pub fn get_start_of_week(input_datetime: NaiveDateTime) -> NaiveDateTime
```

#### 参数

- `input_datetime`: `NaiveDateTime` - 输入日期时间

#### 返回值

- 返回一个包含输入日期时间所在周开始时间的 `NaiveDateTime`。

#### 例子

```rust
use date_util::{DateUtil, TimeUnit};

let input_datetime = NaiveDateTime::from_str("2023-05-15 12:34:56", "%Y-%m-%d %H:%M:%S").unwrap();
let start_of_week = DateUtil::get_start_of_week(input_datetime);

println!("Start of Week: {}", start_of_week);
```

### `get_start_of_day_for_day` 函数

#### 作用

获取输入日期的起始时间。

#### 函数签名

```rust
pub fn get_start_of_day_for_day(input_date: NaiveDate) -> NaiveDateTime
```

#### 参数

- `input_date`: `NaiveDate` - 输入日期（不包含时间部分）

#### 返回值

- 返回一个包含输入日期起始时间的 `NaiveDateTime`。

#### 例子

```rust
use date_util::{DateUtil, TimeUnit};

let input_date = NaiveDate::from_str("2023-05-15", "%Y-%m-%d").unwrap();
let start_of_day = DateUtil::get_start_of_day_for_day(input_date);

println!("Start of Day: {}", start_of_day);
```

### `get_end_of_day_for_day` 函数

#### 作用

获取输入日期的结束时间。

#### 函数签名

```rust
pub fn get_end_of_day_for_day(input_date: NaiveDate) -> NaiveDateTime
```

#### 参数

- `input_date`: `NaiveDate` - 输入日期（不包含时间部分）

#### 返回值

- 返回一个包含输入日期结束时间的 `NaiveDateTime`。

#### 例子

```rust
use date_util::{DateUtil, TimeUnit};

let input_date = NaiveDate::from_str("2023-05-15", "%Y-%m-%d").unwrap();
let end_of_day = DateUtil::get_end_of_day_for_day(input_date);

println!("End of Day: {}", end_of_day);
```

### `get_start_of_day` 函数

#### 作用

获取输入日期时间所

在天的开始时间。

#### 函数签名

```rust
pub fn get_start_of_day(input_datetime: NaiveDateTime) -> NaiveDateTime
```

#### 参数

- `input_datetime`: `NaiveDateTime` - 输入日期时间

#### 返回值

- 返回一个包含输入日期时间所在天开始时间的 `NaiveDateTime`。

#### 例子

```rust
use date_util::{DateUtil, TimeUnit};

let input_datetime = NaiveDateTime::from_str("2023-05-15 12:34:56", "%Y-%m-%d %H:%M:%S").unwrap();
let start_of_day = DateUtil::get_start_of_day(input_datetime);

println!("Start of Day: {}", start_of_day);
```

### `get_end_of_day` 函数

#### 作用

获取输入日期时间所在天的结束时间。

#### 函数签名

```rust
pub fn get_end_of_day(input_datetime: NaiveDateTime) -> NaiveDateTime
```

#### 参数

- `input_datetime`: `NaiveDateTime` - 输入日期时间

#### 返回值

- 返回一个包含输入日期时间所在天结束时间的 `NaiveDateTime`。

#### 例子

```rust
use date_util::{DateUtil, TimeUnit};

let input_datetime = NaiveDateTime::from_str("2023-05-15 12:34:56", "%Y-%m-%d %H:%M:%S").unwrap();
let end_of_day = DateUtil::get_end_of_day(input_datetime);

println!("End of Day: {}", end_of_day);
```

## 时间加减法和时间字符串解析

### `manipulate_time` 函数

#### 作用

对输入时间进行加减法操作。

#### 函数签名

```rust
pub fn manipulate_time(input_time: NaiveDateTime, step: i64, unit: TimeUnit) -> NaiveDateTime
```

#### 参数

- `input_time`: `NaiveDateTime` - 输入时间
- `step`: `i64` - 步长，可以是正数或负数
- `unit`: `TimeUnit` - 时间单位，使用 `TimeUnit` 枚举表示

#### 返回值

- 返回计算后的时间，以 `NaiveDateTime` 格式表示。

#### 例子

```rust
use date_util::{DateUtil, TimeUnit};

let input_time = NaiveDateTime::from_str("2023-05-15 12:34:56", "%Y-%m-%d %H:%M:%S").unwrap();
let manipulated_time = DateUtil::manipulate_time(input_time, -2, TimeUnit::Days);

println!("Manipulated Time: {}", manipulated_time);
```

### `parse_time` 函数

#### 作用

解析输入时间字符串。

#### 函数签名

```rust
pub fn parse_time(input: String, format_str: DateTimeFormat) -> Option<NaiveDateTime>
```

#### 参数

- `input`: `String` - 待解析的时间字符串
- `format_str`: `DateTimeFormat` - 时间格式

#### 返回值

- 返回解析后的时间，以 `NaiveDateTime` 格式表示，如果解析失败则返回 `Option::None`。

#### 例子

```rust
use date_util::{DateUtil, DateTimeFormat};

let input_time_str = "2023-05-15 12:34:56".to_string();
let parsed_time = DateUtil::parse_time(input_time_str, DateTimeFormat::Custom("%Y-%m-%d %H:%M:%S".to_string()));

match parsed_time {
    Some(time) => println!("Parsed Time: {}", time),
    None => println!("Failed to parse time."),
}
```

### `get_now_time_string` 函数

#### 作用

获取当前时间的字符串表示。

#### 函数签名

```rust
pub fn get_now_time_string(format_str: DateTimeFormat) -> String
```

#### 参数

- `format_str`: `DateTimeFormat` - 目标格式

#### 返回值

- 返回当前时间的字符串表示。

#### 例子

```rust
use date_util::{DateUtil, DateTimeFormat};

let now_time_str = DateUtil::get_now_time_string(DateTimeFormat::Custom("%Y-%m-%d %H:%M:%S".to_string()));

println!("Current Time: {}", now_time_str);
```

### `now` 函数

#### 作用

获取当前时间。

#### 函数签名

```rust
pub fn now() -> NaiveDateTime
```

#### 返回值

- 返回当前时间，以 `NaiveDateTime` 格式表示。

#### 例子

```rust
use date_util::DateUtil;

let current_time = DateUtil::now();

println!("Current Time: {}", current_time);
```

### `get_now_time_utc_string` 函数

#### 作用

获取当前 UTC 时间的字符串表示。

#### 函数签名

```rust
pub fn get_now_time_utc_string(format_str: DateTimeFormat) -> String
```

#### 参数

- `format_str`: `DateTimeFormat` - 目标格式

#### 返回值

- 返回当前 UTC 时间的字符串表示。

#### 例子

```rust
use date_util::{DateUtil, DateTimeFormat};

let now_utc_time_str = DateUtil::get_now_time_utc_string(DateTimeFormat::Custom("%Y-%m-%d %H:%M:%S".to_string()));

println!("Current UTC Time: {}", now_utc_time_str);
```