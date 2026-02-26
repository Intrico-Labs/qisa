//! Opcode map for Q-ISA instructions
//!
//! This generates both the named constants (e.g. `OP_H`) and the `OPCODE_MAP`
//! lookup table automatically.

macro_rules! define_opcodes {
    ($( $name:ident => $val:literal ),* $(,)?) => {
        $(pub const $name: u8 = $val;)*

        pub const OPCODE_MAP: &[(&str, u8)] = &[
            $( (stringify!($name), $val) ),*
        ];
    };
}

define_opcodes! {
    // Allocation
    OP_QINIT       => 0x01,

    // Single-qubit (no parameter)
    OP_H           => 0x10,
    OP_X           => 0x11,
    OP_Y           => 0x12,
    OP_Z           => 0x13,

    // Single-qubit (parameterized)
    OP_RX          => 0x14,
    OP_RY          => 0x15,
    OP_RZ          => 0x16,

    // Two-qubit
    OP_CNOT        => 0x20,
    OP_SWAP        => 0x21,
    OP_CPHASE      => 0x22,

    // Synchronization
    OP_BARRIER     => 0x30,
    OP_WAIT        => 0x31,

    // Measurement
    OP_MEASURE     => 0x40,
    OP_MEASURE_ALL => 0x41,

    // Termination
    OP_QEND        => 0xF0,
}
