pub mod format;
pub mod meta;
pub mod util;

use crate::{util::Serializable, meta::{Meta, MetaEntry}};
use format::{Storage, StorageDevice};
use std::fs::File;

const HEAP_ALIGN: usize = 1024 * 4;

fn main() -> Result<(), std::io::Error> {
    let mut fio = File::create("META.bin")?;
    let mut meta = Meta::default();
    let entry = MetaEntry::new("minecraft:stone".to_string(), 0);
    meta.push_entry(entry)?;
    meta.write(&mut fio)?;
    drop(fio);

    let mut fio = File::open("META.bin")?;
    let meta = Meta::read(&mut fio)?;
    println!("{meta:?}");

    let mut fio = File::create("HEAP.bin")?;
    let mut storage = Storage::default();
    let entry = StorageDevice::new(format::NULL, 0);
    storage.push_entry(entry)?;
    storage.write_aligned(&mut fio, HEAP_ALIGN)?;
    drop(fio);

    let mut fio = File::open("HEAP.bin")?;
    let storage = Storage::read(&mut fio)?;
    println!("{storage:?}");
    Ok(())
}
