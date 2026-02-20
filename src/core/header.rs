//! ## Q-ISA Header Representation
//! The header is present at the start of the binary file and contains all the
//! necessary metadata of the instructions and constants present in the file.

pub const QISA_MAGIC: u32 = 0x4155544D;
pub const QISA_VERSION: u16 = 0x0001;
pub const QISA_HEADER_SIZE: usize = 64;

/// ## Header Structure
/// **Note: Refer to the v0.1 specification for detailed architecture.** <br>
/// The header structure does not contain magic bytes 
/// to avoid modification during compilation/runtime.
pub struct Header {
    pub version: u16,
    pub flag: u16,
    pub logical_qubit_count: u32,
    pub classical_register_count: u32,
    pub instruction_count: u64,
    pub constant_pool_offset: u64,
    pub instruction_stream_offset: u64,
    pub constant_pool_size: u64,
    pub instruction_stream_size: u64,
    pub header_checksum: u64,
}