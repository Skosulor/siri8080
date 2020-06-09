
# *S*pace *I*nvaders written in *R*ust emulating *I*ntel*8080**

Yes i really wanted to use the acronym SIRI8080 and yes that is a horrible title name. 

# Intel 8080

Short (relatively) and sweet (if emulation is your jam) reference manual for the Intel 8080

## Annotations/Abbreviations

* SP - Stack Pointer
* PC - Program Counter
* MS(B) - Most Significant (Byte)
* LS(B) - Least Significant (Byte)
* OP(C) - Operation (Code) / instruction
* **Flags**
  - C:  Carry flag
  - A:  Auxiliary carry bit flag
  - S:  Sign bit flag
  - Z:  Zero bit flag
  - P:  Parity bit flag
* reg - register
* mem - memory
* reset - setting bit to zero
* set - setting bit to one

## Overview

The intel 8080 consists of the following parts

- Seven registers
- Memory
- Program counter
- Stack pointer 
- I/O

## Working registers

Seven registers:
* **A** - accumulator
* **B** - 'scratchpad' register
* **C** - 'scratchpad' register
* **D** - 'scratchpad' register
* **E** - 'scratchpad' register
* **H** - Common use: MSB of Address
* **L** - Common use: LSB of Address
 

## Memory

* Size: 65 KB memory, 0x00000 - 0xFFFFH
* Address length: 16 Bits

Memory Addressing Modes:
- **Direct Addressing**: Instruction supplies the exact memory address
- **Register Pair Addressing**: A register contains the address. Register H
  contains most significant Byte, L contains lowest significant Byte.
- **Stack Pointer Addressing**: The stack pointer address is used. See **pop/push** in
  the "Stack Pointer" section.
- **Immediate Addressing**: Loads next byte (Byte after instruction byte) into the
  **A** register.


## Status Flags/Bits (sometimes called status register)

Bits that have special representation. Each individual bit represents a _flag_. 

* **Carry Bit**: Affected by:
  - addition
  - subtraction
  - rotation
  - logical OP
* **Auxiliary Carry Bit**: Indicates overflow (carry out) of bit 3. Special bit only for
 instruction DAA (TODO) and cannot be tested. 
* **Sign Bit**: A byte cab be represented as Two complement, if __bit 7 is set_ the numerical
   range is [-128,-1], if _bit 7 is zero_ the range is [0,127]. The **Sign Bit**
   is set to the conditions of bit 7 after certain instructions (TODO).
* **Zero Bit**: Is set if the result is zero for some instructions.
* **Parity Bit**: Is set after certain operations depending of parity. Parity
means if there's odd or even number of set bits in s byte. Flag is *set if odd*
and *0 if even*.

## Program Counter (PC)

The program counter is a 16 bit register. Contains address of next instruction
to execute.

## Instructions 

Instructions of the same _family_ can be determined by masking out the
instructions bits. For example the **Immediate Move** or **MVI** _family_ have a total
of 8 instructions with bit pattern `00XXX110` where `XXX` determines which MVI
instructions are called.

### Register bits 

When the instructions reference registers some specific bits in the instruction
determines which register is to be used. 

#### Single Register reference
When a single register is referenced by three bits it is denoted by `XXX` or `YYY` in
this documentation. Translation of bits `XXX` or `YYY` to a register:
- 000: B
- 001: C
- 010: D
- 011: E
- 100: H
- 101: L
- 110: memory reference 
- 111: A

#### Register Pair reference
When a register pair is referenced by two bits it is denoted by `XX` or `YY`. Translation to register:
- 00 - *B* and *C*
- 01 - *D* and *E*
- 10 - *H* and *L*
- 11 - Flags and *A*

#### Families
* *Carry Bit Instructions*: operates directly on the carry flag. Two instructions.
  * ```0011X111``` **Set/Clear**
* *Single Register Instructions*. Operates on single registers. If a memory reference is specified, the address is specified by register **H** and **L**
* *Data Transfer Instructions*
  * ```01XXXYYY```  **MOV** move byte to `XXX` from `YYY`. See [reg. ref.](#single-register). If XXX is equal to YYY it counts as a **NOP**
  * ```000XY010```  **(ST/LD)AX** Store load accumulator from/to address specified by MSB *H* and LSB *L*
*  *Register/Memory to Accumulator Instructions*. operations on the accumulator using one byte fetched from a register or memory address. 
   * ```10XXXYYY``` **Arithmetic/Logic** Where `XXX` is OP and `YYY` is register. For `YYY` see [here](#single-register). `XXX`: 
* *Register Pair Instructions* operates on pair of instructions. See [ref](#register-pair-reference).
* *Immediate* 
  * ```00XX0001``` **LXI** Load register XX with two next bytes, instruction bits.
  * ```00XXX110``` **MVI** Load register X with next byte, instructions bits  [reg. ref.](#single-register).
  * ```11XXX110``` **Arithmetic/Logic** Instructions: Operates on the accumulator (reg. **A**) with
    the next byte. Instructions bits
* *Direct Addressing instructions* OP's that reference memory by the next two bytes.
  * ```001XX010``` **STA/LDA/SHLD/LHLD** `XX` denotes OP, second byte is low address, third byte is high address.
* *Jump Instructions* Occupies one or Three bytes.
  * ```11XXX01Y``` 8 different jump instructions, denoted by XX. Y: 1 for jump, 0 otherwise..
* *Call Subroutine instructions* Occupies 3 bytes. Second byte for low address and third for high address
  * ```11XXX10Y``` XXX denotes one of 8 functions. Y: 1 for CALL (special OP), 0 otherwise
* *RET functions* return from subroutines.
  * ```11XXX00X```XXX denotes one of 8 functions. Y: 1 for RET (special OP), 0 otherwise

### Required by Space Invaders

| Machine Code | Assembly    | Cycles | Flags           | Function                                        |
|--------------|-------------|--------|-----------------|-------------------------------------------------|
| 0x00         | NOP         | 1      |                 |                                                 |
| 0x01         | LXI B,D16   | 3      |                 | B <- byte 3, C <- byte 2                        |
| 0x05         | DCR B       | 1      | Z, S, P, AC     | B <- B-1                                        |
| 0x06         | MVI B, D8   | 2      |                 | B <- byte 2                                     |
| 0x09         | DAD B       | 1      | CY              | HL = HL + BC                                    |
| 0x0d         | DCR C       | 1      | Z, S, P, AC     | C <-C-1                                         |
| 0x0e         | MVI C,D8    | 2      |                 | C <- byte 2                                     |
| 0x0f         | RRC         | 1      | CY              | A = A >> 1; bit 7 = prev bit 0; CY = prev bit 0 |
| 0x11         | LXI D,D16   | 3      |                 | D <- byte 3, E <- byte 2                        |
| 0x13         | INX D       | 1      |                 | DE <- DE + 1                                    |
| 0x19         | DAD D       | 1      | CY              | HL = HL + DE                                    |
| 0x1a         | LDAX D      | 1      |                 | A <- (DE)                                       |
| 0x21         | LXI H,D16   | 3      |                 | H <- byte 3, L <- byte 2                        |
| 0x23         | INX H       | 1      |                 | HL <- HL + 1                                    |
| 0x26         | MVI H,D8    | 2      |                 | H <- byte 2                                     |
| 0x29         | DAD H       | 1      | CY              | HL = HL + HI                                    |
| 0x31         | LXI SP, D16 | 3      |                 | SP.hi <- byte 3, SP.lo <- byte 2                |
| 0x32         | STA adr     | 3      |                 | (adr) <- A                                      |
| 0x36         | MVI M,D8    | 2      |                 | (HL) <- byte 2                                  |
| 0x3a         | LDA adr     | 3      |                 | A <- (adr)                                      |
| 0x3e         | MVI A,D8    | 2      |                 | A <- byte 2                                     |
| 0x56         | MOV D,M     | 1      |                 | D <- (HL)                                       |
| 0x5e         | MOV E,M     | 1      |                 | E <- (HL)                                       |
| 0x66         | MOV H,M     | 1      |                 | H <- (HL)                                       |
| 0x6f         | MOV L,A     | 1      |                 | L <- A                                          |
| 0x77         | MOV M,A     | 1      |                 | (HL) <- A                                       |
| 0x7a         | MOV A,D     | 1      |                 | A <- D                                          |
| 0x7b         | MOV A,E     | 1      |                 | A <- E                                          |
| 0x7c         | MOV A,H     | 1      |                 | A <- H                                          |
| 0x7e         | MOV A,M     | 1      |                 | A <- (HL)                                       |
| 0xa7         | ANA A       | 1      | Z, S, P, CY, AC | A <- A & A                                      |
| 0xaf         | XRA A       | 1      | Z, S, P, CY, AC | A <- A ^ A                                      |
| 0xc1         | POP B       | 1      |                 | C <- (sp); B <- (sp+1); sp <- sp+2              |
| 0xc2         | JNZ adr     | 3      |                 | if NZ, PC <- adr                                |
| 0xc3         | JMP adr     | 3      |                 | PC <= adr                                       |
| 0xc5         | PUSH B      | 1      |                 | (sp-2)<-C; (sp-1)<-B; sp <- sp - 2              |
| 0xc6         | ADI D8      | 2      | Z, S, P, CY, AC | A <- A + byte                                   |
| 0xc9         | RET         | 1      |                 | PC.lo <- (sp); PC.hi<-(sp+1); SP <- SP+2        |
| 0xcd         | CALL adr    | 3      |                 | (SP-1)<-PC.hi;(SP-2)<-PC.lo;SP<-SP-2;PC=adr     |
| 0xd1         | POP D       | 1      |                 | E <- (sp); D <- (sp+1); sp <- sp+2              |
| 0xd3         | OUT D8      | 2      |                 | special                                         |
| 0xd5         | PUSH D      | 1      |                 | (sp-2)<-E; (sp-1)<-D; sp <- sp - 2              |
| 0xe1         | POP H       | 1      |                 | L <- (sp); H <- (sp+1); sp <- sp+2              |
| 0xe5         | PUSH H      | 1      |                 | (sp-2)<-L; (sp-1)<-H; sp <- sp - 2              |
| 0xe6         | ANI D8      | 2      | Z, S, P, CY, AC | A <- A & data                                   |
| 0xeb         | XCHG        | 1      |                 | H <-> D; L <-> E                                |
| 0xf1         | POP PSW     | 1      |                 | flags <- (sp); A <- (sp+1); sp <- sp+2          |
| 0xf5         | PUSH PSW    | 1      |                 | (sp-2)<-flags; (sp-1)<-A; sp <- sp - 2          |
| 0xfb         | EI          | 1      |                 | special                                         |
| 0xfe         | CPI D8      | 2      | Z, S, P, CY, AC | A - data                                        |

### All Instructions

### All Instructions

| Machine Code HEX | Bit      | Assembly    | Cycles | Flags           | Function                                        |
|------------------|----------|-------------|--------|-----------------|-------------------------------------------------|
| 0x00             | 00000000 | NOP         | 1      |                 |                                                 |
| 0x01             | 00000001 | LXI B,D16   | 3      |                 | B <- byte 3, C <- byte 2                        |
| 0x02             | 00000010 | STAX B      | 1      |                 | (BC) <- A                                       |
| 0x03             | 00000011 | INX B       | 1      |                 | BC <- BC+1                                      |
| 0x04             | 00000100 | INR B       | 1      | Z, S, P, AC     | B <- B+1                                        |
| 0x05             | 00000101 | DCR B       | 1      | Z, S, P, AC     | B <- B-1                                        |
| 0x06             | 00000110 | MVI B, D8   | 2      |                 | B <- byte 2                                     |
| 0x07             | 00000111 | RLC         | 1      | CY              | A = A << 1; bit 0 = prev bit 7; CY = prev bit 7 |
| 0x08             | 00001000 | -           |        |                 |                                                 |
| 0x09             | 00001001 | DAD B       | 1      | CY              | HL = HL + BC                                    |
| 0x0A             | 00001010 | LDAX B      | 1      |                 | A <- (BC)                                       |
| 0x0B             | 00001011 | DCX B       | 1      |                 | BC = BC-1                                       |
| 0x0C             | 00001100 | INR C       | 1      | Z, S, P, AC     | C <- C+1                                        |
| 0x0D             | 00001101 | DCR C       | 1      | Z, S, P, AC     | C <-C-1                                         |
| 0x0E             | 00001110 | MVI C,D8    | 2      |                 | C <- byte 2                                     |
| 0x0F             | 00001111 | RRC         | 1      | CY              | A = A >> 1; bit 7 = prev bit 0; CY = prev bit 0 |
| 0x10             | 00010000 | -           |        |                 |                                                 |
| 0x11             | 00010001 | LXI D,D16   | 3      |                 | D <- byte 3, E <- byte 2                        |
| 0x12             | 00010010 | STAX D      | 1      |                 | (DE) <- A                                       |
| 0x13             | 00010011 | INX D       | 1      |                 | DE <- DE + 1                                    |
| 0x14             | 00010100 | INR D       | 1      | Z, S, P, AC     | D <- D+1                                        |
| 0x15             | 00010101 | DCR D       | 1      | Z, S, P, AC     | D <- D-1                                        |
| 0x16             | 00010110 | MVI D, D8   | 2      |                 | D <- byte 2                                     |
| 0x17             | 00010111 | RAL         | 1      | CY              | A = A << 1; bit 0 = prev CY; CY = prev bit 7    |
| 0x18             | 00011000 | -           |        |                 |                                                 |
| 0x19             | 00011001 | DAD D       | 1      | CY              | HL = HL + DE                                    |
| 0x1A             | 00011010 | LDAX D      | 1      |                 | A <- (DE)                                       |
| 0x1B             | 00011011 | DCX D       | 1      |                 | DE = DE-1                                       |
| 0x1C             | 00011100 | INR E       | 1      | Z, S, P, AC     | E <-E+1                                         |
| 0x1D             | 00011101 | DCR E       | 1      | Z, S, P, AC     | E <- E-1                                        |
| 0x1E             | 00011110 | MVI E,D8    | 2      |                 | E <- byte 2                                     |
| 0x1F             | 00011111 | RAR         | 1      | CY              | A = A >> 1; bit 7 = prev bit 7; CY = prev bit 0 |
| 0x20             | 00100000 | -           |        |                 |                                                 |
| 0x21             | 00100001 | LXI H,D16   | 3      |                 | H <- byte 3, L <- byte 2                        |
| 0x22             | 00100010 | SHLD adr    | 3      |                 | (adr) <-L; (adr+1)<-H                           |
| 0x23             | 00100011 | INX H       | 1      |                 | HL <- HL + 1                                    |
| 0x24             | 00100100 | INR H       | 1      | Z, S, P, AC     | H <- H+1                                        |
| 0x25             | 00100101 | DCR H       | 1      | Z, S, P, AC     | H <- H-1                                        |
| 0x26             | 00100110 | MVI H,D8    | 2      |                 | H <- byte 2                                     |
| 0x27             | 00100111 | DAA         | 1      |                 | special                                         |
| 0x28             | 00101000 | -           |        |                 |                                                 |
| 0x29             | 00101001 | DAD H       | 1      | CY              | HL = HL + HI                                    |
| 0x2A             | 00101010 | LHLD adr    | 3      |                 | L <- (adr); H<-(adr+1)                          |
| 0x2B             | 00101011 | DCX H       | 1      |                 | HL = HL-1                                       |
| 0x2C             | 00101100 | INR L       | 1      | Z, S, P, AC     | L <- L+1                                        |
| 0x2D             | 00101101 | DCR L       | 1      | Z, S, P, AC     | L <- L-1                                        |
| 0x2E             | 00101110 | MVI L, D8   | 2      |                 | L <- byte 2                                     |
| 0x2F             | 00101111 | CMA         | 1      |                 | A <- !A                                         |
| 0x30             | 00110000 | -           |        |                 |                                                 |
| 0x31             | 00110001 | LXI SP, D16 | 3      |                 | SP.hi <- byte 3, SP.lo <- byte 2                |
| 0x32             | 00110010 | STA adr     | 3      |                 | (adr) <- A                                      |
| 0x33             | 00110011 | INX SP      | 1      |                 | SP = SP + 1                                     |
| 0x34             | 00110100 | INR M       | 1      | Z, S, P, AC     | (HL) <- (HL)+1                                  |
| 0x35             | 00110101 | DCR M       | 1      | Z, S, P, AC     | (HL) <- (HL)-1                                  |
| 0x36             | 00110110 | MVI M,D8    | 2      |                 | (HL) <- byte 2                                  |
| 0x37             | 00110111 | STC         | 1      | CY              | CY = 1                                          |
| 0x38             | 00111000 | -           |        |                 |                                                 |
| 0x39             | 00111001 | DAD SP      | 1      | CY              | HL = HL + SP                                    |
| 0x3A             | 00111010 | LDA adr     | 3      |                 | A <- (adr)                                      |
| 0x3B             | 00111011 | DCX SP      | 1      |                 | SP = SP-1                                       |
| 0x3C             | 00111100 | INR A       | 1      | Z, S, P, AC     | A <- A+1                                        |
| 0x3D             | 00111101 | DCR A       | 1      | Z, S, P, AC     | A <- A-1                                        |
| 0x3E             | 00111110 | MVI A,D8    | 2      |                 | A <- byte 2                                     |
| 0x3F             | 00111111 | CMC         | 1      | CY              | CY=!CY                                          |
| 0x40             | 01000000 | MOV B,B     | 1      |                 | B <- B                                          |
| 0x41             | 01000001 | MOV B,C     | 1      |                 | B <- C                                          |
| 0x42             | 01000010 | MOV B,D     | 1      |                 | B <- D                                          |
| 0x43             | 01000011 | MOV B,E     | 1      |                 | B <- E                                          |
| 0x44             | 01000100 | MOV B,H     | 1      |                 | B <- H                                          |
| 0x45             | 01000101 | MOV B,L     | 1      |                 | B <- L                                          |
| 0x46             | 01000110 | MOV B,M     | 1      |                 | B <- (HL)                                       |
| 0x47             | 01000111 | MOV B,A     | 1      |                 | B <- A                                          |
| 0x48             | 01001000 | MOV C,B     | 1      |                 | C <- B                                          |
| 0x49             | 01001001 | MOV C,C     | 1      |                 | C <- C                                          |
| 0x4A             | 01001010 | MOV C,D     | 1      |                 | C <- D                                          |
| 0x4B             | 01001011 | MOV C,E     | 1      |                 | C <- E                                          |
| 0x4C             | 01001100 | MOV C,H     | 1      |                 | C <- H                                          |
| 0x4D             | 01001101 | MOV C,L     | 1      |                 | C <- L                                          |
| 0x4E             | 01001110 | MOV C,M     | 1      |                 | C <- (HL)                                       |
| 0x4F             | 01001111 | MOV C,A     | 1      |                 | C <- A                                          |
| 0x50             | 01010000 | MOV D,B     | 1      |                 | D <- B                                          |
| 0x51             | 01010001 | MOV D,C     | 1      |                 | D <- C                                          |
| 0x52             | 01010010 | MOV D,D     | 1      |                 | D <- D                                          |
| 0x53             | 01010011 | MOV D,E     | 1      |                 | D <- E                                          |
| 0x54             | 01010100 | MOV D,H     | 1      |                 | D <- H                                          |
| 0x55             | 01010101 | MOV D,L     | 1      |                 | D <- L                                          |
| 0x56             | 01010110 | MOV D,M     | 1      |                 | D <- (HL)                                       |
| 0x57             | 01010111 | MOV D,A     | 1      |                 | D <- A                                          |
| 0x58             | 01011000 | MOV E,B     | 1      |                 | E <- B                                          |
| 0x59             | 01011001 | MOV E,C     | 1      |                 | E <- C                                          |
| 0x5A             | 01011010 | MOV E,D     | 1      |                 | E <- D                                          |
| 0x5B             | 01011011 | MOV E,E     | 1      |                 | E <- E                                          |
| 0x5C             | 01011100 | MOV E,H     | 1      |                 | E <- H                                          |
| 0x5D             | 01011101 | MOV E,L     | 1      |                 | E <- L                                          |
| 0x5E             | 01011110 | MOV E,M     | 1      |                 | E <- (HL)                                       |
| 0x5F             | 01011111 | MOV E,A     | 1      |                 | E <- A                                          |
| 0x60             | 01100000 | MOV H,B     | 1      |                 | H <- B                                          |
| 0x61             | 01100001 | MOV H,C     | 1      |                 | H <- C                                          |
| 0x62             | 01100010 | MOV H,D     | 1      |                 | H <- D                                          |
| 0x63             | 01100011 | MOV H,E     | 1      |                 | H <- E                                          |
| 0x64             | 01100100 | MOV H,H     | 1      |                 | H <- H                                          |
| 0x65             | 01100101 | MOV H,L     | 1      |                 | H <- L                                          |
| 0x66             | 01100110 | MOV H,M     | 1      |                 | H <- (HL)                                       |
| 0x67             | 01100111 | MOV H,A     | 1      |                 | H <- A                                          |
| 0x68             | 01101000 | MOV L,B     | 1      |                 | L <- B                                          |
| 0x69             | 01101001 | MOV L,C     | 1      |                 | L <- C                                          |
| 0x6A             | 01101010 | MOV L,D     | 1      |                 | L <- D                                          |
| 0x6B             | 01101011 | MOV L,E     | 1      |                 | L <- E                                          |
| 0x6C             | 01101100 | MOV L,H     | 1      |                 | L <- H                                          |
| 0x6D             | 01101101 | MOV L,L     | 1      |                 | L <- L                                          |
| 0x6E             | 01101110 | MOV L,M     | 1      |                 | L <- (HL)                                       |
| 0x6F             | 01101111 | MOV L,A     | 1      |                 | L <- A                                          |
| 0x70             | 01110000 | MOV M,B     | 1      |                 | (HL) <- B                                       |
| 0x71             | 01110001 | MOV M,C     | 1      |                 | (HL) <- C                                       |
| 0x72             | 01110010 | MOV M,D     | 1      |                 | (HL) <- D                                       |
| 0x73             | 01110011 | MOV M,E     | 1      |                 | (HL) <- E                                       |
| 0x74             | 01110100 | MOV M,H     | 1      |                 | (HL) <- H                                       |
| 0x75             | 01110101 | MOV M,L     | 1      |                 | (HL) <- L                                       |
| 0x76             | 01110110 | HLT         | 1      |                 | special                                         |
| 0x77             | 01110111 | MOV M,A     | 1      |                 | (HL) <- A                                       |
| 0x78             | 01111000 | MOV A,B     | 1      |                 | A <- B                                          |
| 0x79             | 01111001 | MOV A,C     | 1      |                 | A <- C                                          |
| 0x7A             | 01111010 | MOV A,D     | 1      |                 | A <- D                                          |
| 0x7B             | 01111011 | MOV A,E     | 1      |                 | A <- E                                          |
| 0x7C             | 01111100 | MOV A,H     | 1      |                 | A <- H                                          |
| 0x7D             | 01111101 | MOV A,L     | 1      |                 | A <- L                                          |
| 0x7E             | 01111110 | MOV A,M     | 1      |                 | A <- (HL)                                       |
| 0x7F             | 01111111 | MOV A,A     | 1      |                 | A <- A                                          |
| 0xER             | 11101111 | ERR         | -1     | C               | Take a break m8                                 |
| 0x80             | 10000000 | ADD B       | 1      | Z, S, P, CY, AC | A <- A + B                                      |
| 0x81             | 10000001 | ADD C       | 1      | Z, S, P, CY, AC | A <- A + C                                      |
| 0x82             | 10000010 | ADD D       | 1      | Z, S, P, CY, AC | A <- A + D                                      |
| 0x83             | 10000011 | ADD E       | 1      | Z, S, P, CY, AC | A <- A + E                                      |
| 0x84             | 10000100 | ADD H       | 1      | Z, S, P, CY, AC | A <- A + H                                      |
| 0x85             | 10000101 | ADD L       | 1      | Z, S, P, CY, AC | A <- A + L                                      |
| 0x86             | 10000110 | ADD M       | 1      | Z, S, P, CY, AC | A <- A + (HL)                                   |
| 0x87             | 10000111 | ADD A       | 1      | Z, S, P, CY, AC | A <- A + A                                      |
| 0x88             | 10001000 | ADC B       | 1      | Z, S, P, CY, AC | A <- A + B + CY                                 |
| 0x89             | 10001001 | ADC C       | 1      | Z, S, P, CY, AC | A <- A + C + CY                                 |
| 0x8A             | 10001010 | ADC D       | 1      | Z, S, P, CY, AC | A <- A + D + CY                                 |
| 0x8B             | 10001011 | ADC E       | 1      | Z, S, P, CY, AC | A <- A + E + CY                                 |
| 0x8C             | 10001100 | ADC H       | 1      | Z, S, P, CY, AC | A <- A + H + CY                                 |
| 0x8D             | 10001101 | ADC L       | 1      | Z, S, P, CY, AC | A <- A + L + CY                                 |
| 0x8E             | 10001110 | ADC M       | 1      | Z, S, P, CY, AC | A <- A + (HL) + CY                              |
| 0x8F             | 10001111 | ADC A       | 1      | Z, S, P, CY, AC | A <- A + A + CY                                 |
| 0x90             | 10010000 | SUB B       | 1      | Z, S, P, CY, AC | A <- A - B                                      |
| 0x91             | 10010001 | SUB C       | 1      | Z, S, P, CY, AC | A <- A - C                                      |
| 0x92             | 10010010 | SUB D       | 1      | Z, S, P, CY, AC | A <- A + D                                      |
| 0x93             | 10010011 | SUB E       | 1      | Z, S, P, CY, AC | A <- A - E                                      |
| 0x94             | 10010100 | SUB H       | 1      | Z, S, P, CY, AC | A <- A + H                                      |
| 0x95             | 10010101 | SUB L       | 1      | Z, S, P, CY, AC | A <- A - L                                      |
| 0x96             | 10010110 | SUB M       | 1      | Z, S, P, CY, AC | A <- A + (HL)                                   |
| 0x97             | 10010111 | SUB A       | 1      | Z, S, P, CY, AC | A <- A - A                                      |
| 0x98             | 10011000 | SBB B       | 1      | Z, S, P, CY, AC | A <- A - B - CY                                 |
| 0x99             | 10011001 | SBB C       | 1      | Z, S, P, CY, AC | A <- A - C - CY                                 |
| 0x9A             | 10011010 | SBB D       | 1      | Z, S, P, CY, AC | A <- A - D - CY                                 |
| 0x9B             | 10011011 | SBB E       | 1      | Z, S, P, CY, AC | A <- A - E - CY                                 |
| 0x9C             | 10011100 | SBB H       | 1      | Z, S, P, CY, AC | A <- A - H - CY                                 |
| 0x9D             | 10011101 | SBB L       | 1      | Z, S, P, CY, AC | A <- A - L - CY                                 |
| 0x9E             | 10011110 | SBB M       | 1      | Z, S, P, CY, AC | A <- A - (HL) - CY                              |
| 0x9F             | 10011111 | SBB A       | 1      | Z, S, P, CY, AC | A <- A - A - CY                                 |
| 0xA0             | 10100000 | ANA B       | 1      | Z, S, P, CY, AC | A <- A & B                                      |
| 0xA1             | 10100001 | ANA C       | 1      | Z, S, P, CY, AC | A <- A & C                                      |
| 0xA2             | 10100010 | ANA D       | 1      | Z, S, P, CY, AC | A <- A & D                                      |
| 0xA3             | 10100011 | ANA E       | 1      | Z, S, P, CY, AC | A <- A & E                                      |
| 0xA4             | 10100100 | ANA H       | 1      | Z, S, P, CY, AC | A <- A & H                                      |
| 0xA5             | 10100101 | ANA L       | 1      | Z, S, P, CY, AC | A <- A & L                                      |
| 0xA6             | 10100110 | ANA M       | 1      | Z, S, P, CY, AC | A <- A & (HL)                                   |
| 0xA7             | 10100111 | ANA A       | 1      | Z, S, P, CY, AC | A <- A & A                                      |
| 0xA8             | 10101000 | XRA B       | 1      | Z, S, P, CY, AC | A <- A ^ B                                      |
| 0xA9             | 10101001 | XRA C       | 1      | Z, S, P, CY, AC | A <- A ^ C                                      |
| 0xAA             | 10101010 | XRA D       | 1      | Z, S, P, CY, AC | A <- A ^ D                                      |
| 0xAB             | 10101011 | XRA E       | 1      | Z, S, P, CY, AC | A <- A ^ E                                      |
| 0xAC             | 10101100 | XRA H       | 1      | Z, S, P, CY, AC | A <- A ^ H                                      |
| 0xAD             | 10101101 | XRA L       | 1      | Z, S, P, CY, AC | A <- A ^ L                                      |
| 0xAE             | 10101110 | XRA M       | 1      | Z, S, P, CY, AC | A <- A ^ (HL)                                   |
| 0xAF             | 10101111 | XRA A       | 1      | Z, S, P, CY, AC | A <- A ^ A                                      |
| 0xB0             | 10110000 | ORA B       | 1      | Z, S, P, CY, AC | A <- A                                          |
| 0xB1             | 10110001 | ORA C       | 1      | Z, S, P, CY, AC | A <- A                                          |
| 0xB2             | 10110010 | ORA D       | 1      | Z, S, P, CY, AC | A <- A                                          |
| 0xB3             | 10110011 | ORA E       | 1      | Z, S, P, CY, AC | A <- A                                          |
| 0xB4             | 10110100 | ORA H       | 1      | Z, S, P, CY, AC | A <- A                                          |
| 0xB5             | 10110101 | ORA L       | 1      | Z, S, P, CY, AC | A <- A                                          |
| 0xB6             | 10110110 | ORA M       | 1      | Z, S, P, CY, AC | A <- A                                          |
| 0xB7             | 10110111 | ORA A       | 1      | Z, S, P, CY, AC | A <- A                                          |
| 0xB8             | 10111000 | CMP B       | 1      | Z, S, P, CY, AC | A - B                                           |
| 0xB9             | 10111001 | CMP C       | 1      | Z, S, P, CY, AC | A - C                                           |
| 0xBA             | 10111010 | CMP D       | 1      | Z, S, P, CY, AC | A - D                                           |
| 0xBB             | 10111011 | CMP E       | 1      | Z, S, P, CY, AC | A - E                                           |
| 0xBC             | 10111100 | CMP H       | 1      | Z, S, P, CY, AC | A - H                                           |
| 0xBD             | 10111101 | CMP L       | 1      | Z, S, P, CY, AC | A - L                                           |
| 0xBE             | 10111110 | CMP M       | 1      | Z, S, P, CY, AC | A - (HL)                                        |
| 0xBF             | 10111111 | CMP A       | 1      | Z, S, P, CY, AC | A - A                                           |
| 0xC0             | 11000000 | RNZ         | 1      |                 | if NZ, RET                                      |
| 0xC1             | 11000001 | POP B       | 1      |                 | C <- (sp); B <- (sp+1); sp <- sp+2              |
| 0xC2             | 11000010 | JNZ adr     | 3      |                 | if NZ, PC <- adr                                |
| 0xC3             | 11000011 | JMP adr     | 3      |                 | PC <= adr                                       |
| 0xC4             | 11000100 | CNZ adr     | 3      |                 | if NZ, CALL adr                                 |
| 0xC5             | 11000101 | PUSH B      | 1      |                 | (sp-2)<-C; (sp-1)<-B; sp <- sp - 2              |
| 0xC6             | 11000110 | ADI D8      | 2      | Z, S, P, CY, AC | A <- A + byte                                   |
| 0xC7             | 11000111 | RST 0       | 1      |                 | CALL \\$0                                       |
| 0xC8             | 11001000 | RZ          | 1      |                 | if Z, RET                                       |
| 0xC9             | 11001001 | RET         | 1      |                 | PC.lo <- (sp); PC.hi<-(sp+1); SP <- SP+2        |
| 0xCA             | 11001010 | JZ adr      | 3      |                 | if Z, PC <- adr                                 |
| 0xCB             | 11001011 | -           |        |                 |                                                 |
| 0xCC             | 11001100 | CZ adr      | 3      |                 | if Z, CALL adr                                  |
| 0xCD             | 11001101 | CALL adr    | 3      |                 | (SP-1)<-PC.hi;(SP-2)<-PC.lo;SP<-SP-2;PC=adr     |
| 0xCE             | 11001110 | ACI D8      | 2      | Z, S, P, CY, AC | A <- A + data + CY                              |
| 0xCF             | 11001111 | RST 1       | 1      |                 | CALL \$8                                        |
| 0xD0             | 11010000 | RNC         | 1      |                 | if NCY, RET                                     |
| 0xD1             | 11010001 | POP D       | 1      |                 | E <- (sp); D <- (sp+1); sp <- sp+2              |
| 0xD2             | 11010010 | JNC adr     | 3      |                 | if NCY, PC<-adr                                 |
| 0xD3             | 11010011 | OUT D8      | 2      |                 | special                                         |
| 0xD4             | 11010100 | CNC adr     | 3      |                 | if NCY, CALL adr                                |
| 0xD5             | 11010101 | PUSH D      | 1      |                 | (sp-2)<-E; (sp-1)<-D; sp <- sp - 2              |
| 0xD6             | 11010110 | SUI D8      | 2      | Z, S, P, CY, AC | A <- A - data                                   |
| 0xD7             | 11010111 | RST 2       | 1      |                 | CALL \$10                                       |
| 0xD8             | 11011000 | RC          | 1      |                 | if CY, RET                                      |
| 0xD9             | 11011001 | -           |        |                 |                                                 |
| 0xDA             | 11011010 | JC adr      | 3      |                 | if CY, PC<-adr                                  |
| 0xDB             | 11011011 | IN D8       | 2      |                 | special                                         |
| 0xDC             | 11011100 | CC adr      | 3      |                 | if CY, CALL adr                                 |
| 0xDD             | 11011101 | -           |        |                 |                                                 |
| 0xDE             | 11011110 | SBI D8      | 2      | Z, S, P, CY, AC | A <- A - data - CY                              |
| 0xDF             | 11011111 | RST 3       | 1      |                 | CALL \$18                                       |
| 0xE0             | 11100000 | RPO         | 1      |                 | if PO, RET                                      |
| 0xE1             | 11100001 | POP H       | 1      |                 | L <- (sp); H <- (sp+1); sp <- sp+2              |
| 0xE2             | 11100010 | JPO adr     | 3      |                 | if PO, PC <- adr                                |
| 0xE3             | 11100011 | XTHL        | 1      |                 | L <-> (SP); H <-> (SP+1)                        |
| 0xE4             | 11100100 | CPO adr     | 3      |                 | if PO, CALL adr                                 |
| 0xE5             | 11100101 | PUSH H      | 1      |                 | (sp-2)<-L; (sp-1)<-H; sp <- sp - 2              |
| 0xE6             | 11100110 | ANI D8      | 2      | Z, S, P, CY, AC | A <- A & data                                   |
| 0xE7             | 11100111 | RST 4       | 1      |                 | CALL \$20                                       |
| 0xE8             | 11101000 | RPE         | 1      |                 | if PE, RET                                      |
| 0xE9             | 11101001 | PCHL        | 1      |                 | PC.hi <- H; PC.lo <- L                          |
| 0xEA             | 11101010 | JPE adr     | 3      |                 | if PE, PC <- adr                                |
| 0xEB             | 11101011 | XCHG        | 1      |                 | H <-> D; L <-> E                                |
| 0xEC             | 11101100 | CPE adr     | 3      |                 | if PE, CALL adr                                 |
| 0xED             | 11101101 | -           |        |                 |                                                 |
| 0xEE             | 11101110 | XRI D8      | 2      | Z, S, P, CY, AC | A <- A ^ data                                   |
| 0xEF             | 11101111 | RST 5       | 1      |                 | CALL \$28                                       |
| 0xF0             | 11110000 | RP          | 1      |                 | if P, RET                                       |
| 0xF1             | 11110001 | POP PSW     | 1      |                 | flags <- (sp); A <- (sp+1); sp <- sp+2          |
| 0xF2             | 11110010 | JP adr      | 3      |                 | if P=1 PC <- adr                                |
| 0xF3             | 11110011 | DI          | 1      |                 | special                                         |
| 0xF4             | 11110100 | CP adr      | 3      |                 | if P, PC <- adr                                 |
| 0xF5             | 11110101 | PUSH PSW    | 1      |                 | (sp-2)<-flags; (sp-1)<-A; sp <- sp - 2          |
| 0xF6             | 11110110 | ORI D8      | 2      | Z, S, P, CY, AC | A <- A                                  data    |
| 0xF7             | 11110111 | RST 6       | 1      |                 | CALL \$30                                       |
| 0xF8             | 11111000 | RM          | 1      |                 | if M, RET                                       |
| 0xF9             | 11111001 | SPHL        | 1      |                 | SP=HL                                           |
| 0xFA             | 11111010 | JM adr      | 3      |                 | if M, PC <- adr                                 |
| 0xFB             | 11111011 | EI          | 1      |                 | special                                         |
| 0xFC             | 11111100 | CM adr      | 3      |                 | if M, CALL adr                                  |
| 0xFD             | 11111101 | -           |        |                 |                                                 |
| 0xFE             | 11111110 | CPI D8      | 2      | Z, S, P, CY, AC | A - data                                        |
| 0xFF             | 11111111 | RST 7       | 1      |                 | CALL \$38                                       |

