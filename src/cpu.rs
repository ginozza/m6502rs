use crate::memory::Mem;

/// 6502 CPU emulator.
pub struct CPU {

    /* The program counter (PC), commonly called the instruction pointer
   * (IP) in Intel x86 and Itanium microprocessors, and sometimes called the 
   * instruction address register (IAR), the instruction counter, or just part 
   * of the instruction squencer, is a processor register that indicates where 
   * a compuer is in its program sequence.
   *
   * Usually, the PC is incremented after fetching an instruction, and holds 
   * the memory address of ("points to") the next instruction that would be executed.
   *
   * Source: Wikipedia.
   * Retrieved from: https://en.wikipedia.org/wiki/Program_counter
   */ 

    pc: u16, // Program counter 
    
    /* A stack register is a computer central processor register whose purpose
   * is to keep track of a call stack. On an accumulator-based architecture
   * machine, this may be a dedicated register such as SP on an Intel x86 machine.
   * On a general register machine, it may be a register which is reserved by 
   * convention, such as on the PDP-11 or RISC machines. Some designs such
   * as the Data General Eclipse had no dedicated register, but used a reserverd
   * hardware memory address for this function.
   *
   * Source: Wikipedia.
   * Retrieved from: https://en.wikipedia.org/wiki/Stack_register
   */ 

    sp: u16, // Stack pointer
    
    /* A processor register is a quickly accesible location available to a 
   * computer processor. Registers usually consit of a small amount of fast
   * storage, although some registers have specific hardware functions, and
   * may be read-only or write-only. In computer architecture, registers 
   * are typically addressed by mechanisms other than main memory, but may 
   * in some cases be assigned a memory address e.g DEC PDC-10, ICT 1900.
   *
   * Source: Wikipedia
   * Retrieved from: https://en.wikipedia.org/wiki/Processor_register
   * */ 

    a: u8, // Registers
    x: u8,
    y: u8,

    /* A status register, flag register, or condition code register (CCR) 
   * is a collection of status flag bits for a processor. Examples of 
   * such registers include FLAGS register in the x86 architecture, flags 
   * in the program status word (PSW) register in the IBM System/360 
   * architecture through z/Architecture, and the application program 
   * status register (APSR) in the ARM Cortex-A architecture.[1]
   *
   * The status register is a hardware register that contains information 
   * about the state of the processor. Individual bits are implicitly or 
   * explicitly read and/or written by the machine code instructions 
   * executing on the processor. The status register lets an instruction 
   * take action contingent on the outcome of a previous instruction.
   *
   * Source: Wikipedia.
   * Retrieved from: 
   * */ 

    c: u8, // Carry flag
    z: u8, // Zero flag  
    i: u8, // Interrupt Disable 
    d: u8, // Decimal Mode
    b: u8, // Break Command
    v: u8, // Overflow flag
    n: u8, // Negative flag
}

impl CPU {

    // Instruction opcodes.

    /// LDA inmediate
    pub const INS_LDA_IM: u8 = 0xA9;
    /// LDA zero page
    pub const INS_LDA_ZP: u8 = 0xA5; 
    /// LDA zero page X
    pub const INS_LDA_ZPX: u8 = 0xB5;
    /// JSR (junp to subroutine)
    pub const INS_JSR: u8 = 0x20; 

    /// Create a new CPU Instance with registers cleared.
    pub fn new() -> Self {
        Self {
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
            c: 0,
            z: 0,
            i: 0,
            d: 0,
            b: 0,
            v: 0,
            n: 0,
        }
    }

    /* Reset refers to the process of returning the computer to 
    * the apparent default (or ground) state of the computer – with or 
    * without memory intact. The computer will return to the default start-up 
    * screen if the motherboard has not been damaged, modified or expanded.
    *
    * A reset can be achieved by using the commands JMP (machine code) :
    * followed by the hexadecimal address or SYS (BASIC) followed by the 
    * decimal address of the system reset routine. These commands will 
    * then activate the routine located at the address pointed to by 
    * the reset vector. For example, to reset the C64 from BASIC use SYS 64738.
    *
    * This is the default machine code routine to reset the C64:
    * 
    * @code
    * ; MOS 6510 System Reset routine[3]
    * ; Reset vector (Kernal address $FFFC) points here.
    * ; 
    * ; If cartridge is detected then cartridge cold start routine is activated.
    * ; If no cartridge is detected then I/O and memory are initialised and BASIC cold start routine is activated.
    * FCE2   A2 FF      LDX #$FF        ; 
    * FCE4   78         SEI             ; set interrupt disable
    * FCE5   9A         TXS             ; transfer .X to stack
    * FCE6   D8         CLD             ; clear decimal flag
    * FCE7   20 02 FD   JSR $FD02       ; check for cart
    * FCEA   D0 03      BNE $FCEF       ; .Z=0? then no cart detected
    * FCEC   6C 00 80   JMP ($8000)     ; direct to cartridge cold start via vector
    * FCEF   8E 16 D0   STX $D016       ; sets bit 5 (MCM) off, bit 3 (38 cols) off
    * FCF2   20 A3 FD   JSR $FDA3       ; initialise I/O
    * FCF5   20 50 FD   JSR $FD50       ; initialise memory
    * FCF8   20 15 FD   JSR $FD15       ; set I/O vectors ($0314..$0333) to kernal defaults
    * FCFB   20 5B FF   JSR $FF5B       ; more initialising... mostly set system IRQ to correct value and start
    * FCFE   58         CLI             ; clear interrupt flag
    * FCFF   6C 00 A0   JMP ($A000)     ; direct to BASIC cold start via vector
    *
    * Source: C64-Wiki.
    * Retrieved from: https://www.c64-wiki.com/wiki/Reset_(Process)
    * */

    /// Reset the CPU to its initial state.
    ///
    /// This method sets the program counter to the reset vector (0xFFFC), initializes the stack
    /// pointer, clears registers and flags, and initializes memory.
    pub fn reset(&mut self, memory: &mut Mem) {
        self.pc = 0xFFFC;
        self.sp = 0x0100;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.c = 0;
        self.z = 0;
        self.i = 1;
        self.d = 0;
        self.b = 0;
        self.v = 0;
        self.n = 0;  
        memory.initialize();
    }

    /// Fetch one byte from memory at the current PC.
    /// This decrements the cycle count by 1.
    pub fn fetch_byte(&mut self, cycles: &mut u32, memory: &Mem) -> u8 {
        let data = memory[self.pc as usize];
        self.pc = self.pc.wrapping_add(1);
        *cycles = cycles.saturating_sub(1);
        data
    }

    /// Fetch a 16-bit word (2 bytes) from memory in little-endian order.
    /// This decrements the cycle count by 2.
    pub fn fetch_word(&mut self, cycles: &mut u32, memory: &Mem) -> u16 {
        // 6502 is little-endian: first byte is the least significant.
        let low = memory[self.pc as usize] as u16;
        self.pc = self.pc.wrapping_add(1);
        let high = memory[self.pc as usize] as u16;
        self.pc = self.pc.wrapping_add(1);
        *cycles = cycles.saturating_sub(2);
        (high << 8) | low
    }

    /// Read one byte from memory given an 8-bit address.
    /// This decrements the cycle count by 1.
    pub fn read_byte(&mut self, cycles: &mut u32, address: u8, memory: &Mem) -> u8 {
        let data = memory[address as usize];
        *cycles = cycles.saturating_sub(1);
        data
    }

    /// Set the status flags based on the contents of the accumulator.
    pub fn lda_set_status(&mut self) {
        self.z = if self.a == 0 {1} else {0}; 
        self.n = if (self.a & 0b10000000) > 0 {1} else {0};
    }

    /// Execute instructions until the cycle count reaches zero.
    ///
    /// This method has a loop that fetches an opcoed and uses a `match` to determine which
    /// operation to perform.
    pub fn execute(&mut self, mut cycles: u32, memory: &mut Mem) {
        while cycles > 0 {
            let ins: u8 = self.fetch_byte(&mut cycles, memory);
            match ins {
                Self::INS_LDA_IM => {
                    let value = self.fetch_byte(&mut cycles, memory);
                    self.a = value;
                    self.lda_set_status();
                }
                Self::INS_LDA_ZP => {
                    let zp_addr = self.fetch_byte(&mut cycles, memory);
                    self.a = self.read_byte(&mut cycles, zp_addr, memory);
                    self.lda_set_status();
                }
                Self::INS_JSR => {
                    let sub_addr: u16 = self.fetch_word(&mut cycles, memory);
                    // Write return address (PC - 1) to the stack.
                    memory.write_word(self.pc.wrapping_sub(1), self.sp as u32, &mut cycles);
                    self.pc = sub_addr;
                    self.sp = self.sp.wrapping_add(2);
                    cycles = cycles.saturating_sub(1);
                }
                _ => {
                    println!("Instruction not handled {:#X}", ins);
                    break;
                }
            }
        }
    }
}


impl CPU {
    // Getters
    pub fn get_pc(&self) -> u16 { self.pc }
    pub fn get_sp(&self) -> u16 { self.sp }
    pub fn get_a(&self) -> u8 { self.a }
    pub fn get_x(&self) -> u8 { self.x }
    pub fn get_y(&self) -> u8 { self.y }
    pub fn get_c(&self) -> u8 { self.c }
    pub fn get_z(&self) -> u8 { self.z }
    pub fn get_i(&self) -> u8 { self.i }
    pub fn get_d(&self) -> u8 { self.d }
    pub fn get_b(&self) -> u8 { self.b }
    pub fn get_v(&self) -> u8 { self.v }
    pub fn get_n(&self) -> u8 { self.n }

    // Setters
    pub fn set_pc(&mut self, value: u16) { self.pc = value; }
    pub fn set_a(&mut self, value: u8) { self.a = value; }
}

