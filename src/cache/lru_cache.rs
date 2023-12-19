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
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            cache: HashMap::with_capacity(capacity),
            order: LinkedList::new(),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some((value, timestamp)) = self.cache.get_mut(key) {
            *timestamp = self.order.len();
            let popped_key = self.order.pop_front().unwrap();
            self.order.push_back(popped_key);
            Some(value)
        } else {
            None
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.cache.len() >= self.capacity {
            if let Some(oldest_key) = self.order.pop_front() {
                self.cache.remove(&oldest_key);
            }
        }

        let timestamp = self.order.len();
        self.cache.insert(key.clone(), (value, timestamp));
        self.order.push_back(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache() {
        let mut lru_cache = LRUCache::new(3);

        lru_cache.put("one", 1);
        lru_cache.put("two", 2);
        lru_cache.put("three", 3);

        assert_eq!(lru_cache.get(&"one"), Some(&1));

        lru_cache.put("four", 4);

        assert_eq!(lru_cache.cache.len(), 3);
        assert_eq!(lru_cache.get(&"one"), Some(&1));
        assert_eq!(lru_cache.get(&"two"), None);
        assert_eq!(lru_cache.get(&"three"), Some(&3));
        assert_eq!(lru_cache.get(&"four"), Some(&4));
    }
}
