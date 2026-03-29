//! Opcode map for Q-ISA instructions
//!
//! This generates the named constants, the `OPCODE_MAP` lookup table, and
//! drives `Instruction::operand_count` — all from one place.

macro_rules! define_opcodes {
    ($( $name:ident => ($val:literal, $operands:literal) ),* $(,)?) => {
        $(pub const $name: u8 = $val;)*

        /// (name, opcode byte, operand count)
        pub const OPCODE_MAP: &[(&str, u8, u8)] = &[
            $( (stringify!($name), $val, $operands) ),*
        ];
    };
}

define_opcodes! {
    // Allocation
    OP_QINIT       => (0x01, 1),

    // Single-qubit (no parameter)
    OP_H           => (0x10, 1),
    OP_X           => (0x11, 1),
    OP_Y           => (0x12, 1),
    OP_Z           => (0x13, 1),

    // Single-qubit (parameterized)
    OP_RX          => (0x14, 2),
    OP_RY          => (0x15, 2),
    OP_RZ          => (0x16, 2),

    // Two-qubit
    OP_CNOT        => (0x20, 2),
    OP_SWAP        => (0x21, 2),
    OP_CPHASE      => (0x22, 3),

    // Synchronization
    OP_BARRIER     => (0x30, 0),
    OP_WAIT        => (0x31, 1),

    // Measurement
    OP_MEASURE     => (0x40, 2),
    OP_MEASURE_ALL => (0x41, 0),

    // Termination
    OP_QEND        => (0xF0, 0),
}
