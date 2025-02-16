use std::ops::{Index, IndexMut};

pub const MAX_MEM: usize = 1024 * 64; // 64KB address space

/// 6502 Memory emulator.
pub struct Mem {
    data: [u8; MAX_MEM], // Unified array for program/data/stack 
}

impl Mem {
    /// Create a new memory instance with all bytes initialized.
    pub fn new() -> Self {
        Self { data: [0; MAX_MEM] }
    }

    /* Cycle #0
    * Process Register:
    * AB:00FF D:00 R/W:1 PC:00FF A:AA X:00 Y:00 SP:00 P:02 IR:00 READ $00FF = $00
    *
    * When a 6502 is turned on, the stack pointer is initialized with
    * zero. The BRK/IRQ_/NMI_/RES_ sequence pulls the instruction register
    * (IR) to zero.
    * */ 

    /// Initialize or clear the memory.
    pub fn initialize(&mut self) {
        self.data.fill(0);
    }

    /// Write a 16-bit word (2 bytes) in little-endian order to memory.
    /// This method also decrements the cycle count by 2.
    pub fn write_word(&mut self, value: u16, address: u32, cycles: &mut u32) {
        self.data[address as usize] = (value & 0xFF) as u8;
        self.data[(address + 1) as usize] = (value >> 8) as u8;
        *cycles = cycles.saturating_sub(2);
    }
}

/// Allow read-only indexing into the memory.
impl Index<usize> for Mem {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

/// Allow mutable indexing into the memory.
impl IndexMut<usize> for Mem {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
