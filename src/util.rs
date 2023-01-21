use byteorder::WriteBytesExt;
use std::io::{Read, Result, Write};

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

pub const fn align(num: usize, alignment: usize) -> usize {
    if num % alignment == 0 {
        num
    } else {
        num - num % alignment + alignment
    }
}
