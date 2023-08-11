use std::{
    fmt::Debug,
    io::{Read, Seek},
};

use binrw::{BinRead, BinWrite};

/// A string prefixed by a `u16` size.  
/// The endianess is inherited from the parent struct.
#[derive(Clone, BinRead, BinWrite)]
pub struct PascalStringU16 {
    pub length: u16,
    #[br(count = length)]
    pub data: Vec<u8>,
}
impl PascalStringU16 {
    /// Get the content as utf8 without allocating.  
    /// Returns an error if the content is not valid utf8.
    pub fn as_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.data)
    }
}
impl Debug for PascalStringU16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.as_str() {
            Ok(s) => write!(f, "{:?}", s),
            Err(_) => write!(f, "{:?}", self.data),
        }
    }
}

// /// A wrapper around a type to implement `Seek` where it will panic if the functions are called.
// /// This is used because binrw requires `Seek` to be implemented, but we don't necessarily need it.
// pub struct SeekPanicWrapper<R> {
//     inner: R,
// }
// impl<R> SeekPanicWrapper<R> {
//     pub fn new(inner: R) -> Self {
//         Self { inner }
//     }
// }
// impl<R> Seek for SeekPanicWrapper<R> {
//     fn seek(&mut self, _: std::io::SeekFrom) -> std::io::Result<u64> {
//         panic!("Seek::seek called on SeekPanicWrapper")
//     }

//     fn rewind(&mut self) -> std::io::Result<()> {
//         panic!("Seek::rewind called on SeekPanicWrapper")
//     }

//     fn stream_position(&mut self) -> std::io::Result<u64> {
//         panic!("Seek::stream_position called on SeekPanicWrapper")
//     }
// }
// impl<R: Read> Read for SeekPanicWrapper<R> {
//     fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
//         self.inner.read(buf)
//     }

//     fn read_vectored(&mut self, bufs: &mut [std::io::IoSliceMut<'_>]) -> std::io::Result<usize> {
//         self.inner.read_vectored(bufs)
//     }

//     fn read_to_end(&mut self, buf: &mut Vec<u8>) -> std::io::Result<usize> {
//         self.inner.read_to_end(buf)
//     }

//     fn read_to_string(&mut self, buf: &mut String) -> std::io::Result<usize> {
//         self.inner.read_to_string(buf)
//     }

//     fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
//         self.inner.read_exact(buf)
//     }
// }
// impl<R: BufRead> BufRead for SeekPanicWrapper<R> {
//     fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
//         self.inner.fill_buf()
//     }

//     fn consume(&mut self, amt: usize) {
//         self.inner.consume(amt)
//     }
// }

fn is_eof(err: &binrw::Error) -> bool {
    use binrw::Error;

    match err {
        Error::Io(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => true,
        Error::EnumErrors { variant_errors, .. } => {
            let all_eof = variant_errors.iter().all(|(_, err)| err.is_eof());
            if all_eof {
                return true;
            }

            if variant_errors.len() == 2 {
                match &variant_errors[0].1 {
                    // For [Pre AssertFail, BackTrace ([EOF error, ...])]
                    Error::AssertFail { .. } => {
                        if let (_, Error::Backtrace(bt)) = &variant_errors[1] {
                            bt.error.is_eof()
                        } else {
                            false
                        }
                    }
                    // For [Backtrace ([EOF error, while parsing field self_0])]
                    Error::Backtrace(bt) => bt.error.is_eof(),
                    _ => {
                        return false;
                    }
                }
            } else {
                false
            }
        }
        Error::Backtrace(bt) => bt.error.is_eof(),
        _ => false,
    }
}

pub(crate) fn until_eof2<R, T, Arg, Ret>(
    r: &mut R,
    e: binrw::Endian,
    args: Arg,
) -> Result<Ret, binrw::Error>
where
    T: for<'a> BinRead<Args<'a> = Arg>,
    R: Read + Seek,
    Arg: Clone,
    Ret: FromIterator<T>,
{
    std::iter::from_fn(|| match T::read_options(r, e, args.clone()) {
        Ok(v) => Some(Ok(v)),
        Err(err) if is_eof(&err) => None,
        Err(err) => Some(Err(err)),
    })
    .fuse()
    .collect()
}
