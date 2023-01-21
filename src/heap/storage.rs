use crate::util::Serializable;
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
    fn len(&self) -> usize {
        let mut len = 6 + 4;
        for entry in &self.entries {
            len += entry.len();
        }
        len
    }

    fn read(read: &mut impl Read) -> Result<Self> {
        for _ in 0..6 {
            read.read_i8()?;
        }
        let length = read.read_i32::<BigEndian>()?;
        if length > 454 {
            return Err(Error::new(ErrorKind::Other, "Maximum length is 454"));
        }
        let mut entries = Vec::with_capacity(length as _);
        for _ in 0..length {
            entries.push(StorageDevice::read(read)?);
        }
        Ok(Self { entries })
    }

    fn write(&self, write: &mut impl Write) -> Result<()> {
        for _ in 0..6 {
            write.write_i8(-1)?;
        }
        write.write_i32::<BigEndian>(self.entries.len() as _)?;
        for entry in &self.entries {
            entry.write(write)?;
        }
        Ok(())
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
    fn len(&self) -> usize {
        9
    }

    fn read(read: &mut impl Read) -> Result<Self> {
        let ptr = read.read_i64::<BigEndian>()?;
        let page_type = read.read_i8()?;
        Ok(Self { ptr, page_type })
    }

    fn write(&self, write: &mut impl Write) -> Result<()> {
        write.write_i64::<BigEndian>(self.ptr)?;
        write.write_i8(self.page_type)?;
        Ok(())
    }
}
