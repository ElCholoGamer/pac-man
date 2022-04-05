#![allow(unused)]

use std::mem;
use std::ops::{Index, IndexMut};
use std::rc::Rc;

use crate::{concat_u16, Error, Memory, Result};

#[derive(Debug, Clone)]
pub enum State {
    Active,
    Halted,
}

#[derive(Debug, Clone)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug, Clone)]
pub enum Event {
    PortIn(u8, Register),
    PortOut(u8, u8),
}

#[derive(Debug, Clone)]
pub struct CPU {
    state: State,
    interrupts_enabled: bool,
    clock_cycles: u32,
    event: Option<Event>,
    memory: Memory,
    pc: u16,
    flags: u8,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
}

impl CPU {
    pub fn new(memory: Memory) -> Self {
        Self {
            memory,
            state: State::Active,
            interrupts_enabled: true,
            clock_cycles: 0,
            event: None,
            pc: 0,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
        }
    }

    pub fn reset(&mut self) {
        self.interrupts_enabled = true;
        self.state = State::Active;
        self.clock_cycles = 0;
        self.event = None;
        self.pc = 0;
        self.flags = 0;
        self.a = 0;
        self.b = 0;
        self.c = 0;
        self.d = 0;
        self.e = 0;
        self.h = 0;
        self.l = 0;
        self.sp = 0;
    }

    pub fn cycle(&mut self) -> Result<u32> {
        self.clock_cycles = 1;
        let opcode = self.read_pc();

        match opcode {
            0x00 => {}                                              // NOP
            0x76 => {                                                // HALT
                self.state = State::Halted;
                self.pc -= 1;
            }
            0xF3 => self.interrupts_enabled = false,                 // DI
            0xFB => self.interrupts_enabled = true,                  // EI
            0xD3 => {                                                // OUT (*), a
                let port = self.read_pc();
                self.port_out(port, self.a)
            }
            0xDB => {                                                // IN a, (*)
                let port = self.read_pc();
                self.port_in(Register::A, port);
            }

            _ => return Err(Error::UnimplementedOpcode(opcode)),
        }

        println!("Finished 0x{:02X} in {} cycles", opcode, self.clock_cycles);
        Ok(self.clock_cycles)
    }

    pub fn poll_event(&mut self) -> Option<Event> {
        mem::replace(&mut self.event, None)
    }

    pub fn port_out(&mut self, port: u8, val: u8) {
        self.event = Some(Event::PortOut(port, val));
        self.clock_cycles += 4;
    }

    pub fn port_in(&mut self, reg: Register, port: u8) {
        self.event = Some(Event::PortIn(port, reg));
        self.clock_cycles += 4;
    }

    fn read_pc(&mut self) -> u8 {
        let val = self.memory[self.pc];
        self.pc += 1;
        self.clock_cycles += 3;
        val
    }

    fn read_pc_u16(&mut self) -> u16 {
        let lo = self.read_pc();
        let hi = self.read_pc();
        concat_u16!(hi, lo)
    }

    fn mem_read(&mut self, adr: u16) -> u8 {
        self.clock_cycles += 3;
        self.memory[adr]
    }

    fn mem_write(&mut self, adr: u16, val: u8) {
        self.clock_cycles += 3;
        self.memory[adr] = val;
    }
}
