use crate::{
    log::{LogCommand, LogFile, LogPointer},
    KvsError, Result,
};
use std::{collections::HashMap, path::Path};

#[derive(Debug)]
pub struct KvStore {
    log: LogFile,
    index: HashMap<String, LogPointer>,
}

impl KvStore {
    /// Instantiates a new Key/Value store.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let mut log = LogFile::open(path)?;
        let index = log.build_index()?;
        Ok(Self { log, index })
    }

    /// Sets the value of a `String` key to a `String`.
    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        let log_command = LogCommand::Set(key.to_owned(), value.to_owned());
        let pointer = self.log.append(log_command)?;
        self.index.insert(key.to_owned(), pointer);
        Ok(())
    }

    /// Gets the `String` value of a `String` key. If the key does not exist,
    // returns `None`.
    pub fn get(&self, key: &str) -> Result<Option<String>> {
        match self.index.get(key) {
            Some(pointer) => self.log.get(pointer),
            None => Ok(None),
        }
    }

    /// Removes a given key.
    pub fn remove(&mut self, key: &str) -> Result<()> {
        if !self.index.contains_key(key) {
            return Err(KvsError::KeyNotFound);
        }

        let log_command = LogCommand::Remove(key.to_owned());
        let _ = self.log.append(log_command)?;
        self.index.remove(key);
        Ok(())
    }
}
