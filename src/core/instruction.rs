//! Instruction definition
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
                buffer.push(0x01);
                buffer.extend_from_slice(&qubit.to_le_bytes());
            }
            Instruction::H { qubit } => {
                buffer.push(0x10);
                buffer.extend_from_slice(&qubit.to_le_bytes());
            }
            Instruction::X { qubit } => {
                buffer.push(0x11);
                buffer.extend_from_slice(&qubit.to_le_bytes());
            }
            Instruction::Y { qubit } => {
                buffer.push(0x12);
                buffer.extend_from_slice(&qubit.to_le_bytes());
            }
            Instruction::Z { qubit } => {
                buffer.push(0x13);
                buffer.extend_from_slice(&qubit.to_le_bytes());
            }
            Instruction::RX { qubit, const_index } => {
                buffer.push(0x14);
                buffer.extend_from_slice(&qubit.to_le_bytes());
                buffer.extend_from_slice(&const_index.to_le_bytes());
            }
            Instruction::RY { qubit, const_index } => {
                buffer.push(0x15);
                buffer.extend_from_slice(&qubit.to_le_bytes());
                buffer.extend_from_slice(&const_index.to_le_bytes());
            }
            Instruction::RZ { qubit, const_index } => {
                buffer.push(0x16);
                buffer.extend_from_slice(&qubit.to_le_bytes());
                buffer.extend_from_slice(&const_index.to_le_bytes());
            }
            Instruction::CNOT { control, target } => {
                buffer.push(0x20);
                buffer.extend_from_slice(&control.to_le_bytes());
                buffer.extend_from_slice(&target.to_le_bytes());
            }
            Instruction::SWAP { q1, q2 } => {
                buffer.push(0x21);
                buffer.extend_from_slice(&q1.to_le_bytes());
                buffer.extend_from_slice(&q2.to_le_bytes());
            }
            Instruction::CPHASE { q1, q2 } => {
                buffer.push(0x22);
                buffer.extend_from_slice(&q1.to_le_bytes());
                buffer.extend_from_slice(&q2.to_le_bytes());
            }
            Instruction::Barrier => {
                buffer.push(0x30);
            }
            Instruction::Wait { nanoseconds } => {
                buffer.push(0x31);
                buffer.extend_from_slice(&nanoseconds.to_le_bytes());
            }
            Instruction::Measure { qubit, classical } => {
                buffer.push(0x40);
                buffer.extend_from_slice(&qubit.to_le_bytes());
                buffer.extend_from_slice(&classical.to_le_bytes());
            }
            Instruction::MeasureAll => {
                buffer.push(0x41);
            }
            Instruction::QEnd => {
                buffer.push(0xF0);
            }
        }

        buffer
    }
}
