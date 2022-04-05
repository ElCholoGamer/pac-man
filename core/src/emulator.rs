use crate::{CPU, CPUEvent, Memory, Result};

#[derive(Debug, Clone)]
pub struct Emulator {
    cpu: CPU,
    event: Option<CPUEvent>,
}

impl Emulator {
    pub fn new(program: &[u8]) -> Self {
        let mut memory = Memory::new();
        memory.load_rom(0, program);

        Self {
            cpu: CPU::new(memory),
            event: None,
        }
    }

    pub fn step(&mut self) -> Result<u32> {
        let cycles = self.cpu.cycle()?;
        self.event = self.cpu.poll_event();
        Ok(cycles)
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.event = None;
    }

    pub fn poll_event(&mut self) -> Option<CPUEvent> {
        std::mem::replace(&mut self.event, None)
    }
}
