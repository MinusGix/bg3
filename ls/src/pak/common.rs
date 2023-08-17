use std::{ops::Not, path::PathBuf};

use binrw::{BinRead, BinWrite};
use lsf::{CompressionFlags, CompressionLevel, CompressionMethod};

fn name_len(name: &[u8; 256]) -> usize {
    name.iter().position(|&c| c == 0).unwrap_or(256)
}

// TODO(minor): We could avoid allocating a string, but its probably minor relative to fs calls? Though there might be a lot of files in a pak..
fn name_to_string(name: &[u8; 256]) -> String {
    let name_len = name_len(name);
    String::from_utf8_lossy(&name[..name_len]).to_string()
}

fn name_to_bytes(name: &str) -> [u8; 256] {
    let name = name.as_bytes();
    let mut name_buf = [0u8; 256];
    name_buf[..name.len()].copy_from_slice(name);

    // Standard the path slashes
    for c in &mut name_buf[..name.len()] {
        if *c == b'\\' {
            *c = b'/';
        }
    }

    name_buf
}

// TODO: manual more informative debug impl
#[derive(Debug)]
pub enum PackagedFileInfoError {
    InvalidCompressionFlags(String),
}

/// Only used for v13, v15, and v16 CreateFromEntrys.
fn check_compression_flags(name: &str, flags: u32) -> Result<(), PackagedFileInfoError> {
    let compression_method = flags & 0x0F;
    if compression_method > 2 || (flags & 0x7fu32.not() != 0) {
        Err(PackagedFileInfoError::InvalidCompressionFlags(
            name.to_string(),
        ))
    } else {
        Ok(())
    }
}

pub trait FileInfoLike: Into<FileInfo> {
    fn name(&self) -> &str;

    fn size(&self) -> u64;

    fn crc(&self) -> u32;

    fn is_deletion(&self) -> bool;
}

#[derive(Debug)]
pub enum PackagedFileContentError {
    Io(std::io::Error),
    /// The file is a deleted file.
    IsDeleted,
}

pub const DELETION_OFFSET: u64 = 0xdeadbeefdeadbeef;

#[derive(Debug, Clone, PartialEq)]
pub struct PackagedFileInfo {
    pub name: String,

    pub archive_part: u32,
    pub crc: u32,
    pub flags: u32,
    pub offset_in_file: u64,
    // TODO: ?
    // pub package_stream
    pub size_on_disk: u64,
    pub uncompressed_size: u64,
    pub solid: bool,
    pub solid_offset: u32,
    // TODO: ?
    // pub solid_stream
    // pub uncompressed_stream
}
impl PackagedFileInfo {
    pub fn from_entry13(entry: FileEntry13) -> Result<PackagedFileInfo, PackagedFileInfoError> {
        let name = name_to_string(&entry.name);
        check_compression_flags(&name, entry.flags)?;

        Ok(PackagedFileInfo {
            name,
            offset_in_file: entry.offset_in_file.into(),
            size_on_disk: entry.size_on_disk.into(),
            uncompressed_size: entry.uncompressed_size.into(),
            archive_part: entry.archive_part,
            flags: entry.flags,
            crc: entry.crc,
            solid: false,
            solid_offset: 0,
        })
    }

    pub fn from_entry15(entry: FileEntry15) -> Result<PackagedFileInfo, PackagedFileInfoError> {
        let name = name_to_string(&entry.name);
        check_compression_flags(&name, entry.flags)?;

        Ok(PackagedFileInfo {
            name,
            offset_in_file: entry.offset_in_file.into(),
            size_on_disk: entry.size_on_disk.into(),
            uncompressed_size: entry.uncompressed_size.into(),
            archive_part: entry.archive_part,
            flags: entry.flags,
            crc: entry.crc,
            solid: false,
            solid_offset: 0,
        })
    }

    pub fn from_entry18(entry: FileEntry18) -> Result<PackagedFileInfo, PackagedFileInfoError> {
        let name = name_to_string(&entry.name);
        check_compression_flags(&name, entry.flags.into())?;

        Ok(PackagedFileInfo {
            name,
            offset_in_file: (entry.offset_in_file1 as u64)
                | ((entry.offset_in_file2 as u64) << 32u64),
            size_on_disk: entry.size_on_disk.into(),
            uncompressed_size: entry.uncompressed_size.into(),
            archive_part: entry.archive_part.into(),
            flags: entry.flags.into(),
            crc: 0,
            solid: false,
            solid_offset: 0,
        })
    }

    pub fn solid_from_entry13(
        entry: FileEntry13,
        solid_offset: u32,
    ) -> Result<PackagedFileInfo, PackagedFileInfoError> {
        let mut info = Self::from_entry13(entry)?;
        info.solid = true;
        info.solid_offset = solid_offset;
        Ok(info)
    }

    pub fn from_entry7(entry: FileEntry7) -> PackagedFileInfo {
        let flags = if entry.uncompressed_size > 0 {
            CompressionFlags::new(CompressionMethod::Zlib, CompressionLevel::DefaultCompress).0
        } else {
            0
        };

        PackagedFileInfo {
            name: name_to_string(&entry.name),
            offset_in_file: entry.offset_in_file.into(),
            size_on_disk: entry.size_on_disk.into(),
            uncompressed_size: entry.uncompressed_size.into(),
            archive_part: entry.archive_part,
            flags: flags.into(),
            crc: 0,
            solid: false,
            solid_offset: 0,
        }
    }

    /// Convert this file info to a v7 file entry.  
    /// May truncate various fields.  
    fn to_entry7(&self) -> FileEntry7 {
        FileEntry7 {
            name: name_to_bytes(&self.name),
            // Potentially truncates.
            offset_in_file: self.offset_in_file as u32,
            size_on_disk: self.size_on_disk as u32,
            uncompressed_size: if (self.flags & 0x0F) == 0 {
                0
            } else {
                self.uncompressed_size as u32
            },
            archive_part: self.archive_part,
        }
    }

    /// Convert this file info to a v13 file entry.
    /// May truncate various fields.
    fn to_entry13(&self) -> FileEntry13 {
        FileEntry13 {
            name: name_to_bytes(&self.name),
            offset_in_file: self.offset_in_file as u32,
            size_on_disk: self.size_on_disk as u32,
            uncompressed_size: if (self.flags & 0x0F) == 0 {
                0
            } else {
                self.uncompressed_size as u32
            },
            archive_part: self.archive_part,
            flags: self.flags,
            crc: self.crc,
        }
    }

    /// Convert this file info to a v15 file entry.
    fn to_entry15(&self) -> FileEntry15 {
        FileEntry15 {
            name: name_to_bytes(&self.name),
            offset_in_file: self.offset_in_file,
            size_on_disk: self.size_on_disk,
            uncompressed_size: if (self.flags & 0x0F) == 0 {
                0
            } else {
                self.uncompressed_size
            },
            archive_part: self.archive_part,
            flags: self.flags,
            crc: self.crc,
            unk2: 0,
        }
    }

    /// Convert this file info to a v18 file entry.
    /// May truncate various fields.
    fn to_entry18(&self) -> FileEntry18 {
        FileEntry18 {
            name: name_to_bytes(&self.name),
            offset_in_file1: (self.offset_in_file & 0xffffffff) as u32,
            offset_in_file2: ((self.offset_in_file >> 32) & 0xffff) as u16,
            archive_part: self.archive_part as u8,
            flags: self.flags as u8,
            size_on_disk: self.size_on_disk as u32,
            uncompressed_size: if (self.flags & 0x0F) == 0 {
                0
            } else {
                self.uncompressed_size as u32
            },
        }
    }

    pub fn content(&self) -> Result<(), PackagedFileContentError> {
        if self.is_deletion() {
            return Err(PackagedFileContentError::IsDeleted);
        }

        todo!()
    }
}
impl FileInfoLike for PackagedFileInfo {
    fn name(&self) -> &str {
        &self.name
    }

    fn is_deletion(&self) -> bool {
        self.offset_in_file == DELETION_OFFSET
    }

    fn size(&self) -> u64 {
        if (self.flags & 0x0f) == 0 {
            self.size_on_disk
        } else {
            self.uncompressed_size
        }
    }

    fn crc(&self) -> u32 {
        self.crc
    }
}

impl TryFrom<FileEntry7> for PackagedFileInfo {
    type Error = PackagedFileInfoError;

    fn try_from(entry: FileEntry7) -> Result<Self, Self::Error> {
        Ok(Self::from_entry7(entry))
    }
}
impl TryFrom<FileEntry13> for PackagedFileInfo {
    type Error = PackagedFileInfoError;

    fn try_from(entry: FileEntry13) -> Result<Self, Self::Error> {
        Self::from_entry13(entry)
    }
}
impl TryFrom<FileEntry15> for PackagedFileInfo {
    type Error = PackagedFileInfoError;

    fn try_from(entry: FileEntry15) -> Result<Self, Self::Error> {
        Self::from_entry15(entry)
    }
}
impl TryFrom<FileEntry18> for PackagedFileInfo {
    type Error = PackagedFileInfoError;

    fn try_from(entry: FileEntry18) -> Result<Self, Self::Error> {
        Self::from_entry18(entry)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FilesystemFileInfo {
    pub path: PathBuf,
    pub name: String,
}
impl FilesystemFileInfo {
    pub fn new(path: PathBuf, name: String) -> FilesystemFileInfo {
        FilesystemFileInfo { path, name }
    }
}
impl FileInfoLike for FilesystemFileInfo {
    fn name(&self) -> &str {
        &self.name
    }

    fn size(&self) -> u64 {
        // TODO: cache this
        self.path.metadata().unwrap().len()
    }

    fn crc(&self) -> u32 {
        panic!("Cannot get crc of filesystem file")
    }

    fn is_deletion(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileInfo {
    Packaged(PackagedFileInfo),
    Filesystem(FilesystemFileInfo),
}
impl From<PackagedFileInfo> for FileInfo {
    fn from(info: PackagedFileInfo) -> Self {
        FileInfo::Packaged(info)
    }
}
impl From<FilesystemFileInfo> for FileInfo {
    fn from(info: FilesystemFileInfo) -> Self {
        FileInfo::Filesystem(info)
    }
}
impl FileInfoLike for FileInfo {
    fn name(&self) -> &str {
        match self {
            FileInfo::Packaged(info) => info.name(),
            FileInfo::Filesystem(info) => info.name(),
        }
    }

    fn size(&self) -> u64 {
        match self {
            FileInfo::Packaged(info) => info.size(),
            FileInfo::Filesystem(info) => info.size(),
        }
    }

    fn crc(&self) -> u32 {
        match self {
            FileInfo::Packaged(info) => info.crc(),
            FileInfo::Filesystem(info) => info.crc(),
        }
    }

    fn is_deletion(&self) -> bool {
        match self {
            FileInfo::Packaged(info) => info.is_deletion(),
            FileInfo::Filesystem(_) => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, BinRead, BinWrite)]
#[br(little)]
pub struct LSPKHeader7 {
    pub version: u32,
    pub data_offset: u32,
    pub num_parts: u32,
    pub file_list_size: u32,
    pub little_endian: u8,
    pub num_files: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, BinRead, BinWrite)]
#[br(little)]
pub struct FileEntry7 {
    pub name: [u8; 256],
    pub offset_in_file: u32,
    pub size_on_disk: u32,
    pub uncompressed_size: u32,
    pub archive_part: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, BinRead, BinWrite)]
#[br(little)]
pub struct LSPKHeader10 {
    pub version: u32,
    pub data_offset: u32,
    pub file_list_size: u32,
    pub num_parts: u16,
    pub flags: u8,
    pub priority: u8,
    pub num_files: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, BinRead, BinWrite)]
#[br(little)]
pub struct LSPKHeader13 {
    pub version: u32,
    pub file_list_offset: u32,
    pub file_list_size: u32,
    pub num_parts: u16,
    pub flags: u8,
    pub priority: u8,

    pub md5: [u8; 16],
}

#[derive(Debug, Clone, PartialEq, Eq, BinRead, BinWrite)]
#[br(little)]
pub struct LSPKHeader15 {
    pub version: u32,
    pub file_list_offset: u64,
    pub file_list_size: u32,
    pub flags: u8,
    pub priority: u8,

    pub md5: [u8; 16],
}

#[derive(Debug, Clone, PartialEq, Eq, BinRead, BinWrite)]
#[br(little)]
pub struct LSPKHeader16 {
    pub version: u32,
    pub file_list_offset: u64,
    pub file_lsit_size: u32,
    pub flags: u8,
    pub priority: u8,

    pub md5: [u8; 16],

    pub num_parts: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, BinRead, BinWrite)]
#[br(little)]
pub struct FileEntry13 {
    pub name: [u8; 256],
    pub offset_in_file: u32,
    pub size_on_disk: u32,
    pub uncompressed_size: u32,
    pub archive_part: u32,
    pub flags: u32,
    pub crc: u32,
}
pub const FILE_ENTRY_13_SIZE: usize = 256 + (4 * 6);

#[derive(Debug, Clone, PartialEq, Eq, BinRead, BinWrite)]
#[br(little)]
pub struct FileEntry15 {
    pub name: [u8; 256],

    pub offset_in_file: u64,
    pub size_on_disk: u64,
    pub uncompressed_size: u64,
    pub archive_part: u32,
    pub flags: u32,
    pub crc: u32,
    pub unk2: u32,
}
pub const FILE_ENTRY_15_SIZE: usize = 256 + (8 * 3) + (4 * 4);

#[derive(Debug, Clone, PartialEq, Eq, BinRead, BinWrite)]
#[br(little)]
pub struct FileEntry18 {
    pub name: [u8; 256],

    pub offset_in_file1: u32,
    pub offset_in_file2: u16,
    pub archive_part: u8,
    pub flags: u8,
    pub size_on_disk: u32,
    pub uncompressed_size: u32,
}
pub const FILE_ENTRY_18_SIZE: usize = 256 + 4 + 2 + 2 + 4 + 4;
