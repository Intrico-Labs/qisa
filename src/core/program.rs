use crate::core::{constant::ConstEntry, footer::Footer, header::Header, instruction::Instruction};

pub struct Program {
    pub header: Header,
    pub constants: Vec<ConstEntry>,
    pub instructions: Vec<Instruction>,
    pub footer: Footer,
}
