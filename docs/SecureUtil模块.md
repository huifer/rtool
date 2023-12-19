# SecureUtil 模块

`SecureUtil` 模块提供了一些常用的安全工具，包括使用 MD5 和 SHA-1 哈希算法计算字符串的哈希值。

## `md5` 函数

### 作用

使用 MD5 哈希算法计算输入字符串的哈希值。

### 函数签名

```rust
pub fn md5(input: &str) -> String
```

### 参数

- `input`: `&str` - 待计算哈希值的字符串

### 返回值

- 返回一个包含输入字符串 MD5 哈希值的 `String`。

### 例子

```rust
use secure_util::SecureUtil;

let input_str = "Hello, Rust!";
let md5_hash = SecureUtil::md5(input_str);

println!("MD5 Hash: {}", md5_hash);
```

## `sha1` 函数

### 作用

使用 SHA-1 哈希算法计算输入字符串的哈希值。

### 函数签名

```rust
pub fn sha1(input: &str) -> String
```

### 参数

- `input`: `&str` - 待计算哈希值的字符串

### 返回值

- 返回一个包含输入字符串 SHA-1 哈希值的 `String`。

### 例子

```rust
use secure_util::SecureUtil;

let input_str = "Hello, Rust!";
let sha1_hash = SecureUtil::sha1(input_str);

println!("SHA-1 Hash: {}", sha1_hash);
```


## 示例

```rust
use secure_util::SecureUtil;

fn main() {
    let input_str = "Hello, Rust!";

    // 计算 MD5 哈希值
    let md5_hash = SecureUtil::md5(input_str);
    println!("MD5 Hash: {}", md5_hash);

    // 计算 SHA-1 哈希值
    let sha1_hash = SecureUtil::sha1(input_str);
    println!("SHA-1 Hash: {}", sha1_hash);
}
```
