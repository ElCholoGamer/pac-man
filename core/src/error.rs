use std::fmt::{Debug, Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    UnimplementedOpcode(u8),
    IllegalExtendedOpcode(u16),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnimplementedOpcode(opcode) => write!(f, "unimplemented opcode: 0x{:02X}", opcode),
            Self::IllegalExtendedOpcode(opcode) => write!(f, "illegal extended opcode: 0x{:04X}", opcode),
        }
    }
}

impl std::error::Error for Error {}
