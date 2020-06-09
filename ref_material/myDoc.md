
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

#### Single Register
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


### Families

#### Carry Bit Instructions 
```0011X111```
operates directly on the carry flag

```0```: 
STC _set_ carry flag

```1```:
CMC _complement_ (set carry flag to its opposite value)

#### Single Register Instructions 
Operates on single registers. If a memory reference is specified, the address is specified by register **H** and **L**

```00XXX100``` 
**INC** Increment instruction. Register or memory is incremented by one. For `XXX` see [here](#single-register).
- _Flags_: Z, S, P, A

```00XXX101``` 
**DCR** Decrement instruction. Decrement register or memory by one. For `XXX` see [here](#single-register). 
_Flags_: Z, S, P, A

```00101111```
**CMA** complement accumulator register, i.e. each bit is changed to its opposite value.
_Flags_: None

```00100111```
**DAA** Decimal adjust accumulator register. Special OP.
1. If value of the LS 4 bits of reg. *A* is greater than 9 or flag A is set, add 6 to value of *A*.
2. If value of the MS 4 bits of reg. *A* is greater than 9 or flag A is set, add 6 to value of *A*. 
  - _Flags_: Z, S, P, C, A 
  - If overflow occurs during (1), flag *A* is set. If overflow occurs during
    (2), flag *C* is set. _NOTE_ that overflow in this case is overflow of
    four bits not the whole byte.
        
        
#### Data Transfer Instructions
```01XXXYYY``` 
 **MOV** move byte to `XXX` from `YYY`. See [reg. ref.](#single-register). If XXX is equal to YYY it counts as a **NOP**
instruction
- _Flags_ None

```000XY010``` 
 **(ST/LD)AX** Store load accumulator from/to address specified by MSB *H* and LSB *L*
- `Y` 0: ST
- `Y` 1: LD
- `X` 0: register pair B and C (LSB)
- `X` 1: register pair D and E (LSB)
- _Flags_ None
   
#### Register/Memory to Accumulator Instructions
operations on the accumulator using one byte fetched from a register or memory address. 

```10XXXYYY``` 
Where `XXX` is OP and `YYY` is register. For `YYY` see [here](#single-register). `XXX`: 

| XXX | OP             | LHS | RHS | FLAGS     | Two's Comp. | Note       |
|-----|----------------|-----|-----|-----------|-------------|------------|
| 000 | ADD            | *A* | YYY | C,S,Z,P,A | Yes         |            |
| 001 | ADD with Carry | *A* | YYY | C,S,Z,P,A | Yes         |            |
| 010 | SUB            | *A* | YYY | C,S,Z,P,A | Yes         |            |
| 011 | SUB with Carry | *A* | YYY | C,S,Z,P,A | Yes         |            |
| 100 | AND            | *A* | YYY | C,S,Z,P   |             | Resets *C* |
| 101 | XOR            | *A* | YYY | C,S,Z,P,A |             | Resets *C* |
| 110 | OR             | *A* | YYY | C,S,Z,P,  |             | Resets *C* |
| 111 | CMP            | *A* | YYY | C,S,Z,P,A |             | *          |

  \* *Z* is set if results is zero otherwise its reset. *C* is set if YYY is greater than *A* otherwise reset.
<!-- * TODO Rotate accumulator instructions -->
<!-- * TODO register pair instructions -->
<!-- * TODO Direct addressing instructions -->
<!-- * TODO Jump instructions -->
<!-- * TODO Call subroutine instructions -->
<!-- * TODO Reset Instruction -->
<!-- * TODO Interrupt flip-flop instructions -->
<!-- * TODO I/O instructions -->
<!-- * TODO Halt instructions -->
<!-- * TODO Pseudo instructions -->

* **Immediate**: Occupies 2-3 bytes. 
  * `00XX0001` **LXI** Load register XX with two next bytes, instruction bits. See []
  | XX | MSB | LSB |
  | 00 | B   | C   |
  | 01 | D   | E   |
  | 10 | H   | L   |
  | 11 | SP  | SP  |
  
```00XXX110```
**MVI** Load register X with next byte, instructions bits  [reg. ref.](#single-register)

```11XXX110```
**Arithmetic/Logic** Instructions: Operates on the accumulator (reg. **A**) with
    the next byte. Instructions bits
| XXX | Assembly | Function       | LHS | RHS | FLAGS     | Two's Comp. | Note       |
|-----|----------|----------------|-----|-----|-----------|-------------|------------|
| 000 | ADI      | ADD            | *A* | YYY | C,S,Z,P,A | Yes         |            |
| 001 | ACI      | ADD with Carry | *A* | YYY | C,S,Z,P,A | Yes         |            |
| 010 | SUI      | SUB            | *A* | YYY | C,S,Z,P,A | Yes         |            |
| 011 | SBI      | SUB with Carry | *A* | YYY | C,S,Z,P,A | Yes         |            |
| 100 | ANI      | AND            | *A* | YYY | C,S,Z,P   |             | Resets *C* |
| 101 | XRI      | XOR            | *A* | YYY | C,S,Z,P,A |             | Resets *C* |
| 110 | ORI      | OR             | *A* | YYY | C,S,Z,P,  |             | Resets *C* |
| 111 | CPI      | CMP            | *A* | YYY | C,S,Z,P,A |             | *          |

### Required by Space Invaders

|--------------|-------------|--------|-----------------|-------------------------------------------------|
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
|--------------|-------------|--------|-----------------|-------------------------------------------------|

### All Instructions

|--------------|-------------|--------|-----------------|-------------------------------------------------|
| Machine Code | Assembly    | Cycles | Flags           | Function                                        |
|--------------|-------------|--------|-----------------|-------------------------------------------------|
| 0X00         | NOP         | 1      |                 |                                                 |
| 0X01         | LXI B,D16   | 3      |                 | B <- byte 3, C <- byte 2                        |
| 0X02         | STAX B      | 1      |                 | (BC) <- A                                       |
| 0X03         | INX B       | 1      |                 | BC <- BC+1                                      |
| 0X04         | INR B       | 1      | Z, S, P, AC     | B <- B+1                                        |
| 0X05         | DCR B       | 1      | Z, S, P, AC     | B <- B-1                                        |
| 0X06         | MVI B, D8   | 2      |                 | B <- byte 2                                     |
| 0X07         | RLC         | 1      | CY              | A = A << 1; bit 0 = prev bit 7; CY = prev bit 7 |
| 0X08         | -           |        |                 |                                                 |
| 0X09         | DAD B       | 1      | CY              | HL = HL + BC                                    |
| 0X0A         | LDAX B      | 1      |                 | A <- (BC)                                       |
| 0X0B         | DCX B       | 1      |                 | BC = BC-1                                       |
| 0X0C         | INR C       | 1      | Z, S, P, AC     | C <- C+1                                        |
| 0X0D         | DCR C       | 1      | Z, S, P, AC     | C <-C-1                                         |
| 0X0E         | MVI C,D8    | 2      |                 | C <- byte 2                                     |
| 0X0F         | RRC         | 1      | CY              | A = A >> 1; bit 7 = prev bit 0; CY = prev bit 0 |
| 0X10         | -           |        |                 |                                                 |
| 0X11         | LXI D,D16   | 3      |                 | D <- byte 3, E <- byte 2                        |
| 0X12         | STAX D      | 1      |                 | (DE) <- A                                       |
| 0X13         | INX D       | 1      |                 | DE <- DE + 1                                    |
| 0X14         | INR D       | 1      | Z, S, P, AC     | D <- D+1                                        |
| 0X15         | DCR D       | 1      | Z, S, P, AC     | D <- D-1                                        |
| 0X16         | MVI D, D8   | 2      |                 | D <- byte 2                                     |
| 0X17         | RAL         | 1      | CY              | A = A << 1; bit 0 = prev CY; CY = prev bit 7    |
| 0X18         | -           |        |                 |                                                 |
| 0X19         | DAD D       | 1      | CY              | HL = HL + DE                                    |
| 0X1A         | LDAX D      | 1      |                 | A <- (DE)                                       |
| 0X1B         | DCX D       | 1      |                 | DE = DE-1                                       |
| 0X1C         | INR E       | 1      | Z, S, P, AC     | E <-E+1                                         |
| 0X1D         | DCR E       | 1      | Z, S, P, AC     | E <- E-1                                        |
| 0X1E         | MVI E,D8    | 2      |                 | E <- byte 2                                     |
| 0X1F         | RAR         | 1      | CY              | A = A >> 1; bit 7 = prev bit 7; CY = prev bit 0 |
| 0X20         | -           |        |                 |                                                 |
| 0X21         | LXI H,D16   | 3      |                 | H <- byte 3, L <- byte 2                        |
| 0X22         | SHLD adr    | 3      |                 | (adr) <-L; (adr+1)<-H                           |
| 0X23         | INX H       | 1      |                 | HL <- HL + 1                                    |
| 0X24         | INR H       | 1      | Z, S, P, AC     | H <- H+1                                        |
| 0X25         | DCR H       | 1      | Z, S, P, AC     | H <- H-1                                        |
| 0X26         | MVI H,D8    | 2      |                 | H <- byte 2                                     |
| 0X27         | DAA         | 1      |                 | special                                         |
| 0X28         | -           |        |                 |                                                 |
| 0X29         | DAD H       | 1      | CY              | HL = HL + HI                                    |
| 0X2A         | LHLD adr    | 3      |                 | L <- (adr); H<-(adr+1)                          |
| 0X2B         | DCX H       | 1      |                 | HL = HL-1                                       |
| 0X2C         | INR L       | 1      | Z, S, P, AC     | L <- L+1                                        |
| 0X2D         | DCR L       | 1      | Z, S, P, AC     | L <- L-1                                        |
| 0X2E         | MVI L, D8   | 2      |                 | L <- byte 2                                     |
| 0X2F         | CMA         | 1      |                 | A <- !A                                         |
| 0X30         | -           |        |                 |                                                 |
| 0X31         | LXI SP, D16 | 3      |                 | SP.hi <- byte 3, SP.lo <- byte 2                |
| 0X32         | STA adr     | 3      |                 | (adr) <- A                                      |
| 0X33         | INX SP      | 1      |                 | SP = SP + 1                                     |
| 0X34         | INR M       | 1      | Z, S, P, AC     | (HL) <- (HL)+1                                  |
| 0X35         | DCR M       | 1      | Z, S, P, AC     | (HL) <- (HL)-1                                  |
| 0X36         | MVI M,D8    | 2      |                 | (HL) <- byte 2                                  |
| 0X37         | STC         | 1      | CY              | CY = 1                                          |
| 0X38         | -           |        |                 |                                                 |
| 0X39         | DAD SP      | 1      | CY              | HL = HL + SP                                    |
| 0X3A         | LDA adr     | 3      |                 | A <- (adr)                                      |
| 0X3B         | DCX SP      | 1      |                 | SP = SP-1                                       |
| 0X3C         | INR A       | 1      | Z, S, P, AC     | A <- A+1                                        |
| 0X3D         | DCR A       | 1      | Z, S, P, AC     | A <- A-1                                        |
| 0X3E         | MVI A,D8    | 2      |                 | A <- byte 2                                     |
| 0X3F         | CMC         | 1      | CY              | CY=!CY                                          |
| 0X40         | MOV B,B     | 1      |                 | B <- B                                          |
| 0X41         | MOV B,C     | 1      |                 | B <- C                                          |
| 0X42         | MOV B,D     | 1      |                 | B <- D                                          |
| 0X43         | MOV B,E     | 1      |                 | B <- E                                          |
| 0X44         | MOV B,H     | 1      |                 | B <- H                                          |
| 0X45         | MOV B,L     | 1      |                 | B <- L                                          |
| 0X46         | MOV B,M     | 1      |                 | B <- (HL)                                       |
| 0X47         | MOV B,A     | 1      |                 | B <- A                                          |
| 0X48         | MOV C,B     | 1      |                 | C <- B                                          |
| 0X49         | MOV C,C     | 1      |                 | C <- C                                          |
| 0X4A         | MOV C,D     | 1      |                 | C <- D                                          |
| 0X4B         | MOV C,E     | 1      |                 | C <- E                                          |
| 0X4C         | MOV C,H     | 1      |                 | C <- H                                          |
| 0X4D         | MOV C,L     | 1      |                 | C <- L                                          |
| 0X4E         | MOV C,M     | 1      |                 | C <- (HL)                                       |
| 0X4F         | MOV C,A     | 1      |                 | C <- A                                          |
| 0X50         | MOV D,B     | 1      |                 | D <- B                                          |
| 0X51         | MOV D,C     | 1      |                 | D <- C                                          |
| 0X52         | MOV D,D     | 1      |                 | D <- D                                          |
| 0X53         | MOV D,E     | 1      |                 | D <- E                                          |
| 0X54         | MOV D,H     | 1      |                 | D <- H                                          |
| 0X55         | MOV D,L     | 1      |                 | D <- L                                          |
| 0X56         | MOV D,M     | 1      |                 | D <- (HL)                                       |
| 0X57         | MOV D,A     | 1      |                 | D <- A                                          |
| 0X58         | MOV E,B     | 1      |                 | E <- B                                          |
| 0X59         | MOV E,C     | 1      |                 | E <- C                                          |
| 0X5A         | MOV E,D     | 1      |                 | E <- D                                          |
| 0X5B         | MOV E,E     | 1      |                 | E <- E                                          |
| 0X5C         | MOV E,H     | 1      |                 | E <- H                                          |
| 0X5D         | MOV E,L     | 1      |                 | E <- L                                          |
| 0X5E         | MOV E,M     | 1      |                 | E <- (HL)                                       |
| 0X5F         | MOV E,A     | 1      |                 | E <- A                                          |
| 0X60         | MOV H,B     | 1      |                 | H <- B                                          |
| 0X61         | MOV H,C     | 1      |                 | H <- C                                          |
| 0X62         | MOV H,D     | 1      |                 | H <- D                                          |
| 0X63         | MOV H,E     | 1      |                 | H <- E                                          |
| 0X64         | MOV H,H     | 1      |                 | H <- H                                          |
| 0X65         | MOV H,L     | 1      |                 | H <- L                                          |
| 0X66         | MOV H,M     | 1      |                 | H <- (HL)                                       |
| 0X67         | MOV H,A     | 1      |                 | H <- A                                          |
| 0X68         | MOV L,B     | 1      |                 | L <- B                                          |
| 0X69         | MOV L,C     | 1      |                 | L <- C                                          |
| 0X6A         | MOV L,D     | 1      |                 | L <- D                                          |
| 0X6B         | MOV L,E     | 1      |                 | L <- E                                          |
| 0X6C         | MOV L,H     | 1      |                 | L <- H                                          |
| 0X6D         | MOV L,L     | 1      |                 | L <- L                                          |
| 0X6E         | MOV L,M     | 1      |                 | L <- (HL)                                       |
| 0X6F         | MOV L,A     | 1      |                 | L <- A                                          |
| 0X70         | MOV M,B     | 1      |                 | (HL) <- B                                       |
| 0X71         | MOV M,C     | 1      |                 | (HL) <- C                                       |
| 0X72         | MOV M,D     | 1      |                 | (HL) <- D                                       |
| 0X73         | MOV M,E     | 1      |                 | (HL) <- E                                       |
| 0X74         | MOV M,H     | 1      |                 | (HL) <- H                                       |
| 0X75         | MOV M,L     | 1      |                 | (HL) <- L                                       |
| 0X76         | HLT         | 1      |                 | special                                         |
| 0X77         | MOV M,A     | 1      |                 | (HL) <- A                                       |
| 0X78         | MOV A,B     | 1      |                 | A <- B                                          |
| 0X79         | MOV A,C     | 1      |                 | A <- C                                          |
| 0X7A         | MOV A,D     | 1      |                 | A <- D                                          |
| 0X7B         | MOV A,E     | 1      |                 | A <- E                                          |
| 0X7C         | MOV A,H     | 1      |                 | A <- H                                          |
| 0X7D         | MOV A,L     | 1      |                 | A <- L                                          |
| 0X7E         | MOV A,M     | 1      |                 | A <- (HL)                                       |
| 0X7F         | MOV A,A     | 1      |                 | A <- A                                          |
| 0XER         | ERR         | -1     | C               | Take a break m8                                 |
| 0X80         | ADD B       | 1      | Z, S, P, CY, AC | A <- A + B                                      |
| 0X81         | ADD C       | 1      | Z, S, P, CY, AC | A <- A + C                                      |
| 0X82         | ADD D       | 1      | Z, S, P, CY, AC | A <- A + D                                      |
| 0X83         | ADD E       | 1      | Z, S, P, CY, AC | A <- A + E                                      |
| 0X84         | ADD H       | 1      | Z, S, P, CY, AC | A <- A + H                                      |
| 0X85         | ADD L       | 1      | Z, S, P, CY, AC | A <- A + L                                      |
| 0X86         | ADD M       | 1      | Z, S, P, CY, AC | A <- A + (HL)                                   |
| 0X87         | ADD A       | 1      | Z, S, P, CY, AC | A <- A + A                                      |
| 0X88         | ADC B       | 1      | Z, S, P, CY, AC | A <- A + B + CY                                 |
| 0X89         | ADC C       | 1      | Z, S, P, CY, AC | A <- A + C + CY                                 |
| 0X8A         | ADC D       | 1      | Z, S, P, CY, AC | A <- A + D + CY                                 |
| 0X8B         | ADC E       | 1      | Z, S, P, CY, AC | A <- A + E + CY                                 |
| 0X8C         | ADC H       | 1      | Z, S, P, CY, AC | A <- A + H + CY                                 |
| 0X8D         | ADC L       | 1      | Z, S, P, CY, AC | A <- A + L + CY                                 |
| 0X8E         | ADC M       | 1      | Z, S, P, CY, AC | A <- A + (HL) + CY                              |
| 0X8F         | ADC A       | 1      | Z, S, P, CY, AC | A <- A + A + CY                                 |
| 0X90         | SUB B       | 1      | Z, S, P, CY, AC | A <- A - B                                      |
| 0X91         | SUB C       | 1      | Z, S, P, CY, AC | A <- A - C                                      |
| 0X92         | SUB D       | 1      | Z, S, P, CY, AC | A <- A + D                                      |
| 0X93         | SUB E       | 1      | Z, S, P, CY, AC | A <- A - E                                      |
| 0X94         | SUB H       | 1      | Z, S, P, CY, AC | A <- A + H                                      |
| 0X95         | SUB L       | 1      | Z, S, P, CY, AC | A <- A - L                                      |
| 0X96         | SUB M       | 1      | Z, S, P, CY, AC | A <- A + (HL)                                   |
| 0X97         | SUB A       | 1      | Z, S, P, CY, AC | A <- A - A                                      |
| 0X98         | SBB B       | 1      | Z, S, P, CY, AC | A <- A - B - CY                                 |
| 0X99         | SBB C       | 1      | Z, S, P, CY, AC | A <- A - C - CY                                 |
| 0X9A         | SBB D       | 1      | Z, S, P, CY, AC | A <- A - D - CY                                 |
| 0X9B         | SBB E       | 1      | Z, S, P, CY, AC | A <- A - E - CY                                 |
| 0X9C         | SBB H       | 1      | Z, S, P, CY, AC | A <- A - H - CY                                 |
| 0X9D         | SBB L       | 1      | Z, S, P, CY, AC | A <- A - L - CY                                 |
| 0X9E         | SBB M       | 1      | Z, S, P, CY, AC | A <- A - (HL) - CY                              |
| 0X9F         | SBB A       | 1      | Z, S, P, CY, AC | A <- A - A - CY                                 |
| 0XA0         | ANA B       | 1      | Z, S, P, CY, AC | A <- A & B                                      |
| 0XA1         | ANA C       | 1      | Z, S, P, CY, AC | A <- A & C                                      |
| 0XA2         | ANA D       | 1      | Z, S, P, CY, AC | A <- A & D                                      |
| 0XA3         | ANA E       | 1      | Z, S, P, CY, AC | A <- A & E                                      |
| 0XA4         | ANA H       | 1      | Z, S, P, CY, AC | A <- A & H                                      |
| 0XA5         | ANA L       | 1      | Z, S, P, CY, AC | A <- A & L                                      |
| 0XA6         | ANA M       | 1      | Z, S, P, CY, AC | A <- A & (HL)                                   |
| 0XA7         | ANA A       | 1      | Z, S, P, CY, AC | A <- A & A                                      |
| 0XA8         | XRA B       | 1      | Z, S, P, CY, AC | A <- A ^ B                                      |
| 0XA9         | XRA C       | 1      | Z, S, P, CY, AC | A <- A ^ C                                      |
| 0XAA         | XRA D       | 1      | Z, S, P, CY, AC | A <- A ^ D                                      |
| 0XAB         | XRA E       | 1      | Z, S, P, CY, AC | A <- A ^ E                                      |
| 0XAC         | XRA H       | 1      | Z, S, P, CY, AC | A <- A ^ H                                      |
| 0XAD         | XRA L       | 1      | Z, S, P, CY, AC | A <- A ^ L                                      |
| 0XAE         | XRA M       | 1      | Z, S, P, CY, AC | A <- A ^ (HL)                                   |
| 0XAF         | XRA A       | 1      | Z, S, P, CY, AC | A <- A ^ A                                      |
| 0XB0         | ORA B       | 1      | Z, S, P, CY, AC | A <- A                                          |
| 0XB1         | ORA C       | 1      | Z, S, P, CY, AC | A <- A                                          |
| 0XB2         | ORA D       | 1      | Z, S, P, CY, AC | A <- A                                          |
| 0XB3         | ORA E       | 1      | Z, S, P, CY, AC | A <- A                                          |
| 0XB4         | ORA H       | 1      | Z, S, P, CY, AC | A <- A                                          |
| 0XB5         | ORA L       | 1      | Z, S, P, CY, AC | A <- A                                          |
| 0XB6         | ORA M       | 1      | Z, S, P, CY, AC | A <- A                                          |
| 0XB7         | ORA A       | 1      | Z, S, P, CY, AC | A <- A                                          |
| 0XB8         | CMP B       | 1      | Z, S, P, CY, AC | A - B                                           |
| 0XB9         | CMP C       | 1      | Z, S, P, CY, AC | A - C                                           |
| 0XBA         | CMP D       | 1      | Z, S, P, CY, AC | A - D                                           |
| 0XBB         | CMP E       | 1      | Z, S, P, CY, AC | A - E                                           |
| 0XBC         | CMP H       | 1      | Z, S, P, CY, AC | A - H                                           |
| 0XBD         | CMP L       | 1      | Z, S, P, CY, AC | A - L                                           |
| 0XBE         | CMP M       | 1      | Z, S, P, CY, AC | A - (HL)                                        |
| 0XBF         | CMP A       | 1      | Z, S, P, CY, AC | A - A                                           |
| 0XC0         | RNZ         | 1      |                 | if NZ, RET                                      |
| 0XC1         | POP B       | 1      |                 | C <- (sp); B <- (sp+1); sp <- sp+2              |
| 0XC2         | JNZ adr     | 3      |                 | if NZ, PC <- adr                                |
| 0XC3         | JMP adr     | 3      |                 | PC <= adr                                       |
| 0XC4         | CNZ adr     | 3      |                 | if NZ, CALL adr                                 |
| 0XC5         | PUSH B      | 1      |                 | (sp-2)<-C; (sp-1)<-B; sp <- sp - 2              |
| 0XC6         | ADI D8      | 2      | Z, S, P, CY, AC | A <- A + byte                                   |
| 0XC7         | RST 0       | 1      |                 | CALL \\$0                                       |
| 0XC8         | RZ          | 1      |                 | if Z, RET                                       |
| 0XC9         | RET         | 1      |                 | PC.lo <- (sp); PC.hi<-(sp+1); SP <- SP+2        |
| 0XCA         | JZ adr      | 3      |                 | if Z, PC <- adr                                 |
| 0XCB         | -           |        |                 |                                                 |
| 0XCC         | CZ adr      | 3      |                 | if Z, CALL adr                                  |
| 0XCD         | CALL adr    | 3      |                 | (SP-1)<-PC.hi;(SP-2)<-PC.lo;SP<-SP-2;PC=adr     |
| 0XCE         | ACI D8      | 2      | Z, S, P, CY, AC | A <- A + data + CY                              |
| 0XCF         | RST 1       | 1      |                 | CALL \$8                                        |
| 0XD0         | RNC         | 1      |                 | if NCY, RET                                     |
| 0XD1         | POP D       | 1      |                 | E <- (sp); D <- (sp+1); sp <- sp+2              |
| 0XD2         | JNC adr     | 3      |                 | if NCY, PC<-adr                                 |
| 0XD3         | OUT D8      | 2      |                 | special                                         |
| 0XD4         | CNC adr     | 3      |                 | if NCY, CALL adr                                |
| 0XD5         | PUSH D      | 1      |                 | (sp-2)<-E; (sp-1)<-D; sp <- sp - 2              |
| 0XD6         | SUI D8      | 2      | Z, S, P, CY, AC | A <- A - data                                   |
| 0XD7         | RST 2       | 1      |                 | CALL \$10                                       |
| 0XD8         | RC          | 1      |                 | if CY, RET                                      |
| 0XD9         | -           |        |                 |                                                 |
| 0XDA         | JC adr      | 3      |                 | if CY, PC<-adr                                  |
| 0XDB         | IN D8       | 2      |                 | special                                         |
| 0XDC         | CC adr      | 3      |                 | if CY, CALL adr                                 |
| 0XDD         | -           |        |                 |                                                 |
| 0XDE         | SBI D8      | 2      | Z, S, P, CY, AC | A <- A - data - CY                              |
| 0XDF         | RST 3       | 1      |                 | CALL \$18                                       |
| 0XE0         | RPO         | 1      |                 | if PO, RET                                      |
| 0XE1         | POP H       | 1      |                 | L <- (sp); H <- (sp+1); sp <- sp+2              |
| 0XE2         | JPO adr     | 3      |                 | if PO, PC <- adr                                |
| 0XE3         | XTHL        | 1      |                 | L <-> (SP); H <-> (SP+1)                        |
| 0XE4         | CPO adr     | 3      |                 | if PO, CALL adr                                 |
| 0XE5         | PUSH H      | 1      |                 | (sp-2)<-L; (sp-1)<-H; sp <- sp - 2              |
| 0XE6         | ANI D8      | 2      | Z, S, P, CY, AC | A <- A & data                                   |
| 0XE7         | RST 4       | 1      |                 | CALL \$20                                       |
| 0XE8         | RPE         | 1      |                 | if PE, RET                                      |
| 0XE9         | PCHL        | 1      |                 | PC.hi <- H; PC.lo <- L                          |
| 0XEA         | JPE adr     | 3      |                 | if PE, PC <- adr                                |
| 0XEB         | XCHG        | 1      |                 | H <-> D; L <-> E                                |
| 0XEC         | CPE adr     | 3      |                 | if PE, CALL adr                                 |
| 0XED         | -           |        |                 |                                                 |
| 0XEE         | XRI D8      | 2      | Z, S, P, CY, AC | A <- A ^ data                                   |
| 0XEF         | RST 5       | 1      |                 | CALL \$28                                       |
| 0XF0         | RP          | 1      |                 | if P, RET                                       |
| 0XF1         | POP PSW     | 1      |                 | flags <- (sp); A <- (sp+1); sp <- sp+2          |
| 0XF2         | JP adr      | 3      |                 | if P=1 PC <- adr                                |
| 0XF3         | DI          | 1      |                 | special                                         |
| 0XF4         | CP adr      | 3      |                 | if P, PC <- adr                                 |
| 0XF5         | PUSH PSW    | 1      |                 | (sp-2)<-flags; (sp-1)<-A; sp <- sp - 2          |
| 0XF6         | ORI D8      | 2      | Z, S, P, CY, AC | A <- A                                  data    |
| 0XF7         | RST 6       | 1      |                 | CALL \$30                                       |
| 0XF8         | RM          | 1      |                 | if M, RET                                       |
| 0XF9         | SPHL        | 1      |                 | SP=HL                                           |
| 0XFA         | JM adr      | 3      |                 | if M, PC <- adr                                 |
| 0XFB         | EI          | 1      |                 | special                                         |
| 0XFC         | CM adr      | 3      |                 | if M, CALL adr                                  |
| 0XFD         | -           |        |                 |                                                 |
| 0XFE         | CPI D8      | 2      | Z, S, P, CY, AC | A - data                                        |
| 0XFF         | RST 7       | 1      |                 | CALL \$38                                       |
|--------------|-------------|--------|-----------------|-------------------------------------------------|
