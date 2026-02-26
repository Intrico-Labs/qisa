use crate::{
    constants::{QISA_CONST_ENTRY_SIZE, QISA_FOOTER_SIZE, QISA_HEADER_SIZE},
    core::{constant::ConstEntry, footer::Footer, header::Header, instruction::Instruction},
    utils::fnv1a_64,
};

#[derive(Debug)]
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
        // --------------------------
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

    pub fn parse_from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        // Validate program checksum from footer
        let stored_checksum = u64::from_le_bytes(
            bytes[(bytes.len() - 16)..(bytes.len() - 8)]
                .try_into()
                .unwrap(),
        );

        let computed_checksum = fnv1a_64(&bytes[..(bytes.len() - 16)]);

        if stored_checksum != computed_checksum {
            return Err("Program checksum mismatch");
        }

        // Parse Header
        let header = Header::parse(&bytes[0..QISA_HEADER_SIZE])?;

        // Parse Constants
        let mut constants: Vec<ConstEntry> = Vec::new();
        let const_count = header.constant_pool_size / QISA_CONST_ENTRY_SIZE as u64;

        for c in 0..const_count {
            let const_idx =
                header.constant_pool_offset as usize + c as usize * QISA_CONST_ENTRY_SIZE;
            let const_end = const_idx + QISA_CONST_ENTRY_SIZE;
            let const_entry = ConstEntry::parse(&bytes[const_idx..const_end])?;
            constants.push(const_entry);
        }

        // Parse Instructions
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut instr_parsed = 0;
        let mut instr_idx = header.instruction_stream_offset as usize;

        while instr_parsed < header.instruction_count {
            let instr = Instruction::parse(&bytes[instr_idx..])?;
            instr_idx += instr.byte_size();
            instructions.push(instr);
            instr_parsed += 1;
        }

        // Parse Footer
        let footer = Footer::parse(&bytes[(bytes.len() - 16)..]);

        Ok(Self {
            header,
            constants,
            instructions: instructions,
            footer: footer,
        })
    }
}
