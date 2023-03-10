use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Error, ErrorKind, Read, Result, Write};

pub trait Serializable: Sized {
    fn size(&self) -> usize;

    fn read(read: &mut impl Read) -> Result<Self>;

    fn write(&self, write: &mut impl Write) -> Result<()>;

    fn write_aligned(&self, write: &mut impl Write, alignment: usize) -> Result<usize> {
        let size = self.size();
        let aligned = align(size, alignment);
        let padding = alignment - size;
        self.write(write)?;
        for _ in 0..padding {
            write.write_i8(-1)?;
        }
        Ok(aligned / alignment)
    }
}

pub fn verify_ptr(ptr: i64) -> Result<i64> {
    if ptr != crate::heap::NIL && ptr < 0 {
        return Err(Error::new(ErrorKind::Other, "Negative pointer"));
    }
    Ok(ptr)
}

pub fn write_bytes(write: &mut impl Write, bytes: &[u8]) -> Result<()> {
    let len = bytes.len();
    if len > i32::MAX as _ {
        return Err(Error::new(
            ErrorKind::Other,
            "Maximum length of slice was exceeded",
        ));
    }
    write.write_i32::<BigEndian>(len as _)?;
    write.write_all(bytes)
}

pub fn write_slice<T: Serializable>(write: &mut impl Write, slice: &[T]) -> Result<()> {
    let len = slice.len();
    if len > i32::MAX as _ {
        return Err(Error::new(
            ErrorKind::Other,
            "Maximum length of slice was exceeded",
        ));
    }
    write.write_i32::<BigEndian>(len as _)?;
    for element in slice {
        element.write(write)?;
    }
    Ok(())
}

pub fn write_ptrs(write: &mut impl Write, ptrs: &[i64]) -> Result<()> {
    let len = ptrs.len();
    if len > i32::MAX as _ {
        return Err(Error::new(
            ErrorKind::Other,
            "Maximum length of slice was exceeded",
        ));
    }
    write.write_i32::<BigEndian>(len as _)?;
    for ptr in ptrs.iter().cloned() {
        write.write_i64::<BigEndian>(verify_ptr(ptr)?)?;
    }
    Ok(())
}

pub fn read_string(read: &mut impl Read) -> Result<String> {
    let size = read.read_i32::<BigEndian>()?;
    if size < 0 {
        return Err(Error::new(ErrorKind::Other, "String with negative length"));
    }
    let mut buf = String::with_capacity(size as _);
    read.take(size as _).read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn read_slice<T: Serializable>(read: &mut impl Read, max_length: i32) -> Result<Vec<T>> {
    let len = read.read_i32::<BigEndian>()?;
    if len < 0 {
        return Err(Error::new(ErrorKind::Other, "Slice with negative length"));
    }
    if max_length != -1 && len > max_length {
        return Err(Error::new(
            ErrorKind::Other,
            "Maximum length of slice was exceeded",
        ));
    }
    let mut buf = Vec::with_capacity(len as _);
    for _ in 0..len {
        buf.push(T::read(read)?);
    }
    Ok(buf)
}

pub fn read_ptrs(read: &mut impl Read, max_length: i32) -> Result<Vec<i64>> {
    let len = read.read_i32::<BigEndian>()?;
    if len < 0 {
        return Err(Error::new(ErrorKind::Other, "Slice with negative length"));
    }
    if max_length != -1 && len > max_length {
        return Err(Error::new(
            ErrorKind::Other,
            "Maximum length of slice was exceeded",
        ));
    }
    let mut buf = Vec::with_capacity(len as _);
    for _ in 0..len {
        buf.push(verify_ptr(read.read_i64::<BigEndian>()?)?);
    }
    Ok(buf)
}

pub const fn align(num: usize, alignment: usize) -> usize {
    if num % alignment == 0 {
        num
    } else {
        num - num % alignment + alignment
    }
}
