use crate::Frame;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvStoreError {
    #[error("Key not found")]
    KeyNotFound,
    #[error(transparent)]
    OpenLogError(#[from] io::Error),
    #[error("Failed to construct log table")]
    BuildLogTableFailure,
    #[error("Failed to serialize Command")]
    SerializationFailure,
    #[error("Failed to deserialize Command")]
    DeserializationFailure,
}

pub type Result<T> = std::result::Result<T, KvStoreError>;
type LogTable = HashMap<String, String>;

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

        self.append_to_log(command)?;
        self.log_table.insert(key.into(), value.into());

        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<String>> {
        Ok(self.log_table.get(key).map(|value| value.into()))
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

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(filename)
            .map_err(|err| KvStoreError::OpenLogError(err))?;

        let log_table = build_log_table(&file)?;

        Ok(Self { log_table, file })
    }

    fn append_to_log(&mut self, command: Command) -> Result<()> {
        let serialized =
            bincode::serialize(&command).map_err(|_| KvStoreError::SerializationFailure)?;

        let frame = Frame::new(&serialized).encode();
        let mut writer = BufWriter::new(&self.file);
        writer.write_all(&frame).unwrap();

        Ok(())
    }
}

fn build_log_table(file: &File) -> Result<LogTable> {
    let mut log_table = HashMap::new();
    let mut reader = BufReader::new(file);

    loop {
        let decoded = Frame::decode(&mut reader).map_err(|_| KvStoreError::BuildLogTableFailure)?;

        match decoded {
            Some(frame) => {
                let command = bincode::deserialize::<Command>(&frame.bytes)
                    .map_err(|_| KvStoreError::DeserializationFailure)?;

                match command {
                    Command::Set(key, value) => log_table.insert(key, value),
                    Command::Rm(key) => log_table.remove(&key),
                };
            }
            None => break,
        }
    }

    Ok(log_table)
}
