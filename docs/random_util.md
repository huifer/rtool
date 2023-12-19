# RandomUtil 模块

`RandomUtil` 模块提供了生成随机数和字符串的工具方法，包括随机整数、随机浮点数、随机布尔值、随机字母和随机字符串。

## 使用方法

### `random_int` 函数

```rust
pub fn random_int(min: i32, max: i32) -> i32
```

#### 参数

- `min`: `i32` - 整数范围的下限值（包括该值）。
- `max`: `i32` - 整数范围的上限值（包括该值）。

#### 返回值

- 返回生成的随机整数。

#### 例子

```rust
use random_util::RandomUtil;

let random_int = RandomUtil::random_int(1, 100);
```

### `random_float` 函数

```rust
pub fn random_float(min: f64, max: f64) -> f64
```

#### 参数

- `min`: `f64` - 浮点数范围的下限值（包括该值）。
- `max`: `f64` - 浮点数范围的上限值（包括该值）。

#### 返回值

- 返回生成的随机浮点数。

#### 例子

```rust
use random_util::RandomUtil;

let random_float = RandomUtil::random_float(1.0, 100.0);
```

### `random_bool` 函数

```rust
pub fn random_bool() -> bool
```

#### 返回值

- 返回生成的随机布尔值。

#### 例子

```rust
use random_util::RandomUtil;

let random_bool = RandomUtil::random_bool();
```

### `random_char` 函数

```rust
pub fn random_char() -> char
```

#### 返回值

- 返回生成的随机小写字母。

#### 例子

```rust
use random_util::RandomUtil;

let random_char = RandomUtil::random_char();
```

### `random_string` 函数

```rust
pub fn random_string(length: usize) -> String
```

#### 参数

- `length`: `usize` - 字符串的长度。

#### 返回值

- 返回生成的随机字符串。

#### 例子

```rust
use random_util::RandomUtil;

let random_string = RandomUtil::random_string(10);
```


## 示例

```rust
use random_util::RandomUtil;

fn main() {
    let random_int = RandomUtil::random_int(1, 100);
    let random_float = RandomUtil::random_float(1.0, 100.0);
    let random_bool = RandomUtil::random_bool();
    let random_char = RandomUtil::random_char();
    let random_string = RandomUtil::random_string(10);

    println!("Random Int: {}", random_int);
    println!("Random Float: {}", random_float);
    println!("Random Bool: {}", random_bool);
    println!("Random Char: {}", random_char);
    println!("Random String: {}", random_string);
}
```
