use std::{fmt::Display, ops::Index};

use super::comparator::Comparator;
use std::cmp::Ord;

pub struct NotFound;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SortedVec<T>
where
    T: Ord + Clone,
{
    data: Vec<T>,
    comparator: Comparator<T>,
}

impl<T> SortedVec<T>
where
    T: Ord + Clone,
{
    pub fn new(comparator: Comparator<T>) -> SortedVec<T> {
        SortedVec {
            data: Vec::new(),
            comparator,
        }
    }

    pub fn with_comp_from(sorted_vec: &SortedVec<T>) -> SortedVec<T> {
        SortedVec {
            data: vec![],
            comparator: sorted_vec.comparator.clone(),
        }
    }

    pub fn from_vec(mut vec: Vec<T>, comparator: Comparator<T>) -> SortedVec<T> {
        vec.sort_unstable_by(comparator.compare_fn.as_ref());
        SortedVec {
            data: vec,
            comparator,
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
    }

    pub fn insert(&mut self, value: T) {
        let index = self
            .data
            .binary_search_by(|elem| self.comparator.compare(elem, &value))
            .unwrap_or_else(|x| x);
        self.data.insert(index, value);
    }

    pub fn remove(&mut self, value: &T) -> Result<(), NotFound> {
        if let Ok(index) = self.data.binary_search(value) {
            self.data.remove(index);
            Ok(())
        } else {
            Err(NotFound)
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.data.binary_search(value).is_ok()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            inner: self.data.iter(),
        }
    }
}

impl<T> SortedVec<T>
where
    T: Ord + Clone + Display,
{
    pub fn to_string(&self) -> String {
        let tokens: Vec<String> = self.data.iter().map(|e| e.to_string()).collect();

        let result = tokens
            .iter()
            .skip(1)
            .fold("".to_string(), |acc: String, str: &String| {
                format!("{}, {}", &acc, str)
            });

        format!("[{}{}]", tokens[0], result)
    }
}

impl<T> Display for SortedVec<T>
where
    T: Ord + Clone + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = self.to_string();
        write!(f, "{}", str)
    }
}

impl<T> Index<usize> for SortedVec<T>
where
    T: Ord + Clone,
{
    type Output = T;

    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        &self.data[index]
    }
}

pub struct Iter<'a, T>
where
    T: Clone,
{
    inner: std::slice::Iter<'a, T>,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // self.inner.next()
        let res = self.inner.next()?;

        Some(res.clone())
    }
}
