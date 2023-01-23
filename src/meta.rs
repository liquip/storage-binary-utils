use crate::util::Serializable;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Result, Write};

#[derive(Debug)]
pub struct Meta {
    version: i16,
    material_list_ptr: i64,
    storage_list_ptr: i64,
}

impl Meta {
    pub fn new(version: i16, material_list_ptr: i64, storage_list_ptr: i64) -> Result<Self> {
        Ok(Self {
            version,
            material_list_ptr,
            storage_list_ptr,
        })
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
        let version = read.read_i16::<BigEndian>()?;
        let material_list_ptr = read.read_i64::<BigEndian>()?;
        let storage_list_ptr = read.read_i64::<BigEndian>()?;
        Ok(Self {
            version,
            material_list_ptr,
            storage_list_ptr,
        })
    }

    fn write(&self, write: &mut impl Write) -> Result<()> {
        write.write_i16::<BigEndian>(self.version)?;
        write.write_i64::<BigEndian>(self.material_list_ptr)?;
        write.write_i64::<BigEndian>(self.storage_list_ptr)
    }
}

// #[derive(Debug)]
// pub struct MetaEntry {
//     material_key: String,
//     extended_ptr: i64,
// }

// impl MetaEntry {
//     pub fn new(material_key: String, extended_ptr: i64) -> Self {
//         Self {
//             material_key,
//             extended_ptr,
//         }
//     }
// }

// impl Serializable for MetaEntry {
//     fn len(&self) -> usize {
//         4 + self.material_key.len() + 8
//     }

//     fn read(read: &mut impl Read) -> Result<Self> {
//         let length = read.read_i32::<BigEndian>()? as usize;
//         let mut material_key = String::with_capacity(length);
//         if read.take(length as _).read_to_string(&mut material_key)? != length {
//             return Err(Error::new(ErrorKind::Other, "Unexpected end of source"));
//         }
//         let extended_ptr = read.read_i64::<BigEndian>()?;
//         Ok(Self {
//             material_key,
//             extended_ptr,
//         })
//     }

//     fn write(&self, write: &mut impl Write) -> Result<()> {
//         write.write_i32::<BigEndian>(self.material_key.len() as _)?;
//         write.write_all(self.material_key.as_bytes())?;
//         write.write_i64::<BigEndian>(self.extended_ptr)?;
//         Ok(())
//     }
// }
