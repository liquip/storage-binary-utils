use crate::util::Serializable;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Error, ErrorKind, Read, Result, Write};

#[derive(Debug)]
pub struct Meta {
    version: i16,
    entries: Vec<MetaEntry>,
}

impl Meta {
    pub fn new(version: i16, entries: Vec<MetaEntry>) -> Result<Self> {
        if entries.len() > i32::MAX as _ {
            return Err(Error::new(
                ErrorKind::Other,
                "Maximum amount of entries was exceeded",
            ));
        }
        Ok(Self { version, entries })
    }

    pub fn push_entry(&mut self, entry: MetaEntry) -> Result<()> {
        if self.entries.len() >= i32::MAX as _ {
            return Err(Error::new(
                ErrorKind::Other,
                "Maximum amount of entries was exceeded",
            ));
        }
        self.entries.push(entry);
        Ok(())
    }
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            version: 1,
            entries: Vec::with_capacity(0),
        }
    }
}

impl Serializable for Meta {
    fn len(&self) -> usize {
        let mut len = 2 + 4;
        for entry in &self.entries {
            len += entry.len();
        }
        len
    }

    fn read(read: &mut impl Read) -> Result<Self> {
        let version = read.read_i16::<BigEndian>()?;
        let length = read.read_i32::<BigEndian>()?;
        let mut entries = Vec::with_capacity(length as _);
        for _ in 0..length {
            entries.push(MetaEntry::read(read)?);
        }
        Ok(Self { version, entries })
    }

    fn write(&self, write: &mut impl Write) -> Result<()> {
        write.write_i16::<BigEndian>(self.version)?;
        write.write_i32::<BigEndian>(self.entries.len() as _)?;
        for entry in &self.entries {
            entry.write(write)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct MetaEntry {
    material_key: String,
    extended_ptr: i64,
}

impl MetaEntry {
    pub fn new(material_key: String, extended_ptr: i64) -> Self {
        Self {
            material_key,
            extended_ptr,
        }
    }
}

impl Serializable for MetaEntry {
    fn len(&self) -> usize {
        4 + self.material_key.len() + 8
    }

    fn read(read: &mut impl Read) -> Result<Self> {
        let length = read.read_i32::<BigEndian>()? as usize;
        let mut material_key = String::with_capacity(length);
        if read.take(length as _).read_to_string(&mut material_key)? != length {
            return Err(Error::new(ErrorKind::Other, "Unexpected end of source"));
        }
        let extended_ptr = read.read_i64::<BigEndian>()?;
        Ok(Self {
            material_key,
            extended_ptr,
        })
    }

    fn write(&self, write: &mut impl Write) -> Result<()> {
        write.write_i32::<BigEndian>(self.material_key.len() as _)?;
        write.write_all(self.material_key.as_bytes())?;
        write.write_i64::<BigEndian>(self.extended_ptr)?;
        Ok(())
    }
}
