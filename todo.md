#+TITLE: Todo


* Guide
** How to implement new instruction:
1. Add enum for instruction type in _instructions.rs_
2. Match binary value of op code in function _byte_to_op_
3. set the following:
   + self.adress_mode
   + self.inst_type
   + self.byte1 (optionally)
   + self.byte2 (optionally)
   + self.name
4. create function _[name]_op_ in _i8080.rs_ which sets flags and executes OP function
5. In file _i8080.rs_ match the instruction type in function
   _execute_instruction_ and execute the newly created function for the OP
6. Remember to increment the PC correctly


## Dissasembler bindings

| Binding | name       | function                                   |
|---------|------------|--------------------------------------------|
| c       | continue   | run forever                                |
| s       | step       | stops the loaded program and take one step |
| b+      | breakpoint | Set a breakpoint and run                   |
| q       | quit       | Exit emulation                             |

+Due to a unfixed issue, type a space before typing the line number.

# General
- [ ] Refactor: Break out operations from proc
    - [X] Brake out debug commands from main
    - [X] Give dissasmbler sane variable & function names
- [ ] Make rename byte1 & byte2 in instructions to Registers & give them an enum type?
- [ ] Add tests
- [ ] Update instruction counter based on type of instruction
- [ ] Acutally set aux flag
- [X] Add Program counter to debugger
- [ ] Add cycle count to debugger

# Implement Op Codes
TODO Op codes needed for space invaders
- [X] Arithmetic
    - [X] ADD
    - [X] SUB
    - [X] SBB
    - [X] ADC
    - [X] ANA
    - [X] ORA
    - [X] XRA
    - [X] CMP
    - [X] MOV
- [X] Immediate
    - [X] ADI
    - [X] ACI
    - [X] SBI
    - [X] SUI
    - [X] ANI
    - [X] ORI
    - [X] XRI
    - [X] CPI
    - [X] MVI
- [ ] Misc
    - [X] LXI
    - [X] DCR
    - [X] DAD
    - [X] RRC
    - [ ] INX
    - [ ] LXI
    - [ ] STA
    - [ ] LDA
    - [X] JNZ
    - [X] JMP
    - [ ] PUSH
    - [ ] RET
    - [ ] CALL
    - [ ] POP
    - [ ] OUT
    - [ ] PUSH
    - [ ] XCHG
    - [ ] EI


