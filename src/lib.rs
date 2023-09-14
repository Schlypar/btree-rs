pub mod comparator;
mod key_value;
pub mod sorted_vec;

pub use comparator::Comparator;
use key_value::KeyValue;
use sorted_vec::SortedVec;

use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::rc::Rc;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Node<K, T>
where
    T: Copy + Clone,
    K: Copy + Clone + Ord,
{
    keys: Rc<RefCell<SortedVec<KeyValue<K, T>>>>,
    children: Rc<RefCell<Vec<Self>>>,
    is_leaf: bool,
}

#[allow(dead_code)]
impl<K, T> Node<K, T>
where
    T: Copy + Clone,
    K: Copy + Clone + Ord + 'static,
{
    pub fn new(comparator: Comparator<K>) -> Node<K, T> {
        let comp_fn = move |a: &KeyValue<K, T>, b: &KeyValue<K, T>| -> Ordering {
            let result = comparator.compare(&a.0, &b.0);
            result
        };
        let comparator = Comparator::new(Rc::new(comp_fn));
        Node {
            keys: Rc::new(RefCell::new(SortedVec::new(comparator))),
            children: Rc::new(RefCell::new(Vec::new())),
            is_leaf: false,
        }
    }

    fn from(other: &Node<K, T>) -> Node<K, T> {
        Node {
            keys: other.keys.clone(),
            children: Rc::new(RefCell::new(other.children.borrow().clone())),
            is_leaf: other.is_leaf,
        }
    }

    fn with(is_leaf: bool, comparator: Comparator<K>) -> Node<K, T> {
        let comp_fn = move |a: &KeyValue<K, T>, b: &KeyValue<K, T>| -> Ordering {
            let result = comparator.compare(&a.0, &b.0);
            result
        };
        let comparator = Comparator::new(Rc::new(comp_fn));
        Node {
            keys: Rc::new(RefCell::new(SortedVec::new(comparator))),
            children: Rc::new(RefCell::new(Vec::new())),
            is_leaf,
        }
    }
}

impl<K, T> PartialEq for Node<K, T>
where
    T: Copy + Clone,
    K: Copy + Clone + Ord,
{
    fn eq(&self, other: &Self) -> bool {
        for (ours, theirs) in self.keys.borrow().iter().zip(other.keys.borrow().iter()) {
            if ours != theirs {
                return false;
            }
        }

        for (ours, theirs) in self
            .children
            .borrow()
            .iter()
            .zip(other.children.borrow().iter())
        {
            if ours != theirs {
                return false;
            }
        }

        if self.is_leaf != other.is_leaf {
            return false;
        }

        true
    }
}

pub struct AlreadyPresent;

impl Debug for AlreadyPresent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: AlreadyPresent")
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BTree<K, T>
where
    T: Copy + Clone,
    K: Copy + Clone + Ord,
{
    root: Node<K, T>,
    t: usize,
    comp: Comparator<K>,
}

#[allow(dead_code)]
impl<K, T> BTree<K, T>
where
    T: Copy + Clone + Debug,
    K: Copy + Clone + Ord + Debug + 'static,
{
    pub fn new(t: usize, comparator: Comparator<K>) -> Option<BTree<K, T>> {
        if t < 2 {
            return None;
        }

        let mut btree = BTree {
            root: Node::new(comparator.clone()),
            t,
            comp: comparator,
        };
        btree.root.is_leaf = true;

        Some(btree)
    }

    pub fn search(&self, key: &K) -> Option<T> {
        let node = &self.root;
        let mut index = 0;

        while index < node.keys.borrow().len() {
            match self.comp.compare(key, &node.keys.borrow().get(index)?.0) {
                Ordering::Greater => index += 1,
                Ordering::Equal => return Some(node.keys.borrow().get(index)?.1.clone()),
                Ordering::Less => {
                    if node.is_leaf {
                        return None;
                    } else {
                        return self
                            .search_helper(key, &node.children.borrow().get(index)?.clone());
                    }
                }
            }
        }

        None
    }

    fn search_helper(&self, key: &K, node: &Node<K, T>) -> Option<T> {
        let mut index = 0;

        while index < node.keys.borrow().len() {
            match self
                .comp
                .compare(key, &node.keys.borrow().get(index).unwrap().0)
            {
                Ordering::Greater => index += 1,
                Ordering::Equal => return Some(node.keys.borrow().get(index)?.1.clone()),
                Ordering::Less => {
                    if node.is_leaf {
                        return None;
                    } else {
                        return self
                            .search_helper(key, &node.children.borrow().get(index)?.clone());
                    }
                }
            }
        }

        None
    }

    fn search_node(&self, key: &K) -> Option<(Node<K, T>, usize)> {
        let node = &self.root;
        let mut index = 0;

        while index < node.keys.borrow().len() {
            match self.comp.compare(key, &node.keys.borrow().get(index)?.0) {
                Ordering::Greater => index += 1,
                Ordering::Equal => return Some((node.clone(), index)),
                Ordering::Less => {
                    if node.is_leaf {
                        return None;
                    } else {
                        return self
                            .search_node_helper(key, node.children.borrow().get(index)?.clone());
                    }
                }
            }
        }

        None
    }

    fn search_node_helper(&self, key: &K, node: Node<K, T>) -> Option<(Node<K, T>, usize)> {
        let mut index = 0;

        while index < node.keys.borrow().len() {
            match self.comp.compare(key, &node.keys.borrow().get(index)?.0) {
                Ordering::Greater => index += 1,
                Ordering::Equal => return Some((node.clone(), index)),
                Ordering::Less => {
                    if node.is_leaf {
                        return None;
                    } else {
                        return self
                            .search_node_helper(key, node.children.borrow().get(index)?.clone());
                    }
                }
            }
        }

        None
    }

    fn split_child(&mut self, parent: Node<K, T>, index: usize) {
        let parent_keys = parent.keys.as_ptr();
        let mut parent_children = parent.children.borrow_mut();
        let full_child = parent_children.get_mut(index).unwrap().clone();

        unsafe {
            (*parent_children).insert(index + 1, Node::with(full_child.is_leaf, self.comp.clone()));

            let median = full_child.keys.borrow().get(self.t - 1).unwrap().clone();
            (*parent_keys).insert(median);
        };
        {
            let full_child = parent_children.get_mut(index).unwrap().clone();
            let full_child_sibling = parent_children.get_mut(index + 1).unwrap().clone();

            let mut full_child_sibling_keys = full_child_sibling.keys.borrow_mut();
            let mut full_child_keys = full_child.keys.borrow_mut();

            let sliced_keys: Vec<KeyValue<K, T>> = full_child_keys.iter().skip(self.t).collect();
            let mut sorted_keys = SortedVec::with_comp_from(&(*full_child_keys));
            sliced_keys.iter().for_each(|e| sorted_keys.insert(*e));
            (*full_child_sibling_keys) = sorted_keys;

            let sliced_keys: Vec<KeyValue<K, T>> =
                full_child_keys.iter().take(self.t - 1).collect();
            let mut sorted_keys = SortedVec::with_comp_from(&(*full_child_keys));
            sliced_keys.iter().for_each(|e| sorted_keys.insert(*e));
            (*full_child_keys) = sorted_keys;
        }
        if !full_child.is_leaf {
            let full_child_sibling = parent_children.get_mut(index + 1).unwrap().clone();

            let mut full_child_children = full_child.children.borrow_mut();
            let mut full_child_sibling_children = full_child_sibling.children.borrow_mut();

            (*full_child_sibling_children) =
                full_child_children.iter().cloned().skip(self.t).collect();

            (*full_child_children) = full_child_children
                .iter()
                .cloned()
                .take(self.t - 1)
                .collect();
        }
    }

    pub fn insert(&mut self, record: (K, T)) -> Result<(), AlreadyPresent> {
        if let Some(_) = self.search(&record.0) {
            return Err(AlreadyPresent);
        } else {
            let root_keys = self.root.keys.as_ptr();

            if unsafe { (*root_keys).len() == 2 * self.t - 1 } {
                let new_root: Node<K, T> = Node::new(self.comp.clone());
                let ref mut old_root = self.root;

                new_root.children.borrow_mut().insert(0, old_root.clone());

                self.split_child(new_root.clone(), 0);
                self.insert_non_full(new_root.clone(), record);

                self.root = new_root;

                return Ok(());
            } else {
                self.insert_non_full(self.root.clone(), record);
                return Ok(());
            }
        }
    }

    fn insert_non_full(&mut self, node: Node<K, T>, record: (K, T)) {
        let mut node_keys = node.keys.borrow_mut();
        let mut index = node_keys.len() as i32 - 1;

        if node.is_leaf {
            (*node_keys).insert(KeyValue {
                0: record.0,
                1: record.1,
            });
            return;
        } else {
            while index >= 0 {
                if let Ordering::Less = self
                    .comp
                    .compare(&record.0, &node_keys.get(index as usize).unwrap().0)
                {
                    index -= 1;
                } else {
                    break;
                }
            }

            index += 1;

            let node_children = node.children.borrow();
            let node_children_keys_len = node_children
                .get(index as usize)
                .unwrap()
                .keys
                .borrow()
                .len();

            if node_children_keys_len == 2 * self.t - 1 {
                std::mem::drop(node_children);
                self.split_child(node.clone(), index as usize);

                if let Ordering::Greater = self
                    .comp
                    .compare(&record.0, &node_keys.get(index as usize).unwrap().0)
                {
                    index += 1;
                }
            }
            let node_children = node.children.borrow();

            self.insert_non_full(node_children.get(index as usize).unwrap().clone(), record);
        }
    }
}

impl<K, T> Node<K, T>
where
    T: Copy + Clone + Display,
    K: Copy + Clone + Ord + Display,
{
    fn to_string(&self, level: usize) -> String {
        let mut str = format!("LEVEL {}: {}\n", level, self.keys.borrow());

        self.children.borrow().iter().for_each(|child| {
            str.push_str(&child.to_string(level + 1)[..]);
        });

        format!("{}", str)
    }
}

impl<K, T> Display for Node<K, T>
where
    T: Copy + Clone + Display,
    K: Copy + Clone + Ord + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string(0))
    }
}

impl<K, T> Display for BTree<K, T>
where
    T: Copy + Clone + Display,
    K: Copy + Clone + Ord + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root.to_string(0))
    }
}
