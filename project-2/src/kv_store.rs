use crate::Frame;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
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

        let serialized =
            bincode::serialize(&command).map_err(|_| KvStoreError::SerializationFailure)?;

        let frame = Frame::new(&serialized).encode();
        let mut writer = BufWriter::new(&self.file);
        writer.write_all(&frame).unwrap();

        self.log_table.insert(key.into(), value.into());

        Ok(())
    }

    pub fn get(&self, _key: &str) -> Result<Option<String>> {
        unimplemented!()
    }

    pub fn remove(&mut self, _key: &str) -> Result<()> {
        unimplemented!()
    }

    pub fn open(path: &Path) -> Result<KvStore> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(path)
            .map_err(|err| KvStoreError::OpenLogError(err))?;

        let log_table = build_log_table(&file)?;

        Ok(Self { log_table, file })
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
