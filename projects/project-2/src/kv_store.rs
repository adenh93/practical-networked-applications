use crate::{
    log::{Log, LogCommand, LogPointer},
    KvsError, Result,
};
use std::{collections::BTreeMap, io::Write, path::Path};

const UNCOMPACTED_BYTES_THRESHOLD: u64 = 1_024 * 1_024;

#[derive(Debug)]
pub struct KvStore {
    log: Log,
    index: BTreeMap<String, LogPointer>,
    uncompacted_bytes: u64,
}

impl KvStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let (uncompacted_bytes, index, log) = Log::init(path)?;

        Ok(Self {
            log,
            index,
            uncompacted_bytes,
        })
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        let log_command = LogCommand::Set(key.to_owned(), value.to_owned());
        let pointer = self.log.append(log_command)?;

        if let Some(prev_pointer) = self.index.insert(key.to_owned(), pointer) {
            self.add_uncompacted_bytes(prev_pointer.length)?;
        }

        Ok(())
    }

    pub fn get(&mut self, key: &str) -> Result<Option<String>> {
        match self.index.get(key) {
            Some(pointer) => self.log.get_value(pointer),
            None => Ok(None),
        }
    }

    pub fn remove(&mut self, key: &str) -> Result<()> {
        if !self.index.contains_key(key) {
            return Err(KvsError::KeyNotFound);
        }

        let log_command = LogCommand::Remove(key.to_owned());
        let _ = self.log.append(log_command)?;

        if let Some(prev_pointer) = self.index.remove(key) {
            self.add_uncompacted_bytes(prev_pointer.length)?;
        }

        Ok(())
    }

    fn add_uncompacted_bytes(&mut self, bytes_len: u64) -> Result<()> {
        self.uncompacted_bytes += bytes_len;

        if self.uncompacted_bytes > UNCOMPACTED_BYTES_THRESHOLD {
            self.compact()?;
        }

        Ok(())
    }

    fn compact(&mut self) -> Result<()> {
        let (commit_seq, mut commit_file) = self.log.prepare_commit()?;
        let mut offset = 0;

        for pointer in self.index.values_mut() {
            let bytes_written = self.log.stage_to_commit_file(&mut commit_file, pointer)?;
            pointer.update(commit_seq, offset, bytes_written);
            offset += bytes_written;
        }

        commit_file.flush()?;
        self.log.remove_stale_logs(commit_seq)?;
        self.uncompacted_bytes = 0;

        Ok(())
    }
}
