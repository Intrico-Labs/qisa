//! ## Footer Definition
//! The footer is 16 bytes in length and contains the program checksum
//! and other optional metadata (reserved bytes)
//!

pub struct Footer {
    pub program_checksum: u64,
    pub reserved: u64,
}

impl Footer {
    pub fn new() -> Self {
        Self {
            program_checksum: 0, // will be calculated while building Program
            reserved: 0,
        }
    }
}
