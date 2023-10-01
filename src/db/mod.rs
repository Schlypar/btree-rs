mod file_handler;
mod fixed_str;
pub mod goods;
pub mod person;

use std::marker::PhantomData;

use crate::btree::{key_value::Comparator, BTree};
use crate::error::Error;
use file_handler::{FileHandler, STRUCT_SIZE};
use goods::Crate;

#[derive(Debug)]
struct Comp;

#[derive(Debug, PartialEq, Eq)]
pub enum From {
    Sender,
    Receiver,
}

#[derive(Debug, PartialEq, Eq)]
pub enum KeyType {
    GoodsID,
    PostIndex(From),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Key {
    GoodsID(u64),
    PostIndex(u32),
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
    Indexed(BTree<Key, u64, Comp>, KeyType),
    NotIndexed,
}

#[derive(Debug)]
pub struct DataBase<'a, T> {
    file: FileHandler<'a>,
    len: usize,
    index: Index,
    _ph: PhantomData<T>,
}

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

    pub fn add_record(mut self, data: Crate) -> Result<Self, Error> {
        if let Index::Indexed(index, key_type) = self.index {
            let end_index = self.len as u64;

            match key_type {
                KeyType::GoodsID => {
                    self.index = Index::Indexed(
                        index.insert(Key::GoodsID(data.goods_id), end_index)?,
                        key_type,
                    );
                }
                KeyType::PostIndex(From::Sender) => {
                    self.index = Index::Indexed(
                        index.insert(Key::PostIndex(data.sender.post_index), end_index)?,
                        key_type,
                    );
                }
                KeyType::PostIndex(From::Receiver) => {
                    self.index = Index::Indexed(
                        index.insert(Key::PostIndex(data.receiver.post_index), end_index)?,
                        key_type,
                    );
                }
            }
        }

        self.file.seek_to_end()?;
        self.file.write::<Crate>(data, None)?;
        self.len += 1;

        Ok(DataBase {
            file: self.file,
            len: self.len,
            index: self.index,
            _ph: PhantomData,
        })
    }

    pub fn delete_record(&mut self, pos: u64) -> Result<Crate, Error> {
        let file_len = self.len() as u64;
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

    pub fn test_output(&mut self) -> Result<(), Error> {
        self.file.seek_to_start()?;
        while let Ok(data) = self.file.read::<Crate>(None) {
            println!("{:?}", data);
        }
        Ok(())
    }
}
