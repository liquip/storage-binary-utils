# Specification

Version 1.0.0

## META.bin

```rs
pub struct Meta {
    pub version: i16,
    pub material_list_ptr: i64,
    pub storage_list_ptr: i64,
}
```

The version field should be set to 1 for this specification.

`material_list_ptr` is a pointer to a `MaterialList` on the heap.

`storage_list_ptr` is a pointer to a `StorageList` on the heap.

## HEAP.bin

The heap is aligned to 4096 byte pages.

Indices into the heap are byte-aligned and **allways** 64-bit, signed integers.

If a pointer is -1 it points to nothing and is called NIL (NULL in most programming languages).

### Storage

#### Storage list

```rs
pub struct StorageList {
    pub storages: Vec<i64>,
}
```

`storages` is a list of pointers to all the storages on the heap.

#### Storage

```rs
pub struct Storage {
    pub devices: Vec<StorageDevice>,
}
```

`devices` is the list of `StorageDevice`s the specific storage holds.

#### Storage device

```rs
pub struct StorageDevice {
    pub ptr: i64,
    pub page_type: i8,
}
```

`ptr` is a pointer to a `StoragePage` on the heap holding the items stored on the specific device.

`page_type` is not used yet but exists to specify the type (e.g. size) of the page.

#### Storage page

```rs
pub struct StoragePage {
    pub items: Vec<StorageItem>,
}
```

`items` is the list of items stored on the `StorageDevice`.

#### Storage item

```rs
pub struct StorageItem {
    pub material_index: i32,
    pub count: i64,
    pub meta_ptr: i64,
}
```

`material_index` is an index into the `MaterialList` referenced in the `META.bin` file representing
the type of the item.

`count` is the amount of the item.

`meta_ptr` is a pointer to extended metadata for the item stored on the heap. It is currently not
being used.

> **This specification is still work-in-progress**
