//! ## Footer Definition
//! The footer is 16 bytes in length and contains the program checksum
//! and other optional metadata (reserved bytes)
//!
pub struct Footer {
    pub program_checksum: u64,
    pub reserved: u64,
}
