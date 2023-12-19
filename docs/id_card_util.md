# IdCardUtil 模块

`IdCardUtil` 模块提供了从身份证号码中提取信息的功能。

## 使用方法

### `extract_id_card_info` 函数

```rust
pub fn extract_id_card_info(id_card: &str) -> Option<IdCardInfo>
```

#### 参数

- `id_card`: `&str` - 身份证号码字符串

#### 返回值

- 如果成功提取信息，则返回 `Some(IdCardInfo)`，否则返回 `None`。

#### 例子

```rust
use idcardutil::IdCardUtil;

let id_card = "150526200001010797";
let result = IdCardUtil::extract_id_card_info(id_card);
```


## 示例

```rust
use idcardutil::{IdCardUtil, IdCardInfo};

fn main() {
    // 准备测试数据
    let id_card = "150526200001010797";

    // 调用提取信息的函数
    let result = IdCardUtil::extract_id_card_info(id_card);

    // 输出提取的信息
    match result {
        Some(id_card_info) => {
            println!("Birthday: {:?}", id_card_info.birthday);
            println!("Age: {:?}", id_card_info.age);
            println!("Gender: {:?}", id_card_info.gender);
        }
        None => {
            println!("Failed to extract information from the ID card.");
        }
    }
}
```
