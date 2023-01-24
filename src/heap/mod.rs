use std::{
    fs::File,
    io::{Read, Result, Seek, SeekFrom, Write},
    path::Path,
};

pub mod material;
pub mod storage;

pub const NIL: i64 = -1;

pub struct Heap<T: Read + Write + Seek> {
    pub source: T,
}

impl<T: Read + Write + Seek> Heap<T> {
    pub fn new(source: T) -> Self {
        Self { source }
    }

    pub fn seek(&mut self, start_offset: u64) -> Result<()> {
        self.source.seek(SeekFrom::Start(start_offset))?;
        Ok(())
    }

    pub fn alloc_end(&mut self) -> Result<u64> {
        self.source.seek(SeekFrom::End(0))
    }
}

impl Heap<File> {
    pub fn load_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Self {
            source: File::open(path)?,
        })
    }

    pub fn create_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Self {
            source: File::create(path)?,
        })
    }
}
