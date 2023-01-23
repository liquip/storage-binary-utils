use crate::util::{read_padding, read_slice, write_padding, write_slice, Serializable};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Error, ErrorKind, Read, Result, Write};

#[derive(Debug)]
pub struct Storage {
    entries: Vec<StorageDevice>,
}

impl Storage {
    pub fn new(entries: Vec<StorageDevice>) -> Result<Self> {
        if entries.len() > 454 {
            return Err(Error::new(ErrorKind::Other, "Maximum length is 454"));
        }
        Ok(Self { entries })
    }

    pub fn push_entry(&mut self, entry: StorageDevice) -> Result<()> {
        if self.entries.len() >= 454 {
            return Err(Error::new(ErrorKind::Other, "Maximum length is 454"));
        }
        self.entries.push(entry);
        Ok(())
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self {
            entries: Vec::with_capacity(0),
        }
    }
}

impl Serializable for Storage {
    fn size(&self) -> usize {
        let mut size = 6 + 4;
        for entry in &self.entries {
            size += entry.size();
        }
        size
    }

    fn read(read: &mut impl Read) -> Result<Self> {
        read_padding(read, 6)?;
        Ok(Self {
            entries: read_slice(read, 454)?,
        })
    }

    fn write(&self, write: &mut impl Write) -> Result<()> {
        write_padding(write, 6)?;
        write_slice(write, &self.entries)
    }
}

#[derive(Debug)]
pub struct StorageDevice {
    ptr: i64,
    page_type: i8,
}

impl StorageDevice {
    pub fn new(ptr: i64, page_type: i8) -> Self {
        Self { ptr, page_type }
    }
}

impl Serializable for StorageDevice {
    fn size(&self) -> usize {
        9
    }

    fn read(read: &mut impl Read) -> Result<Self> {
        let ptr = read.read_i64::<BigEndian>()?;
        let page_type = read.read_i8()?;
        Ok(Self { ptr, page_type })
    }

    fn write(&self, write: &mut impl Write) -> Result<()> {
        write.write_i64::<BigEndian>(self.ptr)?;
        write.write_i8(self.page_type)
    }
}
