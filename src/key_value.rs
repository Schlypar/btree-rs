use std::{cmp::Ordering, fmt::Display};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct KeyValue<K, T>(pub K, pub T)
where
    T: Clone + Copy,
    K: Clone + Copy + Ord;

impl<K, T> PartialOrd for KeyValue<K, T>
where
    T: Clone + Copy,
    K: Clone + Copy + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self < other {
            Some(Ordering::Less)
        } else if self == other {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl<K, T> Display for KeyValue<K, T>
where
    T: Clone + Copy + Display,
    K: Clone + Copy + Ord + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", &self.0, &self.1)
    }
}

impl<K, T> PartialEq for KeyValue<K, T>
where
    T: Clone + Copy,
    K: Clone + Copy + Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<K, T> Eq for KeyValue<K, T>
where
    T: Clone + Copy,
    K: Clone + Copy + Ord,
{
}

impl<K, T> Ord for KeyValue<K, T>
where
    T: Clone + Copy,
    K: Clone + Copy + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        if self < other {
            Ordering::Less
        } else if self == other {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}
