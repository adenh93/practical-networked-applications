use crate::{KvsError, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufReader, Seek, SeekFrom},
    path::Path,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum LogCommand {
    Set(String, String),
    Remove(String),
}

#[derive(Debug)]
pub struct LogPointer {
    offset: u64,
}

#[derive(Debug)]
pub struct LogFile(File);

impl LogFile {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let filename = path.as_ref().join("log.0.txt");

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(filename)
            .map_err(KvsError::OpenFile)?;

        Ok(Self(file))
    }

    pub fn append(&mut self, log_command: LogCommand) -> Result<LogPointer> {
        let offset = self.0.stream_position()?;
        bincode::serialize_into(&self.0, &log_command).map_err(KvsError::AppendToLog)?;
        Ok(LogPointer { offset })
    }

    pub fn get(&self, log_pointer: &LogPointer) -> Result<Option<String>> {
        let mut reader = BufReader::new(&self.0);
        reader.seek(SeekFrom::Start(log_pointer.offset))?;

        let value = match bincode::deserialize_from(reader).map_err(KvsError::ReadFromLog)? {
            LogCommand::Set(_, value) => Some(value),
            _ => None,
        };

        Ok(value)
    }

    pub fn build_index(&mut self) -> Result<HashMap<String, LogPointer>> {
        let mut index = HashMap::new();
        let mut reader = BufReader::new(&self.0);
        let mut offset = reader.stream_position()?;

        while let Ok(result) = bincode::deserialize_from(&mut reader) {
            match result {
                LogCommand::Set(key, _) => index.insert(key, LogPointer { offset }),
                LogCommand::Remove(key) => index.remove(&key),
            };

            offset = reader.stream_position()?;
        }

        Ok(index)
    }
}
