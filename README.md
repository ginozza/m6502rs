# MOS Technology 6502

The 6502 is a **little-endian 8-bit processor** with a 16-bit address bus. The original versions were fabricated using an 8 µm process technology chip with a die size of 3.9 mm × 4.3 mm (153 by 168 mils), for a total area of 16.6 mm2.

The internal logic runs at the same speed as the external clock rate. It featured a simple pipeline; on each cycle, the processor fetches one byte from memory and processes another. This means that any single instruction can take as few as two cycles to complete, depending on the number of operands that instruction uses.

## Registers

Like its precursor, the 6800, the 6502 has very few registers. They include.

- `A` = 8-bit accumulator register
- `P` = 7-bit processor status register
- `n` = negative
- `v` = overflow
- `b` = break (only in stack values, not in hardware)
- `d` = decimal
- `i` = interrupt disable
- `z` = zero
- `c` = carry
- `PC` = 16-bit program counter
- `S` = 8-bit stack pointer
- `X` = 8-bit index register
- `Y` = 8-bit index register

In order to make up somewhat for the lack of registers, the 6502 includes a **zero page addressing mode** that uses one address byte in the instruction instead of the two needed to address the full 64 KB of memory. This provides fast access to the first 256 bytes of RAM by using shorter instructions. For instance, an instruction to add a value from memory to the value in the accumulator would normally be three bytes, one for the instruction and two for the 16-bit address. Using the zero page reduces this to an 8-bit address, reducing the total instruction length to two bytes, and thus improving instruction performance.

The stack address space is hardwired to memory page `$01`, i.e. the address range `$0100`–`$01FF` (256–511). Software access to the stack is done via four implied addressing mode instructions, whose functions are to push or pop (pull) the accumulator or the processor status register. The same stack is also used for subroutine calls via the **JSR** (jump to subroutine) and **RTS** (return from subroutine) instructions and for interrupt handling.

## Addressing
The chip uses the index and stack registers effectively with several addressing modes, including a fast "direct page" or "zero page" mode, similar to that found on the PDP-8, that accesses memory locations from addresses 0 to 255 with a single 8-bit address (saving the cycle normally required to fetch the high-order byte of the address)—code for the 6502 uses the zero page much as code for other processors would use registers. On some 6502-based microcomputers with an operating system, the operating system uses most of zero page, leaving only a handful of locations for the user.

Addressing modes also include implied (1-byte instructions); absolute (3 bytes); indexed absolute (3 bytes); indexed zero-page (2 bytes); relative (2 bytes); accumulator (1); indirect,x and indirect,y (2); and immediate (2). Absolute mode is a general-purpose mode. Branch instructions use a signed 8-bit offset relative to the instruction after the branch; the numerical range −128..127 therefore translates to 128 bytes backward and 127 bytes forward from the instruction following the branch (which is 126 bytes backward and 129 bytes forward from the start of the branch instruction). Accumulator mode operates on the accumulator register and does not need any operand data. Immediate mode uses an 8-bit literal operand.

[Wikipedia](https://en.wikipedia.org/wiki/MOS_Technology_6502)