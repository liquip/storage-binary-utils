use crate::util::{read_ptrs, read_slice, verify_ptr, write_ptrs, write_slice, Serializable};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Error, ErrorKind, Read, Result, Write};

#[derive(Debug)]
pub struct StorageList {
    pub storages: Vec<i64>,
}

impl StorageList {
    pub fn new(storages: Vec<i64>) -> Self {
        Self { storages }
    }
}

impl Default for StorageList {
    fn default() -> Self {
        Self {
            storages: Vec::with_capacity(0),
        }
    }
}

impl Serializable for StorageList {
    fn size(&self) -> usize {
        4 + self.storages.len() * 8
    }

    fn read(read: &mut impl Read) -> Result<Self> {
        Ok(Self {
            storages: read_ptrs(read, -1)?,
        })
    }

    fn write(&self, write: &mut impl Write) -> Result<()> {
        write_ptrs(write, &self.storages)
    }
}

#[derive(Debug)]
pub struct Storage {
    pub devices: Vec<StorageDevice>,
}

impl Storage {
    pub fn new(devices: Vec<StorageDevice>) -> Self {
        Self { devices }
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self {
            devices: Vec::with_capacity(0),
        }
    }
}

impl Serializable for Storage {
    fn size(&self) -> usize {
        let mut size = 4;
        for entry in &self.devices {
            size += entry.size();
        }
        size
    }

    fn read(read: &mut impl Read) -> Result<Self> {
        Ok(Self {
            devices: read_slice(read, 454)?,
        })
    }

    fn write(&self, write: &mut impl Write) -> Result<()> {
        if self.devices.len() > 454 {
            return Err(Error::new(ErrorKind::Other, "Maximum length is 454"));
        }
        write_slice(write, &self.devices)
    }
}

#[derive(Debug)]
pub struct StorageDevice {
    pub ptr: i64,
    pub page_type: i8,
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
        Ok(Self {
            ptr: verify_ptr(read.read_i64::<BigEndian>()?)?,
            page_type: read.read_i8()?,
        })
    }

    fn write(&self, write: &mut impl Write) -> Result<()> {
        write.write_i64::<BigEndian>(verify_ptr(self.ptr)?)?;
        write.write_i8(self.page_type)
    }
}

#[derive(Debug)]
pub struct StoragePage {
    pub items: Vec<StorageItem>,
}

impl StoragePage {
    pub fn new(items: Vec<StorageItem>) -> Self {
        Self { items }
    }
}

impl Default for StoragePage {
    fn default() -> Self {
        Self {
            items: Vec::with_capacity(0),
        }
    }
}

impl Serializable for StoragePage {
    fn size(&self) -> usize {
        let mut size = 4;
        for item in &self.items {
            size += item.size();
        }
        size
    }

    fn read(read: &mut impl Read) -> Result<Self> {
        Ok(Self {
            items: read_slice(read, -1)?,
        })
    }

    fn write(&self, write: &mut impl Write) -> Result<()> {
        write_slice(write, &self.items)
    }
}

#[derive(Debug)]
pub struct StorageItem {
    pub material_index: i32,
    pub count: i64,
    pub meta_ptr: i64,
}

impl StorageItem {
    pub fn new(material_index: i32, count: i64, meta_ptr: i64) -> Self {
        Self {
            material_index,
            count,
            meta_ptr,
        }
    }
}

impl Serializable for StorageItem {
    fn size(&self) -> usize {
        4 + 8 + 8
    }

    fn read(read: &mut impl Read) -> Result<Self> {
        let material_index = read.read_i32::<BigEndian>()?;
        if material_index < 0 {
            return Err(Error::new(ErrorKind::Other, "Negative material index"));
        }
        let count = read.read_i64::<BigEndian>()?;
        if count < 1 {
            return Err(Error::new(ErrorKind::Other, "Too small item count"));
        }
        Ok(Self {
            material_index,
            count,
            meta_ptr: verify_ptr(read.read_i64::<BigEndian>()?)?,
        })
    }

    fn write(&self, write: &mut impl Write) -> Result<()> {
        if self.material_index < 0 {
            return Err(Error::new(ErrorKind::Other, "Negative material index"));
        }
        if self.count < 1 {
            return Err(Error::new(ErrorKind::Other, "Too small item count"));
        }
        write.write_i32::<BigEndian>(self.material_index)?;
        write.write_i64::<BigEndian>(self.count)?;
        write.write_i64::<BigEndian>(verify_ptr(self.meta_ptr)?)
    }
}
