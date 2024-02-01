use crate::utils::*;
use crate::{KvsError, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::io::{Read, Seek, Write};
use std::path::PathBuf;
use std::{
    fs::File,
    io::{BufReader, BufWriter, SeekFrom},
    path::Path,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum LogCommand {
    Set(String, String),
    Remove(String),
}

#[derive(Debug)]
pub struct LogPointer {
    pub file_id: u64,
    pub offset: u64,
    pub length: u64,
}

impl LogPointer {
    pub fn new(file_id: u64, offset: u64, length: u64) -> Self {
        Self {
            file_id,
            offset,
            length,
        }
    }

    pub fn update(&mut self, new_file_id: u64, new_offset: u64, new_length: u64) {
        *self = Self::new(new_file_id, new_offset, new_length);
    }
}

#[derive(Debug)]
pub struct Log {
    path: PathBuf,
    readers: BTreeMap<u64, BufReader<File>>,
    writer: BufWriter<File>,
    current_seq: u64,
}

impl Log {
    pub fn init(path: impl AsRef<Path>) -> Result<(u64, BTreeMap<String, LogPointer>, Self)> {
        let path = path.as_ref();
        fs::create_dir_all(path)?;

        let log_seqs = scan_log_seqs(path)?;
        let current_seq = log_seqs.last().unwrap_or(&0) + 1;
        let mut readers = open_log_readers(path, &log_seqs)?;
        let (uncompacted_bytes, index) = build_index(&mut readers)?;

        let (reader, writer) = new_log_pair(path, current_seq)?;
        readers.insert(current_seq, reader);

        let log = Self {
            path: path.to_owned(),
            readers,
            writer,
            current_seq,
        };

        Ok((uncompacted_bytes, index, log))
    }

    pub fn append(&mut self, log_command: LogCommand) -> Result<LogPointer> {
        let offset = self.writer.stream_position()?;
        bincode::serialize_into(&mut self.writer, &log_command).map_err(KvsError::AppendToLog)?;

        let length = self.writer.stream_position()? - offset;
        let pointer = LogPointer::new(self.current_seq, offset, length);
        self.writer.flush()?;

        Ok(pointer)
    }

    pub fn get(&mut self, log_pointer: &LogPointer) -> Result<LogCommand> {
        let reader = self.readers.get_mut(&log_pointer.file_id).unwrap();
        reader.seek(SeekFrom::Start(log_pointer.offset))?;
        let value = bincode::deserialize_from(reader).map_err(KvsError::ReadFromLog)?;

        Ok(value)
    }

    pub fn get_value(&mut self, log_pointer: &LogPointer) -> Result<Option<String>> {
        let value = match self.get(log_pointer)? {
            LogCommand::Set(_, value) => Some(value),
            _ => None,
        };

        Ok(value)
    }

    pub fn new_log_file(&mut self, new_seq: u64) -> Result<BufWriter<File>> {
        let (reader, writer) = new_log_pair(&self.path, new_seq)?;
        self.readers.insert(new_seq, reader);

        Ok(writer)
    }

    pub fn prepare_commit(&mut self) -> Result<(u64, BufWriter<File>)> {
        let commit_seq = self.current_seq + 1;
        let next_writer_seq = self.current_seq + 2;

        self.writer = self.new_log_file(next_writer_seq)?;
        self.current_seq = next_writer_seq;
        let commit_file = self.new_log_file(commit_seq)?;

        Ok((commit_seq, commit_file))
    }

    pub fn stage_to_commit_file(
        &mut self,
        commit_file: &mut BufWriter<File>,
        pointer: &LogPointer,
    ) -> Result<u64> {
        let reader = self.readers.get_mut(&pointer.file_id).unwrap();
        reader.seek(SeekFrom::Start(pointer.offset))?;
        let mut slice = reader.take(pointer.length);
        let bytes_written = std::io::copy(&mut slice, commit_file)?;

        Ok(bytes_written)
    }

    pub fn remove_stale_logs(&mut self, commit_seq: u64) -> Result<()> {
        let stale_seqs = self
            .readers
            .keys()
            .filter(|&&seq| seq < commit_seq)
            .cloned()
            .collect::<Vec<_>>();

        for seq in stale_seqs {
            self.readers.remove(&seq);
            remove_log_file(&self.path, seq)?;
        }

        Ok(())
    }
}
