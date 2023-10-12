pub mod file_handler;
mod fixed_str;
pub mod goods;
pub mod person;

use std::cell::RefCell;
use std::marker::PhantomData;

use crate::app::btree::{key_value::Comparator, BTree};
use crate::Error;
use file_handler::{FileHandler, STRUCT_SIZE};
use goods::Crate;

const DEGREE_OF_TREE: usize = 200;

pub trait Random {
    fn random() -> Self;
}

#[derive(Debug)]
struct Comp;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum From {
    Sender,
    Receiver,
}

impl From {
    fn is_sender(&self) -> bool {
        matches!(self, From::Sender)
    }

    fn is_receiver(&self) -> bool {
        !self.is_sender()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum KeyType {
    GoodsID,
    PostIndex(From),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Key {
    GoodsID(u64),
    PostIndex(u32),
}

impl Key {
    fn is_goods_id(&self) -> bool {
        matches!(self, Key::GoodsID(_))
    }
    fn is_post_index(&self) -> bool {
        !self.is_goods_id()
    }
}

impl Comparator<Key> for Comp {
    fn compare(lhs: &Key, rhs: &Key) -> std::cmp::Ordering {
        match lhs {
            Key::GoodsID(ref lhs_key) => match rhs {
                Key::GoodsID(ref rhs_key) => lhs_key.cmp(rhs_key),
                Key::PostIndex(_) => panic!(),
            },
            Key::PostIndex(ref lhs_key) => match rhs {
                Key::GoodsID(_) => panic!(),
                Key::PostIndex(ref rhs_key) => lhs_key.cmp(rhs_key),
            },
        }
    }
}

#[derive(Debug)]
enum Index {
    Indexed(BTree<Key, RefCell<Vec<u64>>, Comp>, KeyType),
    NotIndexed,
}

#[derive(Debug)]
pub struct DataBase<'a, T> {
    file: FileHandler<'a>,
    len: usize,
    index: Index,
    _ph: PhantomData<T>,
}

#[allow(dead_code)]
impl<'a> DataBase<'a, Crate> {
    pub fn new(mut file: FileHandler<'a>) -> Result<Self, Error> {
        file.open()?;
        let len = file.len()? as usize / STRUCT_SIZE;

        Ok(DataBase {
            file,
            len,
            index: Index::NotIndexed,
            _ph: PhantomData,
        })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clean(&mut self) -> Result<(), Error> {
        self.file.truncate(0)
    }

    pub fn peek(&mut self, pos: u64) -> Result<Crate, Error> {
        if self.len as u64 <= pos {
            return Err(Error::OutOfBounds);
        }
        self.file
            .read::<Crate>(Some(pos * STRUCT_SIZE as u64))
            .or(Err(Error::ErrorDeserializing))
    }

    pub fn indexed_by(&self) -> Option<KeyType> {
        match &self.index {
            Index::Indexed(_, key_type) => Some(*key_type),
            Index::NotIndexed => None,
        }
    }

    pub fn index(&mut self, key_type: KeyType) -> Result<(), Error> {
        self.file.seek_to_start()?;
        match key_type {
            KeyType::GoodsID => {
                let mut index: BTree<Key, RefCell<Vec<u64>>, Comp> =
                    BTree::with(DEGREE_OF_TREE).ok_or(Error::UnexpectedError)?;
                let mut pos: u64 = 0;
                while let Ok(data) = self.file.read::<Crate>(None) {
                    let already_contains = index.contains(Key::GoodsID(data.goods_id));
                    if !already_contains {
                        index.insert(Key::GoodsID(data.goods_id), RefCell::new(vec![pos]))?;
                    } else {
                        let pos_vec = index.search(Key::GoodsID(data.goods_id))?;
                        pos_vec.borrow_mut().push(pos);
                    }
                    pos += 1;
                }
                self.index = Index::Indexed(index, key_type);
            }
            KeyType::PostIndex(From::Sender) => {
                let mut index: BTree<Key, RefCell<Vec<u64>>, Comp> =
                    BTree::with(DEGREE_OF_TREE).ok_or(Error::UnexpectedError)?;
                let mut pos: u64 = 0;
                while let Ok(data) = self.file.read::<Crate>(None) {
                    let already_contains = index.contains(Key::PostIndex(data.sender.post_index));
                    if !already_contains {
                        index.insert(
                            Key::PostIndex(data.sender.post_index),
                            RefCell::new(vec![pos]),
                        )?;
                    } else {
                        let pos_vec = index.search(Key::PostIndex(data.sender.post_index))?;
                        pos_vec.borrow_mut().push(pos);
                    }
                    pos += 1;
                }
                self.index = Index::Indexed(index, key_type);
            }
            KeyType::PostIndex(From::Receiver) => {
                let mut index: BTree<Key, RefCell<Vec<u64>>, Comp> =
                    BTree::with(DEGREE_OF_TREE).ok_or(Error::UnexpectedError)?;
                let mut pos: u64 = 0;
                while let Ok(data) = self.file.read::<Crate>(None) {
                    let already_contains = index.contains(Key::PostIndex(data.receiver.post_index));
                    if !already_contains {
                        index.insert(
                            Key::PostIndex(data.receiver.post_index),
                            RefCell::new(vec![pos]),
                        )?;
                    } else {
                        let pos_vec = index.search(Key::PostIndex(data.receiver.post_index))?;
                        pos_vec.borrow_mut().push(pos);
                    }
                    pos += 1;
                }
                self.index = Index::Indexed(index, key_type);
            }
        }
        Ok(())
    }

    pub fn search_unindexed(
        &mut self,
        key: Key,
        which_post_index: Option<From>,
    ) -> Option<Vec<u64>> {
        self.file.seek_to_start().ok()?;
        let mut poss: Vec<u64> = vec![];
        let mut pos: u64 = 0;

        match key {
            Key::GoodsID(id) => {
                while let Ok(data) = self.file.read::<Crate>(None) {
                    if data.goods_id == id {
                        poss.push(pos);
                    }
                    pos += 1;
                }
            }
            Key::PostIndex(index) => {
                assert!(
                    which_post_index.is_some(),
                    "For this query this must be true"
                );
                match which_post_index.unwrap() {
                    From::Sender => {
                        while let Ok(data) = self.file.read::<Crate>(None) {
                            if data.sender.post_index == index {
                                poss.push(pos);
                            }
                            pos += 1;
                        }
                    }
                    From::Receiver => {
                        while let Ok(data) = self.file.read::<Crate>(None) {
                            if data.receiver.post_index == index {
                                poss.push(pos);
                            }
                            pos += 1;
                        }
                    }
                }
            }
        };

        if poss.is_empty() {
            None
        } else {
            Some(poss)
        }
    }

    pub fn search_indexed(&mut self, key: Key, which_post_index: Option<From>) -> Option<Vec<u64>> {
        if let Index::Indexed(ref index, ref key_type) = self.index {
            match key_type {
                KeyType::GoodsID => {
                    assert!(key.is_goods_id(), "For this query this must be true");
                    Some(index.search(key).ok()?.borrow().clone())
                }
                KeyType::PostIndex(From::Sender) => {
                    assert!(key.is_post_index(), "For this query this must be true");
                    assert!(
                        which_post_index.is_some(),
                        "For this query this must be true"
                    );
                    assert!(
                        which_post_index.unwrap().is_sender(),
                        "For this query this must be true"
                    );
                    Some(index.search(key).ok()?.borrow().clone())
                }
                KeyType::PostIndex(From::Receiver) => {
                    assert!(key.is_post_index(), "For this query this must be true");
                    assert!(
                        which_post_index.is_some(),
                        "For this query this must be true"
                    );
                    assert!(
                        which_post_index.unwrap().is_receiver(),
                        "For this query this must be true"
                    );
                    Some(index.search(key).ok()?.borrow().clone())
                }
            }
        } else {
            self.search_unindexed(key, which_post_index)
        }
    }

    pub fn add_record(&mut self, data: Crate) -> Result<(), Error> {
        if let Index::Indexed(ref mut index, key_type) = self.index {
            let end_index = self.len as u64;
            match key_type {
                KeyType::GoodsID => {
                    let pos_vec = index
                        .search(Key::GoodsID(data.goods_id))
                        .unwrap_or(&RefCell::new(vec![]))
                        .clone();
                    pos_vec.borrow_mut().push(end_index);
                    let pos_vec_clone = pos_vec.clone();
                    std::mem::drop(pos_vec);
                    index.insert(Key::GoodsID(data.goods_id), pos_vec_clone)?;
                }
                KeyType::PostIndex(From::Sender) => {
                    let pos_vec = index
                        .search(Key::PostIndex(data.sender.post_index))
                        .unwrap_or(&RefCell::new(vec![]))
                        .clone();
                    pos_vec.borrow_mut().push(end_index);
                    let pos_vec_clone = pos_vec.clone();
                    std::mem::drop(pos_vec);
                    index.insert(Key::PostIndex(data.sender.post_index), pos_vec_clone)?;
                }
                KeyType::PostIndex(From::Receiver) => {
                    let pos_vec = index
                        .search(Key::PostIndex(data.receiver.post_index))
                        .unwrap_or(&RefCell::new(vec![]))
                        .clone();
                    pos_vec.borrow_mut().push(end_index);
                    let pos_vec_clone = pos_vec.clone();
                    std::mem::drop(pos_vec);
                    index.insert(Key::PostIndex(data.receiver.post_index), pos_vec_clone)?;
                }
            }
        }

        self.file.seek_to_end()?;
        self.file.write::<Crate>(data, None)?;
        self.len += 1;

        Ok(())
    }

    pub fn delete_record(&mut self, pos: u64) -> Result<Crate, Error> {
        let file_len = self.len as u64;
        if file_len <= pos {
            return Err(Error::OutOfBounds);
        }

        let deleted = self.file.read::<Crate>(Some(pos * STRUCT_SIZE as u64))?;

        self.file.seek_to_start()?;
        self.file.seek((pos * STRUCT_SIZE as u64) as i64)?;

        let (mut curr, mut next) = (pos, pos + 1);

        while next < file_len {
            let next_data = self.file.read::<Crate>(Some(next * STRUCT_SIZE as u64))?;
            self.file
                .write(next_data, Some(curr * STRUCT_SIZE as u64))?;
            curr += 1;
            next += 1;
        }

        self.len -= 1;
        self.file.truncate((file_len - 1) * STRUCT_SIZE as u64)?;
        self.index = Index::NotIndexed;

        Ok(deleted)
    }
}
