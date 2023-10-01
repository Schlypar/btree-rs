use std::{
    fs::{self, File, OpenOptions},
    io::{BufReader, Seek, SeekFrom},
    path::Path,
};

pub use crate::db::goods::Crate;
use bincode::{deserialize_from, serialize_into};
pub use serde::{de::DeserializeOwned, Deserialize, Serialize};

// pub const STRUCT_SIZE: usize = std::mem::size_of::<Crate>();
pub const STRUCT_SIZE: usize = 216;

use crate::error::Error;

#[derive(Debug)]
enum FileState<R> {
    Opened(BufReader<R>),
    Closed,
}

#[derive(Debug)]
pub struct FileHandler<'a> {
    path: &'a Path,
    state: FileState<File>,
}

impl<'a> FileHandler<'a> {
    pub fn new(path: &'a Path) -> Self {
        FileHandler {
            path,
            state: FileState::Closed,
        }
    }

    pub fn create_dir(path: &'a Path) -> Result<(), Error> {
        match path.exists() {
            true => Err(Error::DirAlreadyExists),
            false => fs::create_dir(path).or(Err(Error::UnexpectedError)),
        }
    }

    pub fn create_file(path: &'a Path) -> Result<File, Error> {
        match path.exists() {
            true => Err(Error::DirAlreadyExists),
            false => File::create(path).or(Err(Error::UnexpectedError)),
        }
    }

    pub fn open(&mut self) -> Result<(), Error> {
        match self.state {
            FileState::Opened(_) => Err(Error::FileAlreadyOpened),
            FileState::Closed => match self.path.exists() {
                true => {
                    let file = OpenOptions::new().read(true).write(true).open(self.path)?;
                    self.state = FileState::Opened(BufReader::with_capacity(STRUCT_SIZE, file));
                    Ok(())
                }
                false => Err(Error::PathDoesntExist),
            },
        }
    }

    pub fn close(&mut self) {
        self.state = FileState::Closed;
    }

    pub fn get_current_pos(&mut self) -> Result<u64, Error> {
        match self.state {
            FileState::Opened(ref mut file) => Ok(file.stream_position()?),
            FileState::Closed => Err(Error::OutOfBounds),
        }
    }

    pub fn len(&mut self) -> Result<u64, Error> {
        self.seek_to_start()?;
        match self.state {
            FileState::Opened(ref mut file) => Ok(file.seek(SeekFrom::End(0))?),
            FileState::Closed => {
                let mut file = File::open(self.path)?;
                Ok(file.seek(SeekFrom::End(0))?)
            }
        }
    }

    pub fn is_empty(&mut self) -> Result<bool, Error> {
        Ok(0 == self.len()?)
    }

    pub fn truncate(&mut self, size: u64) -> Result<(), Error> {
        match self.state {
            FileState::Opened(ref mut writer) => {
                let writer = writer.get_mut();
                writer.set_len(size)?;
                Ok(())
            }
            FileState::Closed => Err(Error::UnexpectedError),
        }
    }

    pub fn seek(&mut self, bytes: i64) -> Result<(), Error> {
        match self.state {
            FileState::Opened(ref mut file) => {
                let _ = file.seek(SeekFrom::Current(bytes))?;
                Ok(())
            }
            FileState::Closed => Err(Error::UnexpectedError),
        }
    }

    pub fn seek_to_start(&mut self) -> Result<(), Error> {
        match self.state {
            FileState::Opened(ref mut file) => {
                let _ = file.seek(SeekFrom::Start(0))?;
                Ok(())
            }
            FileState::Closed => Err(Error::UnexpectedError),
        }
    }

    pub fn seek_to_end(&mut self) -> Result<(), Error> {
        match self.state {
            FileState::Opened(ref mut file) => {
                let _ = file.seek(SeekFrom::End(0))?;
                Ok(())
            }
            FileState::Closed => Err(Error::UnexpectedError),
        }
    }

    pub fn sync_all(&mut self) -> Result<(), Error> {
        if let FileState::Opened(ref mut file) = self.state {
            let writer = file.get_mut();
            writer.sync_all()?;
        }
        Ok(())
    }

    pub fn read<T: DeserializeOwned>(&mut self, from: Option<u64>) -> Result<T, Error> {
        if from.is_some() {
            self.seek_to_start()?;
            self.seek(from.unwrap() as i64)?;
        }
        match self.state {
            FileState::Opened(ref mut file) => match deserialize_from::<_, T>(file) {
                Ok(data) => Ok(data),
                Err(_) => Err(Error::ErrorDeserializing),
            },
            FileState::Closed => Err(Error::UnexpectedError),
        }
    }

    pub fn write<T: Serialize>(&mut self, data: T, from: Option<u64>) -> Result<(), Error> {
        if from.is_some() {
            self.seek_to_start()?;
            self.seek(from.unwrap() as i64)?;
        }
        match self.state {
            FileState::Opened(ref mut file) => {
                let writer = file.get_mut();
                match serialize_into::<_, T>(writer, &data) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(Error::ErrorSerializing),
                }
            }
            FileState::Closed => Err(Error::UnexpectedError),
        }
    }
}

impl<'a> From<&'a Path> for FileHandler<'a> {
    fn from(value: &'a Path) -> Self {
        FileHandler::new(value)
    }
}
