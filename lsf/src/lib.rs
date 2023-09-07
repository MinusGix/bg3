//! Note: this library is based on the C# implementation by Norbyte at https://github.com/Norbyte/lslib under the MIT license.

pub mod attr;
pub mod decompress;
pub mod lsx;
pub mod util;

use std::{
    fmt::{Debug, Formatter},
    io::{Read, Seek},
};

use attr::TypeId;
use binrw::{io::TakeSeekExt, meta::ReadEndian, BinRead, BinWrite};
use util::{until_eof2, PascalStringU16};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, BinRead)]
#[repr(u32)]
#[br(repr = u32)]
pub enum LSFVersion {
    /// Initial version of the LSF format
    Initial = 1,
    /// LSF version that added chunked compression for substreams
    ChunkedCompress = 2,
    //// LSF version that extended the node descriptors
    ExtendedNodes = 3,
    /// BG3 version, no changes found so far apart from version numbering
    BG3 = 4,
    /// The version where the extended header was added
    BG3ExtendedHeader = 5,
    /// BG3 version with unknown additions
    BG3AdditionalBlob = 6,
}

/// Latest input version supported by this library
pub const MAX_READ_VERSION: LSFVersion = LSFVersion::BG3AdditionalBlob;
/// Latest output version supported by this library
pub const MAX_WRITE_VERSION: LSFVersion = LSFVersion::BG3AdditionalBlob;

/// Get the essential info about the LSF file. This avoids parsing the entire file.
pub fn parse_lsf_base(input: &[u8]) -> Result<LSFBase, binrw::Error> {
    let mut cursor = std::io::Cursor::new(input);
    LSFBase::read_le(&mut cursor)
}

pub fn parse_lsf(input: &[u8]) -> Result<LSF, binrw::Error> {
    let mut cursor = std::io::Cursor::new(input);
    LSF::read_le(&mut cursor)
}

/// The basic information about the LSOF file format.  
/// This skips the more expensive reading of the names/nodes/attributes
#[derive(Debug, Clone, BinRead)]
#[br(little, magic = b"LSOF")]
// Avoid trying to process files we don't understand.
#[br(assert(version >= LSFVersion::Initial && version <= MAX_READ_VERSION, "LSF version {:?} is not supported", version))]
pub struct LSFBase {
    /// Version of the LSOF file.  
    /// D:OS EE is version 1/2.  
    /// D:OS 2 is version 3.
    pub version: LSFVersion,
    #[br(args(version))]
    pub header: Header,
    #[br(args(version))]
    pub metadata: Metadata,
}

#[derive(Debug, Clone, BinRead)]
pub struct LSF {
    #[br(dbg)]
    pub base: LSFBase,
    #[br(args(base.version, base.metadata.strings_size_on_disk, base.metadata.strings_uncompressed_size, base.metadata.compression_flags))]
    pub names: Names,
    #[br(args(base.version, base.metadata.nodes_size_on_disk, base.metadata.nodes_uncompressed_size, base.metadata.compression_flags, base.metadata.has_sibling_data))]
    pub nodes: Nodes,
    #[br(args(base.version, base.metadata.attributes_size_on_disk, base.metadata.attributes_uncompressed_size, base.metadata.compression_flags, base.metadata.has_sibling_data))]
    pub attributes: Attributes,
}
impl LSF {
    // TODO: name_offset makes me wonder whether it is actually an offset or just another index?
    pub fn name(&self, name_index: u32, name_offset: u32) -> Option<&PascalStringU16> {
        self.names
            .names
            .hash_entries
            .get(name_index as usize)?
            .strings
            .get(name_offset as usize)
    }
}

#[derive(Debug, Clone, BinRead)]
#[br(little, import (version: LSFVersion))]
pub enum Header {
    #[br(pre_assert(version < LSFVersion::BG3ExtendedHeader))]
    V0(HeaderV0),
    #[br(pre_assert(version >= LSFVersion::BG3ExtendedHeader))]
    V5(HeaderV5),
}
impl Header {
    /// Get the engine version from the header.  
    /// This upcasts the version to an i64.
    pub fn engine_version(&self) -> i64 {
        match self {
            Self::V0(h) => h.engine_version.into(),
            // TODO: lslib sets the version to (4, 0, 9, 0) if the major version is 0
            // Should we just store that 0 as the value internally and then return that here?
            // or should we set it like lslib does, which would make round trips not identical.
            Self::V5(h) => h.engine_version,
        }
    }
}

/// Header for V1/V2/V3 (maybe V4?). I just name it V0 since it is up to the next
/// structure, `HeaderV5` which changes.
#[derive(Debug, Clone, BinRead, BinWrite)]
#[br(little)]
pub struct HeaderV0 {
    /// Possibly version number? (major, minor, rev, build)
    pub engine_version: i32,
}

#[derive(Debug, Clone, BinRead, BinWrite)]
#[br(little)]
pub struct HeaderV5 {
    /// Possibly version number? (major, minor, rev, build)
    pub engine_version: i64,
}

#[derive(Debug, Clone, BinRead, BinWrite)]
#[br(little, import (version: LSFVersion))]
pub struct Metadata {
    /// Total uncompressed size of the string hash table
    pub strings_uncompressed_size: u32,
    /// Compressed size of the string hash table
    pub strings_size_on_disk: u32,
    #[br(if(version >= LSFVersion::BG3AdditionalBlob))]
    pub unk: u64,
    /// Total uncompressed size of the node list
    pub nodes_uncompressed_size: u32,
    /// Compressed size of the node list
    pub nodes_size_on_disk: u32,
    /// Total uncompressed size of the attribute list
    pub attributes_uncompressed_size: u32,
    /// Compressed size of the attribute list
    pub attributes_size_on_disk: u32,
    /// Total uncompressed size of the raw value buffer
    pub values_uncompressed_size: u32,
    /// Compressed size of the raw value buffer
    pub values_size_on_disk: u32,
    /// Compression method and level used for the string, node, attribute, and value buffers.  
    /// Uses the same format as packages.
    pub compression_flags: CompressionFlags,
    // I bet that these are padding to align the next field
    /// Possibly unused, always 0
    pub unk2: u8,
    pub unk3: u16,
    /// Extended node/attribute format indicator.  
    /// 0 for V2, 0/1 for V3
    pub has_sibling_data: u32,
}

#[derive(Debug, Clone, Copy, BinRead, BinWrite)]
pub struct CompressionFlags(pub u8);
impl CompressionFlags {
    pub fn new(method: CompressionMethod, level: CompressionLevel) -> CompressionFlags {
        CompressionFlags((method as u8) | (level as u8))
    }

    pub fn method(&self) -> CompressionMethod {
        match self.0 & 0x0f {
            0 => CompressionMethod::None,
            1 => CompressionMethod::Zlib,
            2 => CompressionMethod::LZ4,
            // TODO: do a standard binrw error here
            _ => panic!("Invalid compression method"),
        }
    }

    pub fn level(&self) -> CompressionLevel {
        match self.0 & 0xf0 {
            0x10 => CompressionLevel::FastCompress,
            0x20 => CompressionLevel::DefaultCompress,
            0x40 => CompressionLevel::MaxCompression,
            // TODO: do a standard binrw error here
            _ => panic!("Invalid compression level"),
        }
    }
}

// TODO: feature flags?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CompressionMethod {
    None = 0,
    Zlib = 1,
    LZ4 = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum CompressionLevel {
    FastCompress = 0,
    DefaultCompress = 1,
    MaxCompression = 2,
}

fn binread_compressed<R, T, Arg>(
    r: &mut R,
    version: LSFVersion,
    size_on_disk: u32,
    uncompressed_size: u32,
    compression_flags: CompressionFlags,
    allow_chunked: bool,
    args: Arg,
) -> Result<T, binrw::Error>
where
    R: Read + Seek,
    T: ReadEndian + Default + for<'a> BinRead<Args<'a> = Arg>,
{
    // TODO: other parts of the code use logic like this so deduplicate it
    if size_on_disk == 0 && uncompressed_size != 0 {
        // Data is not compressed
        // TODO: assert that uncompressed size is equivalent to the size parsed?
        let mut r = r.take_seek(uncompressed_size.into());
        T::read_args(&mut r, args)
    } else if size_on_disk == 0 && uncompressed_size == 0 {
        // empty data
        Ok(T::default())
    } else {
        let chunked = version >= LSFVersion::ChunkedCompress && allow_chunked;
        let comp_method = compression_flags.method();
        let is_compressed = comp_method != CompressionMethod::None;
        let compressed_size = if is_compressed {
            size_on_disk
        } else {
            uncompressed_size
        };

        // TODO: we should try just reading directly from the decompressed stream
        // after wrapping it in a seek panic
        let mut decomp = decompress::decompress_into(
            r,
            compressed_size,
            uncompressed_size,
            comp_method,
            chunked,
        )?;

        let mut data = vec![0; uncompressed_size as usize];
        decomp.read_to_end(&mut data)?;

        T::read_args(&mut std::io::Cursor::new(data), args)
    }
}

// TODO: I'd really like not to have a double nesting thing.
// I think I can use binread_compressed via a parse_with?
// Could maybe even use map_stream, but idk if that lets you restore the stream afterwards

// TODO: For the various things like nodes which have two versions, we could instead have two different vecs for the versions since you can only have one type in a file.

#[derive(Debug, Default, Clone)]
pub struct Names {
    pub names: Names2,
}
impl BinRead for Names {
    type Args<'a> = (LSFVersion, u32, u32, CompressionFlags);

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _endian: binrw::Endian,
        (version, size_on_disk, uncompressed_size, compression_flags): Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        binread_compressed::<R, Names2, ()>(
            reader,
            version,
            size_on_disk,
            uncompressed_size,
            compression_flags,
            false,
            (),
        )
        .map(|names| Names { names })
    }
}

#[derive(Debug, Default, Clone, BinRead)]
#[br(little)]
pub struct Names2 {
    pub hash_entry_count: u32,
    #[br(count(hash_entry_count))]
    pub hash_entries: Vec<HashEntry>,
}

#[derive(Clone, BinRead)]
#[br(little)]
pub struct HashEntry {
    pub chain_length: u16,
    // We read these as raw bytes because I'm not certain that it is guaranteed to be utf8.
    #[br(count(chain_length), little)]
    pub strings: Vec<PascalStringU16>,
}
impl Debug for HashEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("HashEntry").field(&self.strings).finish()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Nodes {
    pub nodes: Nodes2,
}
impl BinRead for Nodes {
    type Args<'a> = (LSFVersion, u32, u32, CompressionFlags, u32);

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _endian: binrw::Endian,
        (version, size_on_disk, uncompressed_size, compression_flags, has_sibling_data): Self::Args<
            '_,
        >,
    ) -> binrw::BinResult<Self> {
        binread_compressed::<R, Nodes2, _>(
            reader,
            version,
            size_on_disk,
            uncompressed_size,
            compression_flags,
            true,
            (version, has_sibling_data),
        )
        .map(|nodes| Nodes { nodes })
    }
}

#[derive(Debug, Default, Clone, BinRead)]
#[br(little, import(version : LSFVersion, has_sibling_data : u32))]
pub struct Nodes2 {
    #[br(parse_with = |r, e, _: ()| until_eof2(r, e, (version, has_sibling_data)))]
    pub nodes: Vec<NodeEntry>,
}

/// Node (structure) entry in the LSF file
#[derive(Debug, Clone, BinRead, BinWrite)]
#[br(little, import (version: LSFVersion, has_sibling_data: u32))]
pub enum NodeEntry {
    #[br(pre_assert(version < LSFVersion::ExtendedNodes || has_sibling_data != 1))]
    V2(NodeEntryV2),
    #[br(pre_assert(version >= LSFVersion::ExtendedNodes && has_sibling_data == 1))]
    V3(NodeEntryV3),
}
impl NodeEntry {
    pub fn name_hash_table_index(&self) -> u32 {
        match self {
            Self::V2(n) => n.name_hash_table_index,
            Self::V3(n) => n.name_hash_table_index,
        }
    }

    pub fn parent_index(&self) -> i32 {
        match self {
            Self::V2(n) => n.parent_index,
            Self::V3(n) => n.parent_index,
        }
    }

    pub fn first_attribute_index(&self) -> i32 {
        match self {
            Self::V2(n) => n.first_attribute_index,
            Self::V3(n) => n.first_attribute_index,
        }
    }

    pub fn next_sibling_index(&self) -> Option<i32> {
        match self {
            Self::V2(_) => None,
            Self::V3(n) => Some(n.next_sibling_index),
        }
    }

    /// Index into the name hash table
    pub fn name_index(&self) -> u32 {
        match self {
            Self::V2(n) => n.name_index(),
            Self::V3(n) => n.name_index(),
        }
    }

    /// Offset in hash chain
    pub fn name_offset(&self) -> u32 {
        match self {
            Self::V2(n) => n.name_offset(),
            Self::V3(n) => n.name_offset(),
        }
    }
}

/// Node (structure) entry in the LSF file
#[derive(Clone, BinRead, BinWrite)]
#[br(little)]
pub struct NodeEntryV2 {
    /// Name of this node  
    /// (16-bit MSB: index into name hash table, 16-bit LSB: offset in hash chain)
    pub name_hash_table_index: u32,
    /// Index of the first attribute of this node  
    /// (-1: node has no attributes)
    pub first_attribute_index: i32,
    /// Index of the parent node
    /// (-1: this node is a root region)
    pub parent_index: i32,
}
impl NodeEntryV2 {
    /// Index into the name hash table
    pub fn name_index(&self) -> u32 {
        self.name_hash_table_index >> 16
    }

    /// Offset in hash chain
    pub fn name_offset(&self) -> u32 {
        self.name_hash_table_index & 0xFFFF
    }
}
impl Debug for NodeEntryV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeEntryV2")
            .field("name_index", &self.name_index())
            .field("name_offset", &self.name_offset())
            .field("first_attribute_index", &self.first_attribute_index)
            .field("parent_index", &self.parent_index)
            .finish()
    }
}

/// Node (structure) entry in the LSF file
#[derive(Clone, BinRead, BinWrite)]
#[br(little)]
pub struct NodeEntryV3 {
    /// Name of this node
    /// (16-bit MSB: index into name hash table, 16-bit LSB: offset in hash chain)
    pub name_hash_table_index: u32,
    /// Index of the parent node
    /// (-1: this node is a root region)
    pub parent_index: i32,
    /// Index of the next sibling of this node
    /// (-1: this is the last node)
    pub next_sibling_index: i32,
    /// Index of the first attribute of this node
    /// (-1: node has no attributes)
    pub first_attribute_index: i32,
}
impl NodeEntryV3 {
    /// Index into the name hash table
    pub fn name_index(&self) -> u32 {
        self.name_hash_table_index >> 16
    }

    /// Offset in hash chain
    pub fn name_offset(&self) -> u32 {
        self.name_hash_table_index & 0xFFFF
    }
}
impl Debug for NodeEntryV3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeEntryV3")
            .field("name_index", &self.name_index())
            .field("name_offset", &self.name_offset())
            .field("parent_index", &self.parent_index)
            .field("next_sibling_index", &self.next_sibling_index)
            .field("first_attribute_index", &self.first_attribute_index)
            .finish()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Attributes {
    pub attrs: Attributes2,
}
impl BinRead for Attributes {
    type Args<'a> = (LSFVersion, u32, u32, CompressionFlags, u32);

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _endian: binrw::Endian,
        (version, size_on_disk, uncompressed_size, compression_flags, has_sibling_data): Self::Args<
            '_,
        >,
    ) -> binrw::BinResult<Self> {
        binread_compressed::<R, Attributes2, _>(
            reader,
            version,
            size_on_disk,
            uncompressed_size,
            compression_flags,
            true,
            (version, has_sibling_data),
        )
        .map(|attrs| Attributes { attrs })
    }
}

#[derive(Debug, Clone, Default, BinRead, BinWrite)]
#[br(little, import (version: LSFVersion, has_sibling_data: u32))]
pub struct Attributes2 {
    #[br(parse_with = |r, e, _: ()| until_eof2(r, e, (version, has_sibling_data)))]
    pub attrs: Vec<AttributesEntry>,
}

#[derive(Debug, Clone, BinRead, BinWrite)]
#[br(little, import (version: LSFVersion, has_sibling_data: u32))]
pub enum AttributesEntry {
    #[br(pre_assert(version < LSFVersion::ExtendedNodes || has_sibling_data != 1))]
    V2(AttributeEntryV2),
    #[br(pre_assert(version >= LSFVersion::ExtendedNodes && has_sibling_data == 1))]
    V3(AttributeEntryV3),
}
impl AttributesEntry {
    pub fn name_index(&self) -> u32 {
        match self {
            AttributesEntry::V2(v2) => v2.name_index(),
            AttributesEntry::V3(v3) => v3.name_index(),
        }
    }

    pub fn name_offset(&self) -> u32 {
        match self {
            AttributesEntry::V2(v2) => v2.name_offset(),
            AttributesEntry::V3(v3) => v3.name_offset(),
        }
    }

    pub fn type_id(&self) -> Option<TypeId> {
        match self {
            AttributesEntry::V2(v2) => v2.type_id(),
            AttributesEntry::V3(v3) => v3.type_id(),
        }
    }

    pub fn length(&self) -> u32 {
        match self {
            AttributesEntry::V2(v2) => v2.length(),
            AttributesEntry::V3(v3) => v3.length(),
        }
    }
}

#[derive(Clone, BinRead, BinWrite)]
#[br(little)]
pub struct AttributeEntryV2 {
    /// Name of this attribute
    /// (16-bit MSB: index into name hash table, 16-bit LSB: offset in hash chain)
    pub name_hash_table_index: u32,
    /// 6-bit LSB: Type of this attribute
    /// 26-bit MSB: Length of this attribute
    pub type_and_length: u32,
    /// Index of the node that this attribute belongs to  
    /// Note: these indexes are assigned seemingly arbitrarily, and are not necessarily indices
    /// into the node list.
    pub node_index: i32,
}
impl AttributeEntryV2 {
    /// Index into the name hash table
    pub fn name_index(&self) -> u32 {
        self.name_hash_table_index >> 16
    }

    /// Offset in hash chain
    pub fn name_offset(&self) -> u32 {
        self.name_hash_table_index & 0xFFFF
    }

    /// Type of this attribute
    pub fn type_id(&self) -> Option<TypeId> {
        let typ = (self.type_and_length & 0x3F) as u8;
        TypeId::from_id(typ)
    }

    /// Length of this attribute
    pub fn length(&self) -> u32 {
        self.type_and_length >> 6
    }
}
impl Debug for AttributeEntryV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AttributeEntryV2")
            .field("name_index", &self.name_index())
            .field("name_offset", &self.name_offset())
            .field("type_id", &self.type_id())
            .field("length", &self.length())
            .field("node_index", &self.node_index)
            .finish()
    }
}

#[derive(Clone, BinRead, BinWrite)]
#[br(little)]
pub struct AttributeEntryV3 {
    /// Name of this attribute
    /// (16-bit MSB: index into name hash table, 16-bit LSB: offset in hash chain)
    pub name_hash_table_index: u32,
    /// 6-bit LSB: Type of this attribute
    /// 26-bit MSB: Length of this attribute
    pub type_and_length: u32,
    /// Index of the node that this attribute belongs to
    /// Note: these indexes are assigned seemingly arbitrarily, and are not necessarily indices
    pub next_attribute_index: i32,
    /// Absolute position of attribute value in the value stream
    pub offset: u32,
}
impl AttributeEntryV3 {
    /// Index into the name hash table
    pub fn name_index(&self) -> u32 {
        self.name_hash_table_index >> 16
    }

    /// Offset in hash chain
    pub fn name_offset(&self) -> u32 {
        self.name_hash_table_index & 0xFFFF
    }

    /// Type of this attribute
    pub fn type_id(&self) -> Option<TypeId> {
        let typ = (self.type_and_length & 0x3F) as u8;
        TypeId::from_id(typ)
    }

    /// Length of this attribute
    pub fn length(&self) -> u32 {
        self.type_and_length >> 6
    }
}
impl Debug for AttributeEntryV3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AttributeEntryV3")
            .field("name_index", &self.name_index())
            .field("name_offset", &self.name_offset())
            .field("type_id", &self.type_id())
            .field("length", &self.length())
            .field("next_attribute_index", &self.next_attribute_index)
            .field("offset", &self.offset)
            .finish()
    }
}
