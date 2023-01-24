use crate::util::{verify_ptr, Serializable};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Result, Write};

#[derive(Debug)]
pub struct Meta {
    pub version: i16,
    pub material_list_ptr: i64,
    pub storage_list_ptr: i64,
}

impl Meta {
    pub fn new(version: i16, material_list_ptr: i64, storage_list_ptr: i64) -> Self {
        Self {
            version,
            material_list_ptr,
            storage_list_ptr,
        }
    }
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            version: 1,
            material_list_ptr: crate::heap::NIL,
            storage_list_ptr: crate::heap::NIL,
        }
    }
}

impl Serializable for Meta {
    fn size(&self) -> usize {
        2 + 8 + 8
    }

    fn read(read: &mut impl Read) -> Result<Self> {
        Ok(Self {
            version: read.read_i16::<BigEndian>()?,
            material_list_ptr: verify_ptr(read.read_i64::<BigEndian>()?)?,
            storage_list_ptr: verify_ptr(read.read_i64::<BigEndian>()?)?,
        })
    }

    fn write(&self, write: &mut impl Write) -> Result<()> {
        write.write_i16::<BigEndian>(self.version)?;
        write.write_i64::<BigEndian>(verify_ptr(self.material_list_ptr)?)?;
        write.write_i64::<BigEndian>(verify_ptr(self.storage_list_ptr)?)
    }
}
