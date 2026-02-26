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

    pub fn parse(bytes: &[u8]) -> Self {
        Self {
            program_checksum: u64::from_le_bytes(
                bytes[(bytes.len() - 16)..(bytes.len() - 8)]
                    .try_into()
                    .unwrap(),
            ),
            reserved: 0,
        }
    }
}
