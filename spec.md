# Spec V1

## META.bin

```rs
struct Meta {
  version: i16, // = 1
  length: i32,
  entries: Vec<MetaEntry>, // len() = length
}

struct MetaEntry {
  material_key: String,
  extended_ptr: i64,
}
```

## HEAP.bin

The heap is allocated in 4096 byte pages.

Indices into the heap are byte-aligned.

There are different storage pages that reserve different amounts of pages for themselves.

The most basic page will reserve one page and may store up to 341 items (Item types).

### Storage

```rs
struct Storage {
  padding: [i8; 6],
  length: i32,
  entries: Vec<StorageDevice>, // len() = self.length/454
}

struct StorageDevice {
  ptr: i64,
  page_type: i8,
}

struct RegularStoragePage {
  usage: i32,
  items: Vec<StorageItem>, // len() = 341 (only <usage> are filled)
}

struct DoubleStoragePage {
  padding: [i8; 4], // could be used for flags
  usage: i32,
  items: Vec<StorageItem>, // len() = 682 (only <usage> are filled)
}

struct StorageItem {
  type_index: i32,
  extended_ptr: i64,
}
```

### Extended Metadata

> **WIP**

# TODO
* 818 = 4096 / 5 != 4096 / 9
