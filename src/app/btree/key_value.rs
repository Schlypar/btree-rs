use std::cmp::Ordering;
use std::fmt::{Debug, Display};

pub trait Comparator<K> {
    fn compare(lhs: &K, rhs: &K) -> Ordering;
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct KeyValue<K, T>
where
    K: Ord,
{
    pub key: K,
    pub value: T,
}

impl<K, T> PartialOrd for KeyValue<K, T>
where
    K: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.key.cmp(&other.key))
    }
}

impl<K, T> PartialEq for KeyValue<K, T>
where
    K: Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K, T> Eq for KeyValue<K, T>
where
    K: Ord,
{
}

impl<K, T> Ord for KeyValue<K, T>
where
    K: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl<K, T> From<(K, T)> for KeyValue<K, T>
where
    K: Ord,
{
    fn from(value: (K, T)) -> Self {
        let (key, value) = value;
        KeyValue { key, value }
    }
}

impl<K, T> Display for KeyValue<K, T>
where
    T: Display,
    K: Ord + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.key, self.value)
    }
}
