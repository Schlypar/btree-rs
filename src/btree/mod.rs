pub mod key_value;
mod node;

use std::fmt::{Debug, Display};
use std::marker::PhantomData;

use crate::error::Error;
use key_value::KeyValue;
use node::{Comparator, NodeType};
use node::{Node, Split};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BTree<K, V, C>
where
    K: Ord,
    C: Comparator<K>,
{
    root: Option<Node<K, V>>,
    t: usize,
    cmp: PhantomData<C>,
}

#[allow(dead_code)]
impl<K, V, C> BTree<K, V, C>
where
    K: Copy + Clone + Ord,
    V: Clone,
    C: Comparator<K>,
{
    pub fn new() -> Self {
        BTree {
            root: None,
            t: 2,
            cmp: PhantomData,
        }
    }

    pub fn with(t: usize) -> Option<Self> {
        if t < 2 {
            return None;
        }

        Some(BTree {
            root: None,
            t,
            cmp: PhantomData,
        })
    }

    fn search_node<'a>(
        &self,
        node: &'a Node<K, V>,
        key: K,
    ) -> Result<(&'a Node<K, V>, usize), Error> {
        match node.node_type {
            NodeType::Internal(ref pairs, ref children) => {
                let index = match pairs.binary_search_by(|k| C::compare(&k.key, &key)) {
                    Ok(index) => {
                        return Ok((node, index));
                    }
                    Err(index) => index,
                };

                self.search_node(children.get(index).ok_or(Error::UnexpectedError)?, key)
            }
            NodeType::Leaf(ref pairs) => {
                match pairs.binary_search_by(|k| C::compare(&k.key, &key)) {
                    Ok(index) => Ok((node, index)),
                    Err(_) => Err(Error::KeyWasNotFound),
                }
            }
            NodeType::Undefined => Err(Error::UnexpectedError),
        }
    }

    pub fn search(&self, key: K) -> Result<&V, Error> {
        let (node, at) = self.search_node(self.root.as_ref().unwrap(), key)?;

        match node.node_type {
            NodeType::Internal(ref pairs, _) => Ok(&pairs.get(at).unwrap().value),
            NodeType::Leaf(ref pairs) => Ok(&pairs.get(at).unwrap().value),
            NodeType::Undefined => Err(Error::UnexpectedError),
        }
    }

    pub fn insert(mut self, key: K, value: V) -> Result<Self, Error> {
        if self.root.is_none() {
            self.root = Some(Node::new(NodeType::Leaf(vec![(key, value).into()])));
            return Ok(BTree {
                root: self.root,
                t: self.t,
                cmp: PhantomData,
            });
        }

        let mut root = self.root.unwrap();
        self.root = None;

        let is_root_full = root.is_full(self.t)?;
        if is_root_full {
            let split = root.split(self.t)?;
            let mut new_root = Node::new(NodeType::Internal(
                vec![split.pair],
                vec![root, split.new_node],
            ));

            self.insert_recursive(&mut new_root, key, value)?;
            self.root = Some(new_root);

            return Ok(BTree {
                root: self.root,
                t: self.t,
                cmp: PhantomData,
            });
        }

        if let Some(split) = match root.node_type {
            NodeType::Internal(ref pairs, ref mut children) => {
                let index = match pairs.binary_search_by(|k| C::compare(&k.key, &key)) {
                    Ok(_) => return Err(Error::KeyAlreadyExists),
                    Err(index) => index,
                };

                let is_child_full = children
                    .get_mut(index)
                    .ok_or(Error::KeyWasNotFound)?
                    .is_full(self.t)?;

                if !is_child_full {
                    None
                } else {
                    self.insert_recursive(
                        children.get_mut(index).ok_or(Error::KeyWasNotFound)?,
                        key,
                        value.clone(),
                    )?
                }
            }
            NodeType::Leaf(ref mut pairs) => {
                let index = match pairs.binary_search_by(|k| C::compare(&k.key, &key)) {
                    Ok(_) => return Err(Error::KeyAlreadyExists),
                    Err(index) => index,
                };
                pairs.insert(index, (key, value.clone()).into());
                return Ok(BTree {
                    root: Some(root),
                    t: self.t,
                    cmp: PhantomData,
                });
            }
            NodeType::Undefined => return Err(Error::UnexpectedError),
        } {
            root.insert::<C>(split.pair, split.new_node)?;
        }

        match root.node_type {
            NodeType::Internal(ref pairs, ref mut children) => {
                let index = match pairs.binary_search_by(|k| C::compare(&k.key, &key)) {
                    Ok(_) => return Err(Error::KeyAlreadyExists),
                    Err(index) => index,
                };
                self.insert_recursive(
                    children.get_mut(index).ok_or(Error::KeyWasNotFound)?,
                    key,
                    value,
                )?;
            }
            NodeType::Leaf(ref mut pairs) => {
                let index = match pairs.binary_search_by(|k| C::compare(&k.key, &key)) {
                    Ok(_) => return Err(Error::KeyAlreadyExists),
                    Err(index) => index,
                };
                pairs.insert(index, (key, value.clone()).into());
            }
            NodeType::Undefined => return Err(Error::UnexpectedError),
        }

        self.root = Some(root);
        Ok(BTree {
            root: self.root,
            t: self.t,
            cmp: PhantomData,
        })
    }

    fn insert_recursive(
        &mut self,
        node: &mut Node<K, V>,
        key: K,
        value: V,
    ) -> Result<Option<Split<K, V>>, Error> {
        let is_full = node.is_full(self.t)?;
        if is_full {
            let split = node.split(self.t)?;
            return Ok(Some(split));
        }

        if let Some(split) = match node.node_type {
            NodeType::Internal(ref pairs, ref mut children) => {
                let index = match pairs.binary_search_by(|k| C::compare(&k.key, &key)) {
                    Ok(_) => return Err(Error::KeyAlreadyExists),
                    Err(index) => index,
                };

                let is_child_full = children
                    .get_mut(index)
                    .ok_or(Error::KeyWasNotFound)?
                    .is_full(self.t)?;

                if !is_child_full {
                    self.insert_recursive(
                        children.get_mut(index).ok_or(Error::KeyWasNotFound)?,
                        key,
                        value.clone(),
                    )?;
                    return Ok(None);
                } else {
                    self.insert_recursive(
                        children.get_mut(index).ok_or(Error::KeyWasNotFound)?,
                        key,
                        value.clone(),
                    )?
                }
            }
            NodeType::Leaf(ref mut pairs) => {
                let index = match pairs.binary_search_by(|k| C::compare(&k.key, &key)) {
                    Ok(_) => return Err(Error::KeyAlreadyExists),
                    Err(index) => index,
                };
                pairs.insert(index, (key, value.clone()).into());
                return Ok(None);
            }
            NodeType::Undefined => return Err(Error::UnexpectedError),
        } {
            node.insert::<C>(split.pair, split.new_node)?;
        }

        match node.node_type {
            NodeType::Internal(ref pairs, ref mut children) => {
                let index = match pairs.binary_search_by(|k| C::compare(&k.key, &key)) {
                    Ok(_) => return Err(Error::KeyAlreadyExists),
                    Err(index) => index,
                };
                self.insert_recursive(
                    children.get_mut(index).ok_or(Error::KeyWasNotFound)?,
                    key,
                    value,
                )
            }
            NodeType::Leaf(ref mut pairs) => {
                let index = match pairs.binary_search_by(|k| C::compare(&k.key, &key)) {
                    Ok(_) => return Err(Error::KeyAlreadyExists),
                    Err(index) => index,
                };
                pairs.insert(index, (key, value).into());
                Ok(None)
            }
            NodeType::Undefined => Err(Error::UnexpectedError),
        }
    }
}

impl<K, V, C> Default for BTree<K, V, C>
where
    K: Copy + Clone + Ord,
    V: Clone,
    C: Comparator<K>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V, C> Display for BTree<K, V, C>
where
    K: Copy + Clone + Ord + Display,
    V: Clone + Display,
    C: Comparator<K>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.root {
            Some(ref root) => {
                write!(f, "{}", root.to_string(0))
            }
            None => {
                write!(f, "Empty BTree")
            }
        }
    }
}
