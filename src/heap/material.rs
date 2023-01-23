use crate::util::{read_slice, read_string, write_bytes, write_slice, Serializable};
use std::io::{Read, Result, Write};

pub struct MaterialList {
    materials: Vec<Material>,
}

impl Serializable for MaterialList {
    fn size(&self) -> usize {
        let mut size = 4;
        for material in &self.materials {
            size += material.size();
        }
        size
    }

    fn read(read: &mut impl Read) -> Result<Self> {
        Ok(Self {
            materials: read_slice(read, -1)?,
        })
    }

    fn write(&self, write: &mut impl Write) -> Result<()> {
        write_slice(write, &self.materials)
    }
}

pub struct Material {
    key: String,
}

impl Serializable for Material {
    fn size(&self) -> usize {
        4 + self.key.len()
    }

    fn read(read: &mut impl Read) -> Result<Self> {
        Ok(Self {
            key: read_string(read)?,
        })
    }

    fn write(&self, write: &mut impl Write) -> Result<()> {
        write_bytes(write, self.key.as_bytes())
    }
}
