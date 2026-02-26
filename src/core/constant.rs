//! ## Constant Entry Definition
//! A constant entry basically holds any parameters required for the instruction execution. <br>
//! For example: A Rotational-X Instruction (RX) will require the rotation (theta) in order to execute.
//!

use crate::constants::QISA_CONST_ENTRY_SIZE;

/// ## Constant Entry
/// Size: 16 Bytes
///
#[derive(Clone, Debug)]
pub struct ConstEntry {
    pub kind: ConstKind,
}

#[derive(Clone, Debug)]
pub enum ConstKind {
    F64(f64),
}

impl ConstEntry {
    pub fn serialize(&self) -> [u8; QISA_CONST_ENTRY_SIZE] {
        let mut buffer = [0u8; QISA_CONST_ENTRY_SIZE];

        match self.kind {
            ConstKind::F64(value) => {
                buffer[0] = 0x01;
                buffer[8..16].copy_from_slice(&value.to_le_bytes());
            }
        }

        buffer
    }

    pub fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() != QISA_CONST_ENTRY_SIZE {
            return Err("Invalid constant entry size");
        }

        let kind = match bytes[0] {
            0x01 => {
                let value = f64::from_le_bytes(bytes[8..16].try_into().unwrap());
                ConstKind::F64(value)
            }
            _ => return Err("Unsupported constant type"),
        };

        Ok(ConstEntry { kind })
    }
}
