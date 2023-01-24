use crate::util::{read_slice, read_string, write_bytes, write_slice, Serializable};
use std::io::{Read, Result, Write};

#[derive(Debug)]
pub struct MaterialList {
    pub materials: Vec<Material>,
}

impl MaterialList {
    pub fn new(materials: Vec<Material>) -> Self {
        Self { materials }
    }
}

impl Default for MaterialList {
    fn default() -> Self {
        Self {
            materials: Vec::with_capacity(0),
        }
    }
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

#[derive(Debug)]
pub struct Material {
    pub key: String,
}

impl Material {
    pub fn new(key: String) -> Self {
        Self { key }
    }
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
