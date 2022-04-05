#![allow(unused)]

use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Memory {
    rom: [u8; 0x4000],
    ram: [u8; 0x1000],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            rom: [0; 0x4000],
            ram: [0; 0x1000],
        }
    }

    pub fn load_rom(&mut self, offset: usize, rom: &[u8]) {
        for (i, val) in rom.iter().enumerate() {
            self.rom[offset + i] = *val;
        }
    }
}

impl Index<u16> for Memory {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        let index = index as usize;

        if index < self.rom.len() {
            &self.rom[index]
        } else {
            &self.ram[(index - self.rom.len()) % self.ram.len()]
        }
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        let index = index as usize;

        if index < self.rom.len() {
            &mut self.rom[index]
        } else {
            &mut self.ram[(index - self.rom.len()) % self.ram.len()]
        }
    }
}
