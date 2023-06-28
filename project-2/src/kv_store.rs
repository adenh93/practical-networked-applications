use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::Path;

type LogTable = HashMap<String, LogPointer>;

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
        let pointer = self.append_to_log(command)?;
        self.log_table.insert(key.into(), pointer);

        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<String>> {
        match self.log_table.get(key) {
            Some(pointer) => {
                let value = self.read_value_from_log(pointer)?;
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

    fn append_to_log(&self, command: Command) -> Result<LogPointer> {
        let mut writer = BufWriter::new(&self.file);
        let offset = writer.stream_position()?;
        let serialized = bincode::serialize(&command)?;
        let length = serialized.len();

        writer
            .write_all(&serialized)
            .context("Failed to append serialized command to log")?;

        Ok(LogPointer { offset, length })
    }

    fn read_value_from_log(&self, pointer: &LogPointer) -> Result<String> {
        let mut buffer = vec![0u8; pointer.length];
        let mut reader = BufReader::new(&self.file);

        reader.seek(SeekFrom::Start(pointer.offset))?;
        reader.read_exact(&mut buffer)?;

        let result = match bincode::deserialize::<Command>(&buffer)? {
            Command::Set(_, value) => Ok(value),
            _ => bail!("Unable to parse command from log"),
        };

        reader.seek(SeekFrom::End(0))?;

        result
    }
}

fn build_log_table(file: &mut File) -> Result<LogTable> {
    let mut log_table = HashMap::new();
    let mut reader = BufReader::new(file);
    let mut offset = reader.seek(SeekFrom::Start(0))?;

    while let Ok(cmd) = bincode::deserialize_from::<_, Command>(&mut reader) {
        let pos = reader.stream_position()?;
        let length = (pos - offset) as usize;

        match cmd {
            Command::Set(key, _) => log_table.insert(key, LogPointer { offset, length }),
            Command::Rm(key) => log_table.remove(&key),
        };

        offset = reader.seek(SeekFrom::Current(0))?;
    }

    Ok(log_table)
}

#[derive(Debug)]
struct LogPointer {
    offset: u64,
    length: usize,
}
