# LFUCache 模块

`LFUCache` 模块实现了一个基于LFU（Least Frequently Used）策略的缓存结构，用于存储键值对。LFU缓存在容量达到限制时，会淘汰访问频率最低的项。

## 使用方法

### `new` 函数

```rust
pub fn new(capacity: usize) -> LFUCache<K, V>
```

#### 参数

- `capacity`: `usize` - 缓存的容量

#### 返回值

- 返回一个新创建的 `LFUCache<K, V>` 实例。

#### 例子

```rust
use lfu::LFUCache;

let mut lfu_cache = LFUCache::new(3);
```

### `get` 函数

```rust
pub fn get(&mut self, key: &K) -> Option<&V>
```

#### 参数

- `key`: `&K` - 要获取的键的引用

#### 返回值

- 返回一个 `Option<&V>`，如果存在则返回对应值的引用，否则返回 `None`。

#### 例子

```rust
use lfu::LFUCache;

let value = lfu_cache.get(&"one");
```

### `put` 函数

```rust
pub fn put(&mut self, key: K, value: V)
```

#### 参数

- `key`: `K` - 要插入的键
- `value`: `V` - 要插入的值

#### 例子

```rust
use lfu::LFUCache;

lfu_cache.put("one", 1);
lfu_cache.put("two", 2);
```


## 示例

```rust
use lfu::LFUCache;

fn main() {
    let mut lfu_cache = LFUCache::new(3);

    // 插入项
    lfu_cache.put("one", 1);
    lfu_cache.put("two", 2);
    lfu_cache.put("three", 3);

    // 访问一项以使其成为最近使用的项
    let value = lfu_cache.get(&"one");
    println!("Value: {:?}", value);

    // 访问"two"两次，以增加其访问频率
    let value = lfu_cache.get(&"two");
    println!("Value: {:?}", value);
    let value = lfu_cache.get(&"two");
    println!("Value: {:?}", value);

    // 插入新项，淘汰访问频率最低的项（"three"）
    lfu_cache.put("four", 4);

    // 检查缓存的状态
    println!("Cache: {:?}", lfu_cache.cache);
}
```
