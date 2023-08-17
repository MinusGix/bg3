use indexmap::IndexMap;
use lsf::lsx::Region;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LSMetadata {
    pub timestamp: u64,
    pub major_version: u32,
    pub minor_version: u32,
    pub revision: u32,
    pub build_number: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Resource<'b> {
    pub metadata: LSMetadata,
    // TODO: is it lsx region??
    pub regions: IndexMap<String, Region<'b>>,
}
