# Intel 8080

Short and sweet reference manual for the intel 8080

## Annotations

* SP - Stack Pointer
* PC - Program Counter
* MS(B) - Most Significant (Byte)
* LS(B) - Least Significant (Byte)
* OP(C) - Operation (Code)

## Overview

The intel 8080 consists of the following parts

- Seven registers
- Memory
- Program counter
- Stack pointer 
- I/O

## Working registers

Seven registers:
* *A* - accumulator*
* *B* - 'scratchpad' register
* *C* - 'scratchpad' register
* *D* - 'scratchpad' register
* *E* - 'scratchpad' register
* *H* - Common use: MSB of Address
* *L* - Common use: LSB of Address
 

## Memory

* Size: 65 KB memory, 0x00000 - 0xFFFFH
* Address length: 16 Bits

Memory Addressing Modes:
- *Direct Addressing*: Instruction supplies the exact memory address
- *Register Pair Addressing*: A register contains the address. Register H
  contains most significant Byte, L contains lowest significant Byte.
- *Stack Pointer Addressing*: The stack pointer address is used. See *pop/push* in
  the "Stack Pointer" section.
- *Immediate Addressing*: Loads next byte (Byte after instruction byte) into the
  *A* register.


## Status Flags/Bits (sometimes called status register)

Bits that have special representation. Each individual bit represents a _flag_. 

* *Carry Bit*: Affected by:
  - addition:  TODO
  - subtraction: TODO
  - rotation: TODO
  - logical OP:  TOD
* *Auxiliary Carry Bit*: Indicates overflow (carry out) of bit 3. Special bit only for
 instruction DAA (TODO) and cannot be tested. 
* *Sign Bit*: A byte cab be represented as Two complement, if __bit 7 is set_ the numerical
   range is [-128,-1], if _bit 7 is zero_ the range is [0,127]. The *Sign Bit*
   is set to the conditions of bit 7 after certain instructions (TODO).
* *Zero Bit*: Is set if the result is zero for some instructions.
* *Parity Bit*: Is set after certain operations depending of parity. Parity
means if there's odd or even number of set bits in s byte. Flag is *set if odd*
and *0 if even*.

## Program Counter (PC)

The program counter is a 16 bit register. Contains address of next instruction
to execute.

## Stack pointer (SP)

The stack pointer specifies addresses for stack operations. There is only two
stack operations:
- *Push*: 16 bits of data from a register pair or the program counter is moved
  to memory (stack). Push operation:
  1. Most significant  8 bits are stored at _one lower_ address than held in the stack
    pointer
  2. Least significant 8 bits are stored at _two lower_ addresses than held in
    the stack pointer
  3. Stack pointer is decremented by two.
- *Pop*: 16 bits of data is moved to a register pair or the program counter from memory (stack). Pop operation:
  1. The LSB is loaded from the address of the stack pointer
  2. The MSB is loaded from _one address greater_ than the address of the stack pointer. 
  3. The stack pointer is incremented by two
  
## Instructions

* Length: _8 Bits_ 

## Changing sign of byte
To change positive number to negative. Complement each bit with 1 by an _and_
operation, add 1 to the result, ignore carry out. 

For example: change the value 100 to -100:
| Bit   | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| value | 0 | 1 | 1 | 0 | 0 | 1 | 0 | 0 |
*AND*
| Bit   | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| value | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
*Equals*
| Bit   | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| value | 1 | 0 | 0 | 1 | 1 | 0 | 1 | 1 |
which represents -100

## I/O

TODO or not





