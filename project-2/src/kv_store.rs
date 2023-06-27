use crate::Frame;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Seek, SeekFrom, Write};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvStoreError {
    #[error("Key not found")]
    KeyNotFound,
    #[error(transparent)]
    OpenLogError(#[from] io::Error),
    #[error("Failed to read from log")]
    ReadFromLogError,
    #[error("Failed to construct log table")]
    BuildLogTableError,
    #[error("Failed to serialize Command")]
    SerializationError,
    #[error("Failed to deserialize Command")]
    DeserializationError,
}

pub type Result<T> = std::result::Result<T, KvStoreError>;
type LogTable = HashMap<String, u64>;

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set(String, String),
    Rm(String),
}

#[derive(Debug)]
pub struct KvStore {
    log_table: LogTable,
    file: File,
}

impl KvStore {
    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        let command = Command::Set(key.into(), value.into());
        let offset = self.file.stream_position().unwrap();

        self.append_to_log(command)?;
        self.log_table.insert(key.into(), offset);

        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<String>> {
        match self.log_table.get(key) {
            Some(offset) => {
                let value = self.read_value_from_log(*offset)?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    pub fn remove(&mut self, key: &str) -> Result<()> {
        self.log_table
            .remove(key)
            .ok_or_else(|| KvStoreError::KeyNotFound)?;

        let command = Command::Rm(key.into());

        self.append_to_log(command)?;

        Ok(())
    }

    pub fn open(path: &Path) -> Result<KvStore> {
        let filename = path.join("logs.db");

        fs::create_dir_all(path).map_err(|err| KvStoreError::OpenLogError(err))?;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(filename)
            .map_err(|err| KvStoreError::OpenLogError(err))?;

        let log_table = build_log_table(&mut file)?;

        Ok(Self { log_table, file })
    }

    fn append_to_log(&self, command: Command) -> Result<()> {
        let serialized =
            bincode::serialize(&command).map_err(|_| KvStoreError::SerializationError)?;

        let frame = Frame::new(&serialized).encode();
        let mut writer = BufWriter::new(&self.file);
        writer.write_all(&frame).unwrap();

        Ok(())
    }

    fn read_value_from_log(&self, offset: u64) -> Result<String> {
        let mut reader = BufReader::new(&self.file);
        reader.seek(SeekFrom::Start(offset)).unwrap();
        let decoded = Frame::decode(&mut reader)?;

        let result = match decoded {
            Some(frame) => {
                let command = bincode::deserialize::<Command>(&frame.bytes)
                    .map_err(|_| KvStoreError::DeserializationError)?;

                match command {
                    Command::Set(_, value) => Ok(value),
                    _ => Err(KvStoreError::ReadFromLogError),
                }
            }
            None => Err(KvStoreError::ReadFromLogError),
        };

        reader.seek(SeekFrom::End(0)).unwrap();

        result
    }
}

fn build_log_table(file: &mut File) -> Result<LogTable> {
    let mut log_table = HashMap::new();
    let mut reader = BufReader::new(file);

    loop {
        let offset = reader.stream_position().map_err(|_| KvStoreError::BuildLogTableError)?;
        let decoded = Frame::decode(&mut reader).map_err(|_| KvStoreError::BuildLogTableError)?;

        match decoded {
            Some(frame) => {
                let command = bincode::deserialize::<Command>(&frame.bytes)
                    .map_err(|_| KvStoreError::DeserializationError)?;

                match command {
                    Command::Set(key, _) => log_table.insert(key, offset),
                    Command::Rm(key) => log_table.remove(&key),
                };
            }
            None => break,
        }
    }

    Ok(log_table)
}
