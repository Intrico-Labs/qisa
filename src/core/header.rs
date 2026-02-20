//! ## Q-ISA Header Representation
//! The header is present at the start of the binary file and contains all the
//! necessary metadata of the instructions and constants present in the file.

use crate::utils::fnv1a_64;

pub const QISA_MAGIC_BYTES: u32 = 0x4155544D;
pub const QISA_VERSION: u16 = 0x0001;
pub const QISA_HEADER_SIZE: usize = 64;

/// ## Header Structure
/// **Note: Refer to the v0.1 specification for detailed architecture.** <br>
/// The header structure does not contain magic bytes 
/// to avoid modification during compilation/runtime.
pub struct Header {
    pub version: u16,
    pub flags: u16,
    pub logical_qubit_count: u32,
    pub classical_register_count: u32,
    pub instruction_count: u64,
    pub constant_pool_offset: u64,
    pub instruction_stream_offset: u64,
    pub constant_pool_size: u64,
    pub instruction_stream_size: u64,
    pub header_checksum: u64,
}

impl Header {
    /// Internal constructor for Header
    pub fn new(
        logical_qubit_count: u32,
        classical_register_count: u32,
        instruction_count: u64,
        constant_pool_offset: u64,
        instruction_stream_offset: u64,
        constant_pool_size: u64,
        instruction_stream_size: u64,
    ) -> Self {
        Self {
            version: QISA_VERSION,
            flags: 0,
            logical_qubit_count,
            classical_register_count,
            instruction_count,
            constant_pool_offset,
            instruction_stream_offset,
            constant_pool_size,
            instruction_stream_size,
            header_checksum: 0, // computed later
        }
    }

    /// Explicit serializing with little-endian encoding. This serialized buffer contains magic bytes.
    pub fn serialize(&self) -> [u8; QISA_HEADER_SIZE] {
        let mut buffer = [0u8; QISA_HEADER_SIZE];

        buffer[0..4].copy_from_slice(&QISA_MAGIC_BYTES.to_le_bytes());
        buffer[4..6].copy_from_slice(&self.version.to_le_bytes());
        buffer[6..8].copy_from_slice(&self.flags.to_le_bytes());
        buffer[8..12].copy_from_slice(&self.logical_qubit_count.to_le_bytes());
        buffer[12..16].copy_from_slice(&self.classical_register_count.to_le_bytes());
        buffer[16..24].copy_from_slice(&self.instruction_count.to_le_bytes());
        buffer[24..32].copy_from_slice(&self.constant_pool_offset.to_le_bytes());
        buffer[32..40].copy_from_slice(&self.instruction_stream_offset.to_le_bytes());
        buffer[40..48].copy_from_slice(&self.constant_pool_size.to_le_bytes());
        buffer[48..56].copy_from_slice(&self.instruction_stream_size.to_le_bytes());

        let checksum= fnv1a_64(&buffer[0..56]);
        buffer[56..64].copy_from_slice(&checksum.to_le_bytes());

        buffer
    }

    /// Spec-compliant parsing with proper validations
    pub fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < QISA_HEADER_SIZE {
            return Err("Header too small");
        }

        let magic = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        if magic != QISA_MAGIC_BYTES {
            return Err("Invalid magic bytes");
        }

        let stored_checksum =
            u64::from_le_bytes(bytes[56..64].try_into().unwrap());

        let computed_checksum = fnv1a_64(&bytes[0..56]);

        if stored_checksum != computed_checksum {
            return Err("Header checksum mismatch");
        }

        Ok(Self {
            version: u16::from_le_bytes(bytes[4..6].try_into().unwrap()),
            flags: u16::from_le_bytes(bytes[6..8].try_into().unwrap()),
            logical_qubit_count: u32::from_le_bytes(bytes[8..12].try_into().unwrap()),
            classical_register_count: u32::from_le_bytes(bytes[12..16].try_into().unwrap()),
            instruction_count: u64::from_le_bytes(bytes[16..24].try_into().unwrap()),
            constant_pool_offset: u64::from_le_bytes(bytes[24..32].try_into().unwrap()),
            instruction_stream_offset: u64::from_le_bytes(bytes[32..40].try_into().unwrap()),
            constant_pool_size: u64::from_le_bytes(bytes[40..48].try_into().unwrap()),
            instruction_stream_size: u64::from_le_bytes(bytes[48..56].try_into().unwrap()),
            header_checksum: stored_checksum,
        })
    }
}