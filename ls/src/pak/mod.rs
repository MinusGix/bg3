pub mod common;

use std::{
    io::{BufRead, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
};

use binrw::{meta::ReadEndian, BinRead, BinReaderExt};

use crate::pak::common::{FileEntry15, FILE_ENTRY_15_SIZE};

use self::common::{
    FileEntry13, FileEntry18, FileEntry7, FileInfo, LSPKHeader10, LSPKHeader13, LSPKHeader15,
    LSPKHeader16, LSPKHeader7, PackagedFileInfo, PackagedFileInfoError, FILE_ENTRY_13_SIZE,
    FILE_ENTRY_18_SIZE,
};

// TODO: Should we move pak to its own crate?

#[derive(Debug)]
pub enum PackageError {
    Binrw(binrw::Error),
    UnsupportedVersion(i32),
    NotAPackage,
    InvalidSolidArchiveCompression {
        first_offset: u32,
        last_offset: u32,
        total_size_on_disk: u32,
    },
    SolidFileListNotContiguous,
    LZ4DifferentSize {
        expected: usize,
        actual: usize,
    },
    /// Failure in removing base prefix from path
    EnumerateFilesPrefix,
    PackagedFileInfo(PackagedFileInfoError),
    Lz4Decompress(lz4_flex::block::DecompressError),
}
impl From<binrw::Error> for PackageError {
    fn from(e: binrw::Error) -> Self {
        Self::Binrw(e)
    }
}
impl From<std::io::Error> for PackageError {
    fn from(e: std::io::Error) -> Self {
        Self::Binrw(e.into())
    }
}
impl From<PackagedFileInfoError> for PackageError {
    fn from(e: PackagedFileInfoError) -> Self {
        Self::PackagedFileInfo(e)
    }
}
impl From<lz4_flex::block::DecompressError> for PackageError {
    fn from(e: lz4_flex::block::DecompressError) -> Self {
        Self::Lz4Decompress(e)
    }
}

// TODO(minor): We could have it provide a general 'get other file data' trait to allow
// getting the information without a backing file, and then just provide a typical implementation
// but whatever.
pub fn read_package(
    mut data: impl BufRead + Seek,
    path: impl AsRef<Path>,
    metadata_only: bool,
) -> Result<Package, PackageError> {
    let path = path.as_ref();

    // Check for v13 package header at the end of the file
    data.seek(SeekFrom::End(-8))?;
    let header_size: i32 = data.read_le()?;
    let sig: [u8; 4] = BinRead::read(&mut data)?;
    if sig == PACKAGE_MAGIC {
        data.seek(SeekFrom::End(-header_size as i64))?;
        return read_package_v13(data, path, metadata_only);
    }

    // Check for v10 package headers
    data.seek(SeekFrom::Start(0))?;
    let sig: [u8; 4] = BinRead::read(&mut data)?;
    if sig == PACKAGE_MAGIC {
        let version: i32 = data.read_le()?;
        return match version {
            10 => Ok(read_package_v10(data, path, metadata_only)?),
            15 => {
                data.seek(SeekFrom::Start(4))?;
                Ok(read_package_v15(data, path, metadata_only)?)
            }
            16 => {
                data.seek(SeekFrom::Start(4))?;
                Ok(read_package_v16(data, path, metadata_only)?)
            }
            18 => {
                data.seek(SeekFrom::Start(4))?;
                Ok(read_package_v18(data, path, metadata_only)?)
            }
            _ => Err(PackageError::UnsupportedVersion(version)),
        };
    }

    // Check for v9 and v7 package headers
    data.seek(SeekFrom::Start(0))?;
    let version: i32 = data.read_le()?;
    if version == 7 || version == 9 {
        return Ok(read_package_v7(data, path, metadata_only)?);
    }

    Err(PackageError::NotAPackage)
}

pub fn read_package_v7(
    mut data: impl BufRead + Seek,
    path: &Path,
    metadata_only: bool,
) -> Result<Package, PackageError> {
    data.seek(SeekFrom::Start(0))?;

    let header: LSPKHeader7 = BinRead::read(&mut data)?;

    let mut package = Package::new(PackageVersion::V7, path.to_owned());

    if metadata_only {
        return Ok(package);
    }

    for i in 0..header.num_files {
        let mut entry: FileEntry7 = BinRead::read(&mut data)?;
        // We follow how the C# version does this, where it just drops the header information by
        // just adding it to the entry if needed.
        // When writing, we have to undo this transformation obviously.
        if entry.archive_part == 0 {
            entry.offset_in_file += header.data_offset;
        }
        let file_info = PackagedFileInfo::try_from(entry)?;

        package.files.push(file_info.into());
    }

    Ok(package)
}

pub fn read_package_v10(
    mut data: impl BufRead + Seek,
    path: &Path,
    metadata_only: bool,
) -> Result<Package, PackageError> {
    // Skip signature
    data.seek(SeekFrom::Start(4))?;

    let header: LSPKHeader10 = BinRead::read(&mut data)?;

    let mut package = Package::new(PackageVersion::V10, path.to_owned());
    package.metadata.flags = PackageFlags(header.flags.into());
    package.metadata.priority = header.priority;

    if metadata_only {
        return Ok(package);
    }

    for _ in 0..header.num_files {
        let mut entry: FileEntry13 = BinRead::read(&mut data)?;
        if entry.archive_part == 0 {
            entry.offset_in_file += header.data_offset;
        }

        // Add missing compression level flags
        entry.flags = (entry.flags & 0x0F) | 0x20;

        let file_info = PackagedFileInfo::try_from(entry)?;

        package.files.push(file_info.into());
    }

    Ok(package)
}

pub fn read_package_v13(
    mut data: impl BufRead + Seek,
    path: &Path,
    metadata_only: bool,
) -> Result<Package, PackageError> {
    let header: LSPKHeader13 = BinRead::read(&mut data)?;

    if header.version != PackageVersion::V13 as u32 {
        return Err(PackageError::UnsupportedVersion(header.version as i32));
    }

    let mut package = Package::new(PackageVersion::V13, path.to_owned());
    package.metadata.flags = PackageFlags(header.flags.into());
    package.metadata.priority = header.priority;

    if metadata_only {
        return Ok(package);
    }

    data.seek(SeekFrom::Start(header.file_list_offset.into()))?;

    let num_files: u32 = data.read_le()?;
    let file_buffer_size = FILE_ENTRY_13_SIZE * num_files as usize;
    let file_list_size = header.file_list_size.saturating_sub(4);

    let mut compressed_file_list = vec![0; file_list_size as usize];
    data.read_exact(&mut compressed_file_list)?;

    let uncompressed_list = lz4_flex::decompress(&compressed_file_list, file_buffer_size)?;
    if uncompressed_list.len() != file_buffer_size {
        return Err(PackageError::LZ4DifferentSize {
            expected: file_buffer_size,
            actual: uncompressed_list.len(),
        });
    }

    let mut uncompressed_list_cursor = std::io::Cursor::new(uncompressed_list);

    let mut entries = Vec::with_capacity(num_files as usize);
    for _ in 0..num_files {
        let entry: FileEntry13 = BinRead::read(&mut uncompressed_list_cursor)?;
        entries.push(entry);
    }

    // FIXME: We don't currently pass the decompressed/compressed data streams to the classes.
    // It looks like we could wait to compute this until it is actually needed/desired.
    if package.metadata.flags.solid() && num_files > 0 {
        // Calculate compressed frame offsets and bounds
        // let mut total_uncompressed_size = 0;
        let mut total_size_on_disk = 0;
        let mut first_offset = u32::MAX;
        let mut last_offset = 0;

        // TODO: Should we work in u64 to ensure it doesn't overflow?
        for entry in &entries {
            // total_uncompressed_size += entry.uncompressed_size;
            total_size_on_disk += entry.size_on_disk;
            if entry.offset_in_file < first_offset {
                first_offset = entry.offset_in_file;
            }

            if entry.offset_in_file + entry.size_on_disk > last_offset {
                last_offset = entry.offset_in_file + entry.size_on_disk;
            }
        }

        if first_offset != 7 || last_offset - first_offset != total_size_on_disk {
            return Err(PackageError::InvalidSolidArchiveCompression {
                first_offset,
                last_offset,
                total_size_on_disk,
            });
        }

        // Decompress all files as a single frame (solid)
        let mut frame = vec![0; last_offset as usize];

        data.seek(SeekFrom::Start(0))?;
        data.read_exact(&mut frame)?;

        let frame_cursor = std::io::Cursor::new(frame);

        let mut wtr = lz4_flex::frame::FrameDecoder::new(frame_cursor);
        let mut decompressed = Vec::new();
        wtr.read_to_end(&mut decompressed)?;

        let decompressed_cursor = std::io::Cursor::new(decompressed);

        // Update offsets to point to the decompressed chunk
        let mut offset = 7;
        let mut compressed_offset = 0;
        for entry in entries {
            if entry.offset_in_file != offset {
                return Err(PackageError::SolidFileListNotContiguous);
            }

            let size_on_disk = entry.size_on_disk;
            let uncompressed_size = entry.uncompressed_size;

            let file = PackagedFileInfo::solid_from_entry13(entry, compressed_offset)?;

            package.files.push(file.into());

            offset += size_on_disk;
            compressed_offset += uncompressed_size;
        }
    } else {
        let files: Result<Vec<_>, _> = entries
            .into_iter()
            .map(PackagedFileInfo::try_from)
            .map(|v| v.map(|v| v.into()))
            .collect();
        package.files = files?;
    }

    Ok(package)
}

fn read_file_list_v15(
    data: impl BufRead + Seek,
    package: &mut Package,
) -> Result<(), PackageError> {
    read_file_list_v15_or_v18::<FILE_ENTRY_15_SIZE, FileEntry15>(data, package)
}

fn read_file_list_v18(
    data: impl BufRead + Seek,
    package: &mut Package,
) -> Result<(), PackageError> {
    read_file_list_v15_or_v18::<FILE_ENTRY_18_SIZE, FileEntry18>(data, package)
}

fn read_file_list_v15_or_v18<const SIZE: usize, T>(
    mut data: impl BufRead + Seek,
    package: &mut Package,
) -> Result<(), PackageError>
where
    T: ReadEndian + for<'a> BinRead<Args<'a> = ()>,
    PackagedFileInfo: TryFrom<T, Error = PackagedFileInfoError>,
{
    let num_files: u32 = data.read_le()?;
    let compressed_size: u32 = data.read_le()?;

    let mut compressed_file_list = vec![0; compressed_size as usize];
    data.read_exact(&mut compressed_file_list)?;

    let file_buffer_size = SIZE * num_files as usize;

    let uncompressed_list = lz4_flex::decompress(&compressed_file_list, file_buffer_size)?;
    if uncompressed_list.len() != file_buffer_size {
        return Err(PackageError::LZ4DifferentSize {
            expected: file_buffer_size,
            actual: uncompressed_list.len(),
        });
    }

    let mut uncompressed_list_cursor = std::io::Cursor::new(uncompressed_list);

    package.files.reserve(num_files as usize);
    for _ in 0..num_files {
        let entry: T = BinRead::read(&mut uncompressed_list_cursor)?;
        let file = PackagedFileInfo::try_from(entry)?;
        package.files.push(file.into());
    }

    Ok(())
}

fn read_package_v15(
    mut data: impl BufRead + Seek,
    path: &Path,
    metadata_only: bool,
) -> Result<Package, PackageError> {
    let header: LSPKHeader15 = BinRead::read(&mut data)?;

    if header.version != PackageVersion::V15 as u32 {
        return Err(PackageError::UnsupportedVersion(header.version as i32));
    }

    let mut package = Package::new(PackageVersion::V15, path.to_owned());
    package.metadata.flags = PackageFlags(header.flags.into());
    package.metadata.priority = header.priority;

    if metadata_only {
        return Ok(package);
    }

    data.seek(SeekFrom::Start(header.file_list_offset.into()))?;

    read_file_list_v15(data, &mut package)?;

    Ok(package)
}

fn read_package_v16(
    mut data: impl BufRead + Seek,
    path: &Path,
    metadata_only: bool,
) -> Result<Package, PackageError> {
    let header: LSPKHeader16 = BinRead::read(&mut data)?;

    if header.version != PackageVersion::V16 as u32 {
        return Err(PackageError::UnsupportedVersion(header.version as i32));
    }

    let mut package = Package::new(PackageVersion::V16, path.to_owned());
    package.metadata.flags = PackageFlags(header.flags.into());
    package.metadata.priority = header.priority;

    if metadata_only {
        return Ok(package);
    }

    data.seek(SeekFrom::Start(header.file_list_offset.into()))?;

    read_file_list_v15(data, &mut package)?;

    Ok(package)
}

fn read_package_v18(
    mut data: impl BufRead + Seek,
    path: &Path,
    metadata_only: bool,
) -> Result<Package, PackageError> {
    let header: LSPKHeader16 = BinRead::read(&mut data)?;

    if header.version != PackageVersion::V18 as u32 {
        return Err(PackageError::UnsupportedVersion(header.version as i32));
    }

    let mut package = Package::new(PackageVersion::V18, path.to_owned());
    package.metadata.flags = PackageFlags(header.flags.into());
    package.metadata.priority = header.priority;

    if metadata_only {
        return Ok(package);
    }

    data.seek(SeekFrom::Start(header.file_list_offset.into()))?;

    read_file_list_v18(data, &mut package)?;

    Ok(package)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum PackageVersion {
    /// D:OS 1
    V7 = 7,
    /// D:OS 1 EE
    V9 = 9,
    /// D:OS 2
    V10 = 10,
    /// D:OS 2 DE
    V13 = 13,
    /// BG3 EA
    V15 = 15,
    /// BG3 EA Patch4
    V16 = 16,
    /// BG3 Release
    V18 = 18,
}

// TODO: should we just use the bitflags crate?
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PackageFlags(pub u32);
impl PackageFlags {
    pub fn allow_memory_mapping(self) -> bool {
        self.0 & 0x02 != 0
    }

    pub fn solid(self) -> bool {
        self.0 & 0x04 != 0
    }

    pub fn preload(self) -> bool {
        self.0 & 0x08 != 0
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PackageMetadata {
    /// PackageFlags
    pub flags: PackageFlags,
    /// Load priority. Packages with higher priority are loaded later and override earlier packages.
    pub priority: u8,
}

pub const PACKAGE_MAGIC: [u8; 4] = [0x4C, 0x53, 0x50, 0x4B];

#[derive(Debug, Clone, PartialEq)]
pub struct Package {
    pub metadata: PackageMetadata,
    pub files: Vec<FileInfo>,
    pub version: PackageVersion,
    pub path: PathBuf,
}
impl Package {
    pub fn new(version: PackageVersion, path: PathBuf) -> Self {
        Self {
            metadata: PackageMetadata::default(),
            files: Vec::new(),
            version,
            path,
        }
    }

    pub fn make_part_filename(&self, part: u32) -> PathBuf {
        make_part_filename(&self.path, part)
    }
}

fn make_part_filename(path: &Path, part: u32) -> PathBuf {
    // TODO: don't unwrap
    let dir = path.parent().unwrap();
    let base_name = dir.file_stem().unwrap();
    let extension = dir.extension().unwrap();

    let mut full_name = base_name.to_os_string();
    full_name.push("_");
    full_name.push(part.to_string());
    full_name.push(".");
    full_name.push(extension);

    dir.join(full_name)
}
