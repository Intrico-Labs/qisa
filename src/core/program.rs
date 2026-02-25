use crate::{
    core::{
        constant::{ConstEntry, QISA_CONST_ENTRY_SIZE},
        footer::{Footer, QISA_FOOTER_SIZE},
        header::{Header, QISA_HEADER_SIZE},
        instruction::Instruction,
    },
    utils::fnv1a_64,
};

pub struct Program {
    pub header: Header,
    pub constants: Vec<ConstEntry>,
    pub instructions: Vec<Instruction>,
    pub footer: Footer,
}

impl Program {
    pub fn compile_to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        // Placeholder for building header later on
        buffer.resize(QISA_HEADER_SIZE, 0);

        // ---------------------------
        // Constant Pool
        // ---------------------------
        let const_pool_offset = QISA_HEADER_SIZE as u64;

        for c in &self.constants {
            buffer.extend_from_slice(&c.serialize());
        }

        let const_pool_size = (self.constants.len() * QISA_CONST_ENTRY_SIZE) as u64;

        // ---------------------------
        // Instruction Stream
        // ---------------------------
        let instruction_stream_offset = buffer.len() as u64;

        for i in &self.instructions {
            buffer.extend_from_slice(&i.serialize());
        }

        let instruction_stream_size = (buffer.len() as u64) - instruction_stream_offset;

        // Footer placeholder
        let footer_offset = buffer.len();
        buffer.resize(footer_offset + QISA_FOOTER_SIZE, 0);

        // ---------------------------
        // Build Header
        // ---------------------------
        let header = self.header.build(
            self.instructions.len() as u64,
            const_pool_offset,
            instruction_stream_offset,
            const_pool_size,
            instruction_stream_size,
        );

        buffer[0..QISA_HEADER_SIZE].copy_from_slice(&header.serialize());

        // ---------------------------
        // Program Checksum
        // ---------------------------
        let program_checksum = fnv1a_64(&buffer[..footer_offset]);
        buffer[footer_offset..footer_offset + 8].copy_from_slice(&program_checksum.to_le_bytes());

        buffer
    }
}
