use std::io::{BufReader, Cursor, Read};

use flate2::bufread::ZlibDecoder;

use crate::CompressionMethod;

pub enum DecompressedStream<R: Read> {
    Direct {
        inner: std::io::Take<R>,
    },
    Zlib {
        inner: ZlibDecoder<BufReader<R>>,
    },
    LZ4Frame {
        inner: lz4_flex::frame::FrameDecoder<R>,
    },
    LZ4Block {
        // TODO: is there a way to read it iteratively?
        inner: Cursor<Vec<u8>>,
    },
}
impl<R: Read> Read for DecompressedStream<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            DecompressedStream::Direct { inner } => inner.read(buf),
            DecompressedStream::Zlib { inner } => inner.read(buf),
            DecompressedStream::LZ4Frame { inner } => inner.read(buf),
            DecompressedStream::LZ4Block { inner } => inner.read(buf),
        }
    }

    fn read_vectored(&mut self, bufs: &mut [std::io::IoSliceMut<'_>]) -> std::io::Result<usize> {
        match self {
            DecompressedStream::Direct { inner } => inner.read_vectored(bufs),
            DecompressedStream::Zlib { inner } => inner.read_vectored(bufs),
            DecompressedStream::LZ4Frame { inner } => inner.read_vectored(bufs),
            DecompressedStream::LZ4Block { inner } => inner.read_vectored(bufs),
        }
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        match self {
            DecompressedStream::Direct { inner } => inner.read_to_end(buf),
            DecompressedStream::Zlib { inner } => inner.read_to_end(buf),
            DecompressedStream::LZ4Frame { inner } => inner.read_to_end(buf),
            DecompressedStream::LZ4Block { inner } => inner.read_to_end(buf),
        }
    }

    fn read_to_string(&mut self, buf: &mut String) -> std::io::Result<usize> {
        match self {
            DecompressedStream::Direct { inner } => inner.read_to_string(buf),
            DecompressedStream::Zlib { inner } => inner.read_to_string(buf),
            DecompressedStream::LZ4Frame { inner } => inner.read_to_string(buf),
            DecompressedStream::LZ4Block { inner } => inner.read_to_string(buf),
        }
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        match self {
            DecompressedStream::Direct { inner } => inner.read_exact(buf),
            DecompressedStream::Zlib { inner } => inner.read_exact(buf),
            DecompressedStream::LZ4Frame { inner } => inner.read_exact(buf),
            DecompressedStream::LZ4Block { inner } => inner.read_exact(buf),
        }
    }
}

pub fn decompress_into<R: Read>(
    mut compressed: R,
    compressed_size: u32,
    uncompressed_size: u32,
    method: CompressionMethod,
    chunked: bool,
) -> Result<DecompressedStream<R>, binrw::Error> {
    Ok(match method {
        CompressionMethod::None => DecompressedStream::Direct {
            inner: compressed.take(uncompressed_size as u64),
        },
        CompressionMethod::Zlib => {
            // TODO: can we go more directly to the type?
            let v = BufReader::new(compressed);
            let d = ZlibDecoder::new(v);

            DecompressedStream::Zlib { inner: d }
        }
        CompressionMethod::LZ4 => {
            if chunked {
                let wtr = lz4_flex::frame::FrameDecoder::new(compressed);

                DecompressedStream::LZ4Frame { inner: wtr }
            } else {
                let mut data = vec![0; compressed_size as usize];
                compressed.read_exact(&mut data).unwrap();

                let data =
                    lz4_flex::decompress(&data, uncompressed_size as usize).map_err(|e| {
                        binrw::Error::Custom {
                            pos: 0,
                            err: Box::new(e),
                        }
                    })?;

                let dc = Cursor::new(data);
                DecompressedStream::LZ4Block { inner: dc }
            }
        }
    })
}
