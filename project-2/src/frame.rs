use std::io::{BufReader, Read, self};

#[derive(Debug)]
pub struct Frame {
    pub bytes: Vec<u8>,
}

impl Frame {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            bytes: bytes.to_vec(),
        }
    }
    pub fn encode(&self) -> Vec<u8> {
        let len_bytes = self.bytes.len().to_le_bytes();
        [&len_bytes[..], &self.bytes[..]].concat()
    }

    pub fn decode<T: Read>(stream: &mut BufReader<T>) -> io::Result<Option<Frame>> {
        let mut length_slice = [0u8; 8];
        stream.read(&mut length_slice)?;

        if length_slice.is_empty() {
            return Ok(None);
        }

        let length = usize::from_le_bytes(length_slice);
        let mut bytes = vec![0u8; length];
        stream.read(&mut bytes)?;

        if bytes.is_empty() {
            return Ok(None);
        }

        Ok(Some(Frame::new(&bytes)))
    }
}
