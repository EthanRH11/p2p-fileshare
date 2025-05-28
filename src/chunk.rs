use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write, BufReader};
use std::collections::HashSet;
use anyhow::Result;

// Represents a single piece of a file
pub struct Chunk {
    pub index: u64,

    pub data: Vec<u8>,
}

// Iterator over file chunks
pub struct FileChunks {
    reader: BufReader<File>,
    chunk_size: usize,
    total_len: u64,
    current_index: u64,
}

impl FileChunks {
    //Create a new chunk iterator:
    // - open the file and get its length
    // - Initialize index = 0;

    pub fn new(path: &std::path::Path, chunk_size: usize) -> Result<Self> {
        let file = File::open(path)?;
        let total_len = file.metadata()?.len();
        Ok(FileChunks {
            reader: BufReader::new(file),
            chunk_size,
            total_len,
            current_index: 0,
        })
    }
}

impl Iterator for FileChunks {
    type Item = Result<Chunk>;

    fn next(&mut self) -> Option<Self::Item> {
        let offset = self.current_index * self.chunk_size as u64;
        if offset >= self.total_len {
            //no more chunks
            return None;
        }

        if let Err(e) = self.reader.seek(SeekFrom::Start(offset)) {
            return Some(Err(e.into()));
        }

        //read up to chunk_size bytes
        let mut buf = vec![0; self.chunk_size];
        let n = match self.reader.read(&mut buf) {
            Ok(n) => n,
            Err(e) => return Some(Err(e.into())),
        };
        buf.truncate(n);

        //Build a chunk and increment index
        let chunk = Chunk {index: self.current_index, data: buf};
        self.current_index += 1;
        Some(Ok(chunk))
    }
}

// Track download progress and write pieces
pub struct DownloadState {
    total_chunks: u64,
    received: HashSet<u64>,
    output: File,
    chunk_size: u64,
}

impl DownloadState {
    pub fn new(total_chunks: u64, chunk_size: u64, out_path: &std::path::Path) -> Result<Self> {
        let mut f = File::create(out_path)?;
        f.set_len(total_chunks * chunk_size)?;
        OK(DownloadState {
            total_chunks,
            received: HashSet<u64>,
            output: f,
            chunk_size,
        })
    }

    //Write recieved chunk to correct offset
    pub fn write_chunk(&mut self, chunk: Chunk) -> Result<()> {
        let offset = chunk.index * self.chunk_size;
        self.output.seek(SeekFrom::Start(offset))?;
        self.output.write_all(&chunk.data)?;
        self.received.insert(chunk.index);
        Ok(())
    }

    // get next missing chunk index, or None if complete
    pub fn next_missing(&self) -> Option<u64> {
        (0..self.total_chunks).find(|i| !self.received.contains(i))
    }

    //check if all chunks have been recieved
    pub fn is_complete(&self) -> bool {
        self.received.len() as u64 == self.total_chunks
    }
}