use std::fmt::Display;

pub use crate::app::btree::key_value::Comparator;
use crate::app::btree::KeyValue;
use crate::Error;

pub struct Split<K: Ord, V> {
    pub pair: KeyValue<K, V>,
    pub new_node: Node<K, V>,
}

impl<K: Ord, V> Split<K, V> {
    pub fn new(pair: KeyValue<K, V>, new_node: Node<K, V>) -> Self {
        Split { pair, new_node }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType<K: Ord, V> {
    Internal(Vec<KeyValue<K, V>>, Vec<Node<K, V>>),
    Leaf(Vec<KeyValue<K, V>>),
    Undefined,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<K: Ord, V> {
    pub node_type: NodeType<K, V>,
}

#[allow(dead_code)]
impl<K, V> Node<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    pub fn new(node_type: NodeType<K, V>) -> Self {
        Node { node_type }
    }

    pub fn split(&mut self, t: usize) -> Result<Split<K, V>, Error> {
        match self.node_type {
            NodeType::Internal(ref mut key_val_pairs, ref mut children) => {
                let mut sibling_pairs = key_val_pairs.split_off(t - 1);
                let median = sibling_pairs.remove(0);
                let sibling_children = children.split_off(t);

                Ok(Split::new(
                    median,
                    Node::new(NodeType::Internal(sibling_pairs, sibling_children)),
                ))
            }
            NodeType::Leaf(ref mut key_val_pairs) => {
                let sibling_pairs = key_val_pairs.split_off(t);
                let median = key_val_pairs.remove(t - 1);

                Ok(Split::new(median, Node::new(NodeType::Leaf(sibling_pairs))))
            }
            NodeType::Undefined => Err(Error::UnexpectedError),
        }
    }

    pub fn is_full(&self, t: usize) -> Result<bool, Error> {
        match self.node_type {
            NodeType::Internal(ref pairs, _) => Ok(pairs.len() >= 2 * t - 1),
            NodeType::Leaf(ref pairs) => Ok(pairs.len() >= 2 * t - 1),
            NodeType::Undefined => Err(Error::UnexpectedError),
        }
    }

    pub fn insert<C>(&mut self, pair: KeyValue<K, V>, child: Self) -> Result<(), Error>
    where
        C: Comparator<K>,
    {
        match self.node_type {
            NodeType::Internal(ref mut pairs, ref mut children) => {
                let index = match pairs.binary_search_by(|k| C::compare(&k.key, &pair.key)) {
                    Ok(_) => {
                        return Err(Error::KeyAlreadyExists);
                    }
                    Err(index) => index,
                };

                pairs.insert(index, pair);
                children.insert(index + 1, child);

                Ok(())
            }
            _ => Err(Error::UnexpectedError),
        }
    }
}

impl<K, V> Node<K, V>
where
    K: Clone + Ord + Display,
    V: Clone + Display,
{
    pub fn to_string(&self, level: usize) -> String {
        match self.node_type {
            NodeType::Internal(ref pairs, ref children) => {
                let pairs_str = format!(
                    "[{}]",
                    pairs
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                );
                let mut str = format!("LEVEL {}: {}\n", level, pairs_str);
                children
                    .iter()
                    .for_each(|child| str.push_str(&child.to_string(level + 1)[..]));
                str
            }
            NodeType::Leaf(ref pairs) => {
                let pairs_str = format!(
                    "[{}]",
                    pairs
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                );
                format!("LEVEL {}: {}\n", level, pairs_str)
            }
            NodeType::Undefined => "".to_string(),
        }
    }
}
