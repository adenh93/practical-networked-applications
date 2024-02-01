use crate::{
    log::{LogCommand, LogPointer},
    KvsError, Result,
};
use std::{
    collections::BTreeMap,
    ffi::OsStr,
    fs::{self, File, OpenOptions},
    io::{BufReader, BufWriter, Seek, SeekFrom},
    path::{Path, PathBuf},
};

pub fn get_log_path(path: impl AsRef<Path>, seq: u64) -> PathBuf {
    let filename = format!("{seq}.log");
    path.as_ref().join(&filename)
}

pub fn is_log_file(path: &PathBuf) -> bool {
    path.is_file() && path.extension() == Some("log".as_ref())
}

pub fn new_log_reader(path: impl AsRef<Path>, seq: u64) -> Result<BufReader<File>> {
    let log_path = get_log_path(path, seq);
    let file = File::open(log_path)?;
    Ok(BufReader::new(file))
}

pub fn new_log_writer(path: impl AsRef<Path>, seq: u64) -> Result<BufWriter<File>> {
    let log_path = get_log_path(path, seq);

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(log_path)
        .map_err(KvsError::OpenFile)?;

    Ok(BufWriter::new(file))
}

pub fn new_log_pair(
    path: impl AsRef<Path>,
    seq: u64,
) -> Result<(BufReader<File>, BufWriter<File>)> {
    let path = path.as_ref();
    let writer = new_log_writer(path, seq)?;
    let reader = new_log_reader(path, seq)?;
    Ok((reader, writer))
}

pub fn open_log_readers(
    path: impl AsRef<Path>,
    seqs: &Vec<u64>,
) -> Result<BTreeMap<u64, BufReader<File>>> {
    let path = path.as_ref();

    seqs.into_iter()
        .map(|seq| {
            let reader = new_log_reader(path, *seq)?;
            Ok((*seq, reader))
        })
        .collect()
}

pub fn scan_log_seqs(path: impl AsRef<Path>) -> Result<Vec<u64>> {
    let log_seqs = fs::read_dir(&path)?
        .filter_map(|entry| {
            entry.ok().filter(|e| is_log_file(&e.path())).and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(OsStr::to_str)
                    .map(|s| s.trim_end_matches(".log"))
                    .and_then(|s| s.parse().ok())
            })
        })
        .collect::<Vec<_>>();

    Ok(log_seqs)
}

pub fn remove_log_file(path: impl AsRef<Path>, seq: u64) -> Result<()> {
    let filename = get_log_path(path, seq);
    fs::remove_file(filename)?;
    Ok(())
}

pub fn build_index(
    readers: &mut BTreeMap<u64, BufReader<File>>,
) -> Result<(u64, BTreeMap<String, LogPointer>)> {
    let mut index = BTreeMap::new();
    let mut uncompacted_bytes = 0;

    for (seq, reader) in readers.iter_mut() {
        let mut offset = 0;
        reader.seek(SeekFrom::Start(0))?;

        while let Ok(command) = bincode::deserialize_from(&mut *reader) {
            let position = reader.stream_position()?;
            let pointer_length = position - offset;

            match command {
                LogCommand::Set(key, _) => {
                    let pointer = LogPointer::new(*seq, offset, pointer_length);

                    if let Some(prev_pointer) = index.insert(key, pointer) {
                        uncompacted_bytes += prev_pointer.length;
                    }
                }
                LogCommand::Remove(key) => {
                    if let Some(prev_pointer) = index.remove(&key) {
                        uncompacted_bytes += prev_pointer.length;
                    }
                }
            };

            offset = position;
        }
    }

    Ok((uncompacted_bytes, index))
}
