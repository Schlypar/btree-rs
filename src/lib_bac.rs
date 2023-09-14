pub mod comparator;
mod key_value;
pub mod sorted_vec;

pub use comparator::Comparator;

use key_value::KeyValue;
use sorted_vec::{NotFound, SortedVec};

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
            is_leaf: true,
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

        Some(BTree {
            root: Node::new(comparator.clone()),
            t,
            comp: comparator,
        })
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

    fn split_child(&mut self, node: Node<K, T>, index: usize) {
        // let mut binding = node.children.borrow_mut();
        // let full_child = binding.get_mut(index).unwrap();
        //
        // // let mut binding = node.children.as_ref().borrow_mut();
        // binding.insert(
        //     index + 1,
        //     Node::with(full_child.is_leaf.clone(), self.comp.clone()),
        // );

        let mut binding = node.children.borrow_mut();
        if let Some(full_child) = binding.get_mut(index) {
            let new_node = Node::with(full_child.is_leaf.clone(), self.comp.clone());
            binding.insert(index + 1, new_node);
        }
        std::mem::drop(binding);

        let binding = node.children.borrow();
        let full_child = binding.get(index).unwrap();

        let mut binding = node.keys.as_ref().borrow_mut();
        binding.insert(*full_child.keys.as_ref().borrow().get(self.t - 1).unwrap());

        let vec: Vec<KeyValue<K, T>> = full_child
            .keys
            .as_ref()
            .borrow()
            .iter()
            .skip(self.t)
            .collect();

        let mut sorted_vec = SortedVec::with_comp_from(&self.root.keys.as_ref().borrow());
        vec.iter().for_each(|e| sorted_vec.insert(*e));

        let binding = node.children.as_ref().borrow();
        let binding = binding.get(index + 1).unwrap();
        *binding.keys.as_ref().borrow_mut() = sorted_vec;

        let vec: Vec<KeyValue<K, T>> = full_child
            .keys
            .as_ref()
            .borrow()
            .iter()
            .take(self.t - 1)
            .collect();

        let mut sorted_vec = SortedVec::with_comp_from(&self.root.keys.as_ref().borrow());
        vec.iter().for_each(|e| sorted_vec.insert(*e));

        let binding = node.children.as_ref().borrow();
        let binding = binding.get(index).unwrap();
        *binding.keys.as_ref().borrow_mut() = sorted_vec;

        if !full_child.is_leaf {
            let mut children: Vec<Node<K, T>> = Vec::new();

            full_child
                .children
                .as_ref()
                .borrow()
                .iter()
                .skip(self.t)
                .for_each(|e| children.push(e.clone()));

            let binding = node.children.as_ref().borrow();
            let binding = binding.get(index + 1).unwrap();
            let mut binding = binding.children.as_ref().borrow_mut();
            *binding = children;

            let mut children: Vec<Node<K, T>> = Vec::new();

            full_child
                .children
                .as_ref()
                .borrow()
                .iter()
                .take(self.t - 1)
                .for_each(|e| children.push(e.clone()));

            let binding = node.children.as_ref().borrow();
            let binding = binding.get(index).unwrap();
            let mut binding = binding.children.as_ref().borrow_mut();
            *binding = children;
        }
    }

    pub fn insert(&mut self, key_value: (K, T)) -> Result<(), AlreadyPresent> {
        if let Some(_) = self.search(&key_value.0) {
            return Err(AlreadyPresent);
        } else if self.root.keys.as_ref().borrow().len() == 2 * self.t - 1 {
            println!("{:#?}", self);
            let new_root: Node<K, T> = Node::new(self.comp.clone());
            let old_root: &mut Node<K, T> = &mut self.root;

            let binding = new_root.clone();
            let mut binding = binding.children.borrow_mut();
            binding.insert(0, old_root.clone());

            std::mem::drop(binding);

            println!("{:#?}", self);
            self.split_child(new_root.clone(), 0);
            println!("{:#?}", self);
            self.insert_non_full(new_root.clone(), &key_value);

            return Ok(());
        } else {
            self.insert_non_full(self.root.clone(), &key_value);
            Ok(())
        }
    }

    fn insert_non_full(&mut self, node: Node<K, T>, key_value: &(K, T)) {
        let mut index = (node.keys.as_ref().borrow().len() as i32 - 1) as i32;

        if node.is_leaf {
            let mut binding = node.keys.as_ref().borrow_mut();
            binding.insert(KeyValue {
                0: key_value.0,
                1: key_value.1,
            });
        } else {
            while index >= 0 {
                if let Ordering::Less = self.comp.compare(
                    &key_value.0,
                    &node.keys.as_ref().borrow().get(index as usize).unwrap().0,
                ) {
                    index += 1;
                } else {
                    break;
                }
            }

            index += 1;

            let binding = node.clone();
            let binding = binding.children.as_ref().borrow();
            let binding = binding.get(index as usize).unwrap();
            let len = binding.keys.as_ref().borrow().len();

            if len == 2 * self.t - 1 {
                self.split_child(node.clone(), index as usize);

                if let Ordering::Greater = self.comp.compare(
                    &key_value.0,
                    &node.keys.as_ref().borrow().get(index as usize).unwrap().0,
                ) {
                    index += 1;
                }
            }

            let mut binding = node.children.as_ref().borrow_mut();
            let node_child = binding.get_mut(index as usize).unwrap();
            self.insert_non_full(node_child.clone(), &key_value);
        }
    }
}

// impl<K,T> Display for BTree<K,T> {
//
// }
