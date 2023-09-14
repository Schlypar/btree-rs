use core::fmt;
use std::{cmp::Ordering, rc::Rc};

#[allow(dead_code)]
pub trait Compare {
    fn compare(&self) -> Ordering;
}

impl Compare for () {
    fn compare(&self) -> Ordering {
        Ordering::Equal
    }
}

impl<T> Compare for (T, T)
where
    T: Ord,
{
    fn compare(&self) -> Ordering {
        let (ref t1, ref t2) = *self;
        t1.cmp(t2)
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Comparator<T>
where
    T: Ord + Clone,
{
    pub compare_fn: Rc<dyn Fn(&T, &T) -> std::cmp::Ordering>,
}

impl<T> Comparator<T>
where
    T: Clone + Ord, // Ensure T implements Clone
{
    pub fn new(compare_fn: Rc<dyn Fn(&T, &T) -> Ordering>) -> Self {
        Comparator { compare_fn }
    }

    pub fn compare(&self, a: &T, b: &T) -> Ordering {
        (self.compare_fn)(a, b)
    }
}

impl<T> fmt::Debug for Comparator<T>
where
    T: Ord + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Comparator")
    }
}

#[allow(dead_code)]
impl<T, Rest> Compare for (T, T, Rest)
where
    T: Ord,
    Rest: Compare,
{
    fn compare(&self) -> Ordering {
        let (ref t1, ref t2, ref rest) = *self;
        let cmp = t1.cmp(t2);
        if cmp == Ordering::Equal {
            rest.compare()
        } else {
            cmp
        }
    }
}
