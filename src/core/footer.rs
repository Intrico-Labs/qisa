//! ## Footer Definition
//! The footer is 16 bytes in length and contains the program checksum
//! and other optional metadata (reserved bytes)
//!

pub const QISA_FOOTER_SIZE: usize = 16;

pub struct Footer {
    pub program_checksum: u64,
    pub reserved: u64,
}
