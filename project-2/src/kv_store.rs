use crate::Frame;
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter, Seek, SeekFrom, Write};
use std::path::Path;

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
        self.log_table.remove(key).context("Key not found")?;

        let command = Command::Rm(key.into());

        self.append_to_log(command)?;

        Ok(())
    }

    pub fn open(path: &Path) -> Result<KvStore> {
        let filename = path.join("logs.db");

        fs::create_dir_all(path)?;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(filename)?;

        let log_table = build_log_table(&mut file)?;

        Ok(Self { log_table, file })
    }

    fn append_to_log(&self, command: Command) -> Result<()> {
        let serialized = bincode::serialize(&command)?;

        let frame = Frame::new(&serialized).encode();
        let mut writer = BufWriter::new(&self.file);

        writer
            .write_all(&frame)
            .context("Failed to append serialized command to log")?;

        Ok(())
    }

    fn read_value_from_log(&self, offset: u64) -> Result<String> {
        let mut reader = BufReader::new(&self.file);
        reader.seek(SeekFrom::Start(offset)).unwrap();
        let decoded = Frame::decode(&mut reader)?;

        let result = match decoded {
            Some(frame) => {
                let command = bincode::deserialize::<Command>(&frame.bytes)?;

                match command {
                    Command::Set(_, value) => Ok(value),
                    _ => bail!("Unable to parse command from log"),
                }
            }
            None => bail!("Value not found in log at offset {}", offset),
        };

        reader.seek(SeekFrom::End(0)).unwrap();

        result
    }
}

fn build_log_table(file: &mut File) -> Result<LogTable> {
    let mut log_table = HashMap::new();
    let mut reader = BufReader::new(file);

    loop {
        let offset = reader.stream_position()?;
        let decoded = Frame::decode(&mut reader)?;

        match decoded {
            Some(frame) => {
                let command = bincode::deserialize::<Command>(&frame.bytes)?;

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
