use std::collections::{HashMap, LinkedList};

pub struct LRUCache<K, V> {
    capacity: usize,
    cache: HashMap<K, (V, usize)>,
    order: LinkedList<K>,
}

impl<K, V> LRUCache<K, V>
    where
        K: Eq + std::hash::Hash + Clone,
{
    /// 创建一个新的LRU缓存实例。
    ///
    /// # 参数
    ///
    /// * `capacity`: usize - 缓存的容量
    ///
    /// 返回值：LRUCache<K, V> - 新创建的LRU缓存实例
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            cache: HashMap::with_capacity(capacity),
            order: LinkedList::new(),
        }
    }

    /// 获取缓存中指定键的值，并将该键标记为最近使用。
    ///
    /// # 参数
    ///
    /// * `key`: &K - 要获取的键的引用
    ///
    /// 返回值：Option<&V> - 如果存在则返回对应值的引用，否则返回None
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some((value, timestamp)) = self.cache.get_mut(key) {
            // 更新时间戳
            *timestamp = self.order.len();
            // 将键移动到顺序列表的末尾（最近使用）
            let popped_key = self.order.pop_front().unwrap();
            self.order.push_back(popped_key);
            Some(value)
        } else {
            None
        }
    }

    /// 将键值对插入缓存中，如果缓存已满则淘汰最久未使用的项。
    ///
    /// # 参数
    ///
    /// * `key`: K - 要插入的键
    /// * `value`: V - 要插入的值
    pub fn put(&mut self, key: K, value: V) {
        if self.cache.len() >= self.capacity {
            // 淘汰最久未使用的项
            if let Some(oldest_key) = self.order.pop_front() {
                self.cache.remove(&oldest_key);
            }
        }

        // 插入新项
        let timestamp = self.order.len();
        self.cache.insert(key.clone(), (value, timestamp));
        // 将键移动到顺序列表的末尾（最近使用）
        self.order.push_back(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache() {
        let mut lru_cache = LRUCache::new(3);

        // 插入项
        lru_cache.put("one", 1);
        lru_cache.put("two", 2);
        lru_cache.put("three", 3);

        // 访问一项以使其成为最近使用的项
        assert_eq!(lru_cache.get(&"one"), Some(&1));

        // 插入新项，淘汰最久未使用的项（"two"）
        lru_cache.put("four", 4);

        // 检查缓存的状态
        assert_eq!(lru_cache.cache.len(), 3);
        assert_eq!(lru_cache.get(&"one"), Some(&1));
        assert_eq!(lru_cache.get(&"two"), None); // "two" 应该被淘汰
        assert_eq!(lru_cache.get(&"three"), Some(&3));
        assert_eq!(lru_cache.get(&"four"), Some(&4));
    }
}
