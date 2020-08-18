use rysk_core::{Mmu,Register32};

use std::cell::RefCell;
use rand::prelude::*;

pub struct Memory<'a> {
    /// ROM for the firmware
    pub firmware: Vec<u8>,
    /// Non-zero when the system should shutdown
    pub shutdown: u8,
    /// RAM for system use
    // ICE when attempting to use const with lifetime bounded struct
    pub memory: [u8; 1024],

    rng: RefCell<ThreadRng>,
    input: RefCell<&'a [u8]>,
    pub output: Vec<u8>
}
impl<'a> Memory<'a> {
    /// Create a new memory manager with `memory` bytes of main memory
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            firmware: Default::default(),
            shutdown: 0,
            memory: [0; Self::MEMORY_SIZE as usize],
            rng: RefCell::new(rand::thread_rng()),
            input: RefCell::new(input),
            output: vec![]
        }
    }

    /// Fetch an instruction
    pub fn fetch(&self, address: u32) -> [u8; 4] {
        [
            self.get(address),
            self.get(address + 1),
            self.get(address + 2),
            self.get(address + 3)
        ]
    }
    /// Load in a firmware file
    pub fn load_firmware<P: AsRef<std::path::Path>>(&mut self, path: P) -> std::io::Result<()> {
        use std::{fs::File, io::Read};
        let mut file = File::open(path.as_ref())?;
        if file.read_to_end(&mut self.firmware)? > Self::FIRMWARE_SIZE as _ {
            panic!("Firmware file {:?} is too big to fit in its address space of {} bytes", path.as_ref(), Self::FIRMWARE_SIZE)
        };

        Ok(())
    }

    // Address ranges
    pub const FIRMWARE_START: u32 = 0x1000;
    pub const FIRMWARE_SIZE: u32 = 0xF000;
    pub const FIRMWARE_END: u32 = Self::FIRMWARE_START + Self::FIRMWARE_SIZE - 1;

    pub const MEMORY_START: u32 = 0x80_0000;
    pub const MEMORY_SIZE: u32 = 1024;
    pub const MEMORY_END: u32 = Self::MEMORY_START + Self::MEMORY_SIZE - 1;
}
impl<'a> Mmu<Register32> for Memory<'a> {
    fn get(&self, address: u32) -> u8 {
        match address {
            1 => {
                let mut input = self.input.borrow_mut();
                if let Some((pop, rest)) = input.split_first() {
                    *input = rest;
                    *pop
                } else {
                    0
                }
            },
            // memory mapped RNG
            10..=73 => self.rng.borrow_mut().next_u32() as u8,
            Self::FIRMWARE_START..=Self::FIRMWARE_END if ((address - Self::FIRMWARE_START)  as usize) < self.firmware.len() => self.firmware[(address - Self::FIRMWARE_START) as usize],
            Self::MEMORY_START..=Self::MEMORY_END if ((address - Self::MEMORY_START)  as usize) < self.memory.len() => self.memory[(address - Self::MEMORY_START) as usize],
            _ => 0
        }
    }

    fn set(&mut self, address: u32, value: u8) {
        match address {
            0 => self.shutdown = value,
            2 => self.output.push(value),
            Self::FIRMWARE_START..=Self::FIRMWARE_END if ((address - Self::FIRMWARE_START)  as usize) < self.firmware.len() => self.firmware[(address - Self::FIRMWARE_START) as usize] = value,
            Self::MEMORY_START..=Self::MEMORY_END => self.memory[(address - Self::MEMORY_START) as usize] = value,
            _ => ()
        }
    }
}