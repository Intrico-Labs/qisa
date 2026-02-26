//! Instruction definition
use crate::core::opcodes::*;

#[derive(Debug, Clone)]
pub enum Instruction {
    // Allocation
    QInit { qubit: u32 },

    // Single-qubit (no parameter)
    H { qubit: u32 },
    X { qubit: u32 },
    Y { qubit: u32 },
    Z { qubit: u32 },

    // Single-qubit (parameterized)
    RX { qubit: u32, const_index: u64 },
    RY { qubit: u32, const_index: u64 },
    RZ { qubit: u32, const_index: u64 },

    // Two-qubit
    CNOT { control: u32, target: u32 },
    SWAP { q1: u32, q2: u32 },
    CPHASE { q1: u32, q2: u32 },

    // Synchronization
    Barrier,
    Wait { nanoseconds: u64 },

    // Measurement
    Measure { qubit: u32, classical: u32 },
    MeasureAll,

    // Termination
    QEnd,
}

impl Instruction {
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        match self {
            Instruction::QInit { qubit } => {
                buffer.push(OP_QINIT);
                buffer.extend_from_slice(&qubit.to_le_bytes());
            }
            Instruction::H { qubit } => {
                buffer.push(OP_H);
                buffer.extend_from_slice(&qubit.to_le_bytes());
            }
            Instruction::X { qubit } => {
                buffer.push(OP_X);
                buffer.extend_from_slice(&qubit.to_le_bytes());
            }
            Instruction::Y { qubit } => {
                buffer.push(OP_Y);
                buffer.extend_from_slice(&qubit.to_le_bytes());
            }
            Instruction::Z { qubit } => {
                buffer.push(OP_Z);
                buffer.extend_from_slice(&qubit.to_le_bytes());
            }
            Instruction::RX { qubit, const_index } => {
                buffer.push(OP_RX);
                buffer.extend_from_slice(&qubit.to_le_bytes());
                buffer.extend_from_slice(&const_index.to_le_bytes());
            }
            Instruction::RY { qubit, const_index } => {
                buffer.push(OP_RY);
                buffer.extend_from_slice(&qubit.to_le_bytes());
                buffer.extend_from_slice(&const_index.to_le_bytes());
            }
            Instruction::RZ { qubit, const_index } => {
                buffer.push(OP_RZ);
                buffer.extend_from_slice(&qubit.to_le_bytes());
                buffer.extend_from_slice(&const_index.to_le_bytes());
            }
            Instruction::CNOT { control, target } => {
                buffer.push(OP_CNOT);
                buffer.extend_from_slice(&control.to_le_bytes());
                buffer.extend_from_slice(&target.to_le_bytes());
            }
            Instruction::SWAP { q1, q2 } => {
                buffer.push(OP_SWAP);
                buffer.extend_from_slice(&q1.to_le_bytes());
                buffer.extend_from_slice(&q2.to_le_bytes());
            }
            Instruction::CPHASE { q1, q2 } => {
                buffer.push(OP_CPHASE);
                buffer.extend_from_slice(&q1.to_le_bytes());
                buffer.extend_from_slice(&q2.to_le_bytes());
            }
            Instruction::Barrier => {
                buffer.push(OP_BARRIER);
            }
            Instruction::Wait { nanoseconds } => {
                buffer.push(OP_WAIT);
                buffer.extend_from_slice(&nanoseconds.to_le_bytes());
            }
            Instruction::Measure { qubit, classical } => {
                buffer.push(OP_MEASURE);
                buffer.extend_from_slice(&qubit.to_le_bytes());
                buffer.extend_from_slice(&classical.to_le_bytes());
            }
            Instruction::MeasureAll => {
                buffer.push(OP_MEASURE_ALL);
            }
            Instruction::QEnd => {
                buffer.push(OP_QEND);
            }
        }

        buffer
    }

    pub fn parse(bytes: &[u8]) -> Result<Instruction, &'static str> {
        let opcode = bytes.first().ok_or("empty instruction bytes")?;

        match *opcode {
            OP_QINIT => {
                if bytes.len() < 5 {
                    return Err("not enough bytes for QInit");
                }
                let qubit = u32::from_le_bytes(bytes[1..5].try_into().unwrap());
                Ok(Instruction::QInit { qubit })
            }
            OP_H => {
                if bytes.len() < 5 {
                    return Err("not enough bytes for H");
                }
                let qubit = u32::from_le_bytes(bytes[1..5].try_into().unwrap());
                Ok(Instruction::H { qubit })
            }
            OP_X => {
                if bytes.len() < 5 {
                    return Err("not enough bytes for X");
                }
                let qubit = u32::from_le_bytes(bytes[1..5].try_into().unwrap());
                Ok(Instruction::X { qubit })
            }
            OP_Y => {
                if bytes.len() < 5 {
                    return Err("not enough bytes for Y");
                }
                let qubit = u32::from_le_bytes(bytes[1..5].try_into().unwrap());
                Ok(Instruction::Y { qubit })
            }
            OP_Z => {
                if bytes.len() < 5 {
                    return Err("not enough bytes for Z");
                }
                let qubit = u32::from_le_bytes(bytes[1..5].try_into().unwrap());
                Ok(Instruction::Z { qubit })
            }
            OP_RX => {
                if bytes.len() < 13 {
                    return Err("not enough bytes for RX");
                }
                let qubit = u32::from_le_bytes(bytes[1..5].try_into().unwrap());
                let const_index = u64::from_le_bytes(bytes[5..13].try_into().unwrap());
                Ok(Instruction::RX { qubit, const_index })
            }
            OP_RY => {
                if bytes.len() < 13 {
                    return Err("not enough bytes for RY");
                }
                let qubit = u32::from_le_bytes(bytes[1..5].try_into().unwrap());
                let const_index = u64::from_le_bytes(bytes[5..13].try_into().unwrap());
                Ok(Instruction::RY { qubit, const_index })
            }
            OP_RZ => {
                if bytes.len() < 13 {
                    return Err("not enough bytes for RZ");
                }
                let qubit = u32::from_le_bytes(bytes[1..5].try_into().unwrap());
                let const_index = u64::from_le_bytes(bytes[5..13].try_into().unwrap());
                Ok(Instruction::RZ { qubit, const_index })
            }
            OP_CNOT => {
                if bytes.len() < 9 {
                    return Err("not enough bytes for CNOT");
                }
                let control = u32::from_le_bytes(bytes[1..5].try_into().unwrap());
                let target = u32::from_le_bytes(bytes[5..9].try_into().unwrap());
                Ok(Instruction::CNOT { control, target })
            }
            OP_SWAP => {
                if bytes.len() < 9 {
                    return Err("not enough bytes for SWAP");
                }
                let q1 = u32::from_le_bytes(bytes[1..5].try_into().unwrap());
                let q2 = u32::from_le_bytes(bytes[5..9].try_into().unwrap());
                Ok(Instruction::SWAP { q1, q2 })
            }
            OP_CPHASE => {
                if bytes.len() < 9 {
                    return Err("not enough bytes for CPHASE");
                }
                let q1 = u32::from_le_bytes(bytes[1..5].try_into().unwrap());
                let q2 = u32::from_le_bytes(bytes[5..9].try_into().unwrap());
                Ok(Instruction::CPHASE { q1, q2 })
            }
            OP_BARRIER => Ok(Instruction::Barrier),
            OP_WAIT => {
                if bytes.len() < 9 {
                    return Err("not enough bytes for Wait");
                }
                let nanoseconds = u64::from_le_bytes(bytes[1..9].try_into().unwrap());
                Ok(Instruction::Wait { nanoseconds })
            }
            OP_MEASURE => {
                if bytes.len() < 9 {
                    return Err("not enough bytes for Measure");
                }
                let qubit = u32::from_le_bytes(bytes[1..5].try_into().unwrap());
                let classical = u32::from_le_bytes(bytes[5..9].try_into().unwrap());
                Ok(Instruction::Measure { qubit, classical })
            }
            OP_MEASURE_ALL => Ok(Instruction::MeasureAll),
            OP_QEND => Ok(Instruction::QEnd),
            _ => Err("unknown opcode"),
        }
    }

    pub fn operand_count(opcode: &u8) -> Option<u8> {
        OPCODE_MAP
            .iter()
            .find(|(_, op, _)| *op == *opcode)
            .map(|(_, _, count)| *count)
    }
}
