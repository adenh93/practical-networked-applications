use fake::Dummy;
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};

#[derive(Debug, PartialEq, Serialize, Deserialize, Dummy)]
pub enum Move {
    Up(u8),
    Right(u8),
    Down(u8),
    Left(u8),
}

#[derive(Debug)]
pub struct ReadBuffer {
    bytes: Vec<u8>,
    cursor: usize,
}

impl ReadBuffer {
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
            cursor: 0,
        }
    }
}

impl Default for ReadBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Write for ReadBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.bytes.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.bytes.flush()
    }
}

impl Read for ReadBuffer {
    fn read(&mut self, mut buf: &mut [u8]) -> std::io::Result<usize> {
        if self.cursor == self.bytes.len() {
            return Err(io::ErrorKind::UnexpectedEof)?;
        }

        let bytes_to_read = buf.len().min(self.bytes.len() - self.cursor);
        let slice = &self.bytes[self.cursor..self.cursor + bytes_to_read];
        buf.write_all(slice)?;
        self.cursor += bytes_to_read;

        Ok(bytes_to_read)
    }
}
