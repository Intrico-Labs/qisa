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
