use std::collections::{HashMap, LinkedList};

pub struct LFUCache<K:Eq, V> {
    capacity: usize,
    cache: HashMap<K, (V, usize, usize)>,
    order: LinkedList<K>,
}

impl<K :Eq, V> LFUCache<K, V>
    where
        K: Eq + std::hash::Hash + Clone,
{
    /// 创建一个新的LFU缓存实例。
    ///
    /// # 参数
    ///
    /// * `capacity`: usize - 缓存的容量
    ///
    /// 返回值：LFUCache<K, V> - 新创建的LFU缓存实例
    pub fn new(capacity: usize) -> Self {
        LFUCache {
            capacity,
            cache: HashMap::with_capacity(capacity),
            order: LinkedList::new(),
        }
    }

    /// 获取缓存中指定键的值，并将该键的访问频率增加，并将该键标记为最近使用。
    ///
    /// # 参数
    ///
    /// * `key`: &K - 要获取的键的引用
    ///
    /// 返回值：Option<&V> - 如果存在则返回对应值的引用，否则返回None
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some((value, frequency, timestamp)) = self.cache.get_mut(key) {
            // 更新访问频率
            *frequency += 1;
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

    /// 将键值对插入缓存中，如果缓存已满则淘汰访问频率最低的项。
    ///
    /// # 参数
    ///
    /// * `key`: K - 要插入的键
    /// * `value`: V - 要插入的值
    pub fn put(&mut self, key: K, value: V) {
        if self.cache.len() >= self.capacity {
            // 找到访问频率最低的项
            let min_frequency_key = self
                .order
                .iter()
                .min_by_key(|&k| self.cache.get(k).map_or(usize::MAX, |(_, f, _)| *f))
                .cloned();

            // 淘汰访问频率最低的项
            if let Some(oldest_key) = min_frequency_key {
                self.cache.remove(&oldest_key);
            }
        }

        // 插入新项
        let timestamp = self.order.len();
        self.cache.insert(key.clone(), (value, 1, timestamp));
        // 将键移动到顺序列表的末尾（最近使用）
        self.order.push_back(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lfu_cache() {
        let mut lfu_cache = LFUCache::new(3);

        // 插入项
        lfu_cache.put("one", 1);
        lfu_cache.put("two", 2);
        lfu_cache.put("three", 3);

        // 访问一项以使其成为最近使用的项
        assert_eq!(lfu_cache.get(&"one"), Some(&1));

        // 访问"two"两次，以增加其访问频率
        assert_eq!(lfu_cache.get(&"two"), Some(&2));
        assert_eq!(lfu_cache.get(&"two"), Some(&2));

        // 插入新项，淘汰访问频率最低的项（"three"）
        lfu_cache.put("four", 4);

        // 检查缓存的状态
        assert_eq!(lfu_cache.cache.len(), 3);
        assert_eq!(lfu_cache.get(&"one"), Some(&1));
        assert_eq!(lfu_cache.get(&"two"), Some(&2));
        assert_eq!(lfu_cache.get(&"three"), None); // "three" 应该被淘汰
        assert_eq!(lfu_cache.get(&"four"), Some(&4));
    }
}
