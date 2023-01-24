pub mod heap;
pub mod meta;
pub mod util;

use crate::{heap::NIL, meta::Meta, util::Serializable};
use heap::{
    material::{Material, MaterialList},
    storage::{Storage, StorageDevice, StorageItem, StorageList, StoragePage},
    Heap,
};
use std::{fs::File, io::Result};

const HEAP_ALIGN: usize = 1024 * 4;
const YLW: &str = "\x1B[33;1m";
const BLUE: &str = "\x1B[34;1m";
const RST: &str = "\x1B[0m";

fn main() -> Result<()> {
    write()?;
    read()
}

fn write() -> Result<()> {
    let mut meta_io = File::create("META.bin")?;
    let mut heap_io = Heap::create_file("HEAP.bin")?;
    let mut meta = Meta::default();

    let mut material_list = MaterialList::default();
    let stone_material = material_list.push(Material::new("minecraft:stone".to_string())) as i32;
    let material_list_ptr = heap_io.alloc_end()? as i64;
    material_list.write_aligned(&mut heap_io.source, HEAP_ALIGN)?;
    meta.material_list_ptr = material_list_ptr;

    let mut storage_page = StoragePage::default();
    storage_page
        .items
        .push(StorageItem::new(stone_material, 64000, NIL));
    let storage_page_ptr = heap_io.alloc_end()? as i64;
    storage_page.write_aligned(&mut heap_io.source, HEAP_ALIGN)?;

    let mut storage = Storage::default();
    let storage_device = StorageDevice::new(storage_page_ptr, 0);
    storage.devices.push(storage_device);
    let storage_ptr = heap_io.alloc_end()? as i64;
    storage.write_aligned(&mut heap_io.source, HEAP_ALIGN)?;

    let mut storage_list = StorageList::default();
    storage_list.storages.push(storage_ptr);
    let storage_list_ptr = heap_io.alloc_end()? as i64;
    storage_list.write_aligned(&mut heap_io.source, HEAP_ALIGN)?;
    meta.storage_list_ptr = storage_list_ptr;

    meta.write(&mut meta_io)?;
    Ok(())
}

fn read() -> Result<()> {
    let mut meta_io = File::open("META.bin")?;
    let mut heap_io = Heap::load_file("HEAP.bin")?;

    let meta = Meta::read(&mut meta_io)?;
    println!("{BLUE}Meta{RST}:");
    println!("{meta:#?}");

    if meta.material_list_ptr == NIL {
        return Ok(());
    }

    println!("{BLUE}Material{RST}:");
    heap_io.seek(meta.material_list_ptr as _)?;
    let material_list = MaterialList::read(&mut heap_io.source)?;
    println!(
        "{YLW}@0x{:04X}{RST}: {material_list:#?}",
        meta.material_list_ptr
    );

    if meta.storage_list_ptr == NIL {
        return Ok(());
    }

    println!("{BLUE}Storage{RST}:");
    heap_io.seek(meta.storage_list_ptr as _)?;
    let storage_list = StorageList::read(&mut heap_io.source)?;
    println!(
        "{YLW}@0x{:04X}{RST}: {storage_list:#?}",
        meta.storage_list_ptr
    );
    println!("{BLUE}Storage-entries{RST}:");
    let mut storage_pages = Vec::new();
    for storage_ptr in storage_list.storages.iter().cloned() {
        if storage_ptr == NIL {
            println!("{YLW}warn{RST}: nil storage pointer");
            continue;
        }
        heap_io.seek(storage_ptr as _)?;
        let storage = Storage::read(&mut heap_io.source)?;
        println!("{YLW}@0x{storage_ptr:04X}{RST}: {storage:#?}");
        for device in &storage.devices {
            if device.ptr != NIL {
                storage_pages.push(device.ptr);
            }
        }
    }
    println!("{BLUE}Storage-pages{RST}:");
    for storage_page_ptr in storage_pages {
        heap_io.seek(storage_page_ptr as _)?;
        let storage_page = StoragePage::read(&mut heap_io.source)?;
        println!("{YLW}@0x{storage_page_ptr:04X}{RST}: {storage_page:#?}");
        for StorageItem {
            material_index,
            count,
            meta_ptr,
        } in &storage_page.items
        {
            println!(
                "{count} of {} with{}",
                material_list.materials[*material_index as usize].key,
                if *meta_ptr == NIL {
                    "out metadata".to_string()
                } else {
                    format!(" metadata at @0x{meta_ptr:04X}")
                }
            );
        }
    }
    Ok(())
}
