# Desensitized 模块

`Desensitized` 模块提供了对字符串进行脱敏处理的功能，通过保留起始索引和末尾索引的字符，将其他字符用 '*' 代替。

## 使用方法

### `desensitize` 函数

```rust
pub fn desensitize(input: &str, start_index: usize, end_index: usize) -> String
```

#### 参数

- `input`: `&str` - 待脱敏的字符串。
- `start_index`: `usize` - 脱敏保留的起始索引，从0开始。
- `end_index`: `usize` - 脱敏保留的结束索引，从字符串末尾往前数。

#### 返回值

- 返回经过脱敏处理后的字符串，其中保留了起始索引和末尾索引的字符，其他字符用 '*' 代替。

#### 例子

```rust
use desensitized::Desensitized;

let result1 = Desensitized::desensitize("hello world", 2, 3);
assert_eq!(result1, "he******rld");

let result2 = Desensitized::desensitize("example123", 0, 4);
assert_eq!(result2, "******e123");

let result3 = Desensitized::desensitize("confidential", 5, 5);
assert_eq!(result3, "confi**ntial");
```


## 示例

```rust
use desensitized::Desensitized;

fn main() {
    let input_str = "sensitive_data";

    // 对字符串进行脱敏处理
    let desensitized_result = Desensitized::desensitize(input_str, 4, 6);
    println!("Desensitized Result: {}", desensitized_result);
}
```
