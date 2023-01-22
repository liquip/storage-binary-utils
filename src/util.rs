use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Error, ErrorKind, Read, Result, Write};

pub trait Serializable: Sized {
    fn len(&self) -> usize;

    fn read(read: &mut impl Read) -> Result<Self>;

    fn write(&self, write: &mut impl Write) -> Result<()>;

    fn write_aligned(&self, write: &mut impl Write, alignment: usize) -> Result<usize> {
        let len = self.len();
        let aligned = align(len, alignment);
        let padding = alignment - len;
        self.write(write)?;
        for _ in 0..padding {
            write.write_i8(-1)?;
        }
        Ok(aligned / alignment)
    }
}

pub fn write_bytes(write: &mut impl Write, bytes: &[u8]) -> Result<()> {
    let len = bytes.len();
    if len > i32::MAX as _ {
        return Err(Error::new(
            ErrorKind::Other,
            "Maximum length of byte array was exceeded",
        ));
    }
    write.write_i32::<BigEndian>(len as _)?;
    write.write(bytes)?;
    Ok(())
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

pub fn write_padding(write: &mut impl Write, len: usize) -> Result<()> {
    for _ in 0..len {
        write.write_i8(-1)?;
    }
    Ok(())
}

pub fn read_string(read: &mut impl Read) -> Result<String> {
    let len = read.read_i32::<BigEndian>()?;
    if len < 0 {
        return Err(Error::new(ErrorKind::Other, "String with negative length"));
    }
    let mut buf = String::with_capacity(len as _);
    read.take(len as _).read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn read_slice<T: Serializable>(read: &mut impl Read, max_length: i32) -> Result<Vec<T>> {
    let len = read.read_i32::<BigEndian>()?;
    if len < 0 {
        return Err(Error::new(ErrorKind::Other, "Slice with negative length"));
    }
    if max_length != -1 && len > max_length {
        return Err(Error::new(ErrorKind::Other, "Maximum length of slice was exceeded"));
    }
    let mut buf = Vec::with_capacity(len as _);
    for _ in 0..len {
        buf.push(T::read(read)?);
    }
    Ok(buf)
}

pub fn read_padding(read: &mut impl Read, len: usize) -> Result<()> {
    for _ in 0..len {
        read.read_i8()?;
    }
    Ok(())
}

pub const fn align(num: usize, alignment: usize) -> usize {
    if num % alignment == 0 {
        num
    } else {
        num - num % alignment + alignment
    }
}
