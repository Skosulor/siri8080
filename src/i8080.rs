mod instructions;
pub mod registers;
pub mod flags;

use crate::disassembler;
use std::fs::File;
use std::io::Read;
use instructions::*;
use registers::*;
use flags::*;

const MEMORY_SIZE: usize = 0xFFFFF;

pub struct Processor {
    clock_freq: f32,
    stack_pointer: u16,
    program_counter: u16,
    memory: [u8; MEMORY_SIZE],
    registers: Registers,
    flags: StatusFlags,
    current_op: Instruction,
}

impl Processor {
    pub fn from(p: String) -> Processor {
        let mut proc = Processor {
            clock_freq: 0.0,
            stack_pointer: 0x20,
            program_counter: 0,
            memory: [0; MEMORY_SIZE],
            flags: StatusFlags::new(),
            current_op: Instruction::new(),
            registers: Registers::new(),
        };

        let mut file = File::open(p).expect("No such file");
        file.read(&mut proc.memory).expect("opsie");
        proc
    }

    pub fn clock(&mut self, debug: bool) {
        self.next_instruction();
        self.execute_instruction();
        self.update_program_counter();
        // if debug {
        //     self.update_disassembler();
        // }
     }

    pub fn reset_pc(&mut self){
        self.program_counter = 0;
    }

    fn next_instruction(&mut self)  {
        self.current_op.byte_to_op(self.memory[self.program_counter as usize]);
            // Update program counter here?
    }

    fn execute_instruction(&mut self){
        match self.current_op.inst_type{
            InstructionTypes::MOV => self.mov_op(),
            InstructionTypes::ADD => self.add_op(false),
            InstructionTypes::ADC => self.add_op(true),
            InstructionTypes::SUB => self.sub_op(false),
            InstructionTypes::SBB => self.sub_op(true),
            InstructionTypes::MVI => self.mvi_op(),
            InstructionTypes::ANA => self.ana_op(),
            InstructionTypes::ORA => self.ora_op(),
            InstructionTypes::XRA => self.xra_op(),
            InstructionTypes::CMP => self.cmp_op(),
            InstructionTypes::ADI => self.add_op(false),
            InstructionTypes::ACI => self.add_op(true),
            InstructionTypes::SUI => self.sub_op(false),
            InstructionTypes::SBI => self.sub_op(true),
            InstructionTypes::ANI => self.ana_op(),
            InstructionTypes::XRI => (), // TODO
            InstructionTypes::ORI => (), // TODO
            InstructionTypes::CPI => (), // TODO
            _ => (),
        }
    }

    fn update_program_counter(&mut self){
        self.program_counter += 1;
    }

    pub fn get_pc(&self) -> usize {
        return self.program_counter as usize;
    }

    pub fn update_disassembler(&mut self){
        let mut test: Vec<String> = Vec::new();//= vec!["".to_string(), "0xf3 : MOV B,D".to_string(),"0xf3 : MOV B,D".to_string() ];
        test.push("".to_string());
        for x in 1..48{
            //test.push("");
            let instruction = Instruction::from_byte(self.memory[self.program_counter as usize + x]);
            let (bin, stri) = instruction.get_name_byte();
            test.push(String::from(format!("{a:>6}:     0x{b:02X} {c:}", a=(self.program_counter as usize + x), b=bin, c=stri)));
        }
        let mut term = disassembler::Term::default();
        term.set_flags(&self.flags);
        term.set_regs(&self.registers);
        term.update_instructions(test);
        term.test_tui()
    }

    // Set register to value
    fn set_reg(&mut self, reg:u8, val: u8){
        match reg & 0b111{

            B_REG   => self.registers.b = val,
            C_REG   => self.registers.c = val,
            D_REG   => self.registers.d = val,
            E_REG   => self.registers.e = val,
            H_REG   => self.registers.h = val,
            L_REG   => self.registers.l = val,
            MEM_REF => self.memory[self.stack_pointer as usize] = val,
            A_REG   => self.registers.accumulator = val,
            _ => panic!("No register {}", reg)
        }
    }
    // Get current value from register
    fn get_reg(&self, reg: u8) -> u8{
        match reg & 0b111{
            B_REG   => self.registers.b,
            C_REG   => self.registers.c,
            D_REG   => self.registers.d,
            E_REG   => self.registers.e,
            H_REG   => self.registers.h,
            L_REG   => self.registers.l,
            MEM_REF => self.memory[self.stack_pointer as usize] ,
            A_REG   => self.registers.accumulator,
            _ => panic!("No register {}", reg)

        }
    }

    fn mov_op(&mut self){

        let to = (self.current_op.byte_val & 0b00111000) >> MOVE_TO;
        let from = (self.current_op.byte_val & 0b00000111) >> MOVE_FROM;
        let val = self.get_reg(from);
        self.set_reg(to, val);
    }

    fn add_op(&mut self, with_carry: bool){
        let operand1 = self.get_reg(A_REG);

        // Fetch operand from register/memory or immediate
        let operand2 = match self.current_op.inst_type {
            InstructionTypes::ADD | InstructionTypes::ADC => {
                self.get_reg(self.current_op.byte1.unwrap())
            }
            InstructionTypes::ADI | InstructionTypes::ACI => {
                self.program_counter += 1;
                self.memory[self.program_counter as usize]
            },
            _ => {panic!("Add type is wrong, this panic should be impossible");}
        };


        // Either add with or wihtout the carry bit
        let (res, carry ) = if with_carry {
            let c = if self.flags.carry_flag {1} else {0};
            operand2.overflowing_add(operand1 + c)
        }else{
            operand2.overflowing_add(operand1)
        };

        self.flags.carry_flag = carry;
        self.flags.parity_flag = parity(res);
        self.flags.auxiliary_flag = auxiliary();
        self.flags.sign_flag = sign(res);
        self.flags.zero_flag = zero(res);

        self.set_reg(A_REG, res);
    }

    fn sub_op(&mut self, with_carry: bool){
        let operand1 = self.get_reg(A_REG);

        // Fetch operand from register/memory or immediate
        let operand2 = match self.current_op.inst_type {
            InstructionTypes::SUB | InstructionTypes::SBB => {
                self.get_reg(self.current_op.byte1.unwrap())
            }
            InstructionTypes::SUI | InstructionTypes::SBI => {
                self.program_counter += 1;
                self.memory[self.program_counter as usize]
            },
            _ => {panic!("Add type is wrong, this panic should be impossible");}
        };


        let (res, carry ) = if with_carry {
            let c = if self.flags.carry_flag {1} else {0};
            operand2.overflowing_add(operand1 + c)
        }else{
            operand2.overflowing_sub(operand1)
        };

        self.flags.carry_flag = carry;
        self.flags.parity_flag = parity(res);
        self.flags.auxiliary_flag = auxiliary();
        self.flags.sign_flag = sign(res);
        self.flags.zero_flag = zero(res);

        self.set_reg(A_REG, res);
    }

    // Logical AND
    fn ana_op(&mut self){
        let operand1 = self.get_reg(A_REG);
        let operand2 = match self.current_op.inst_type {
            InstructionTypes::ANA => {
                self.get_reg(self.current_op.byte1.unwrap())
            }
            InstructionTypes::ANI => {
                self.program_counter += 1;
                self.memory[self.program_counter as usize]
            }
            _ => panic!("should impossible case"),
        };
        let res = operand1 & operand2;
        self.set_flags_CZSP(false, res);
        self.set_reg(A_REG, res);
    }

    // Logaical OR
    fn ora_op(&mut self){
        let operand1 = self.get_reg(A_REG);
        // let operand2 = self.get_reg(self.current_op.byte_val);
        let operand2 = self.get_reg(self.current_op.byte1.unwrap());
        let res = operand1 | operand2;
        self.set_flags_CZSP(false, res);
        self.set_reg(A_REG, res);
    }

    // Logixal exclusive-OR
    fn xra_op(&mut self){
        let operand1 = self.get_reg(A_REG);
        //let operand2 = self.get_reg(self.current_op.byte_val);
        let operand2 = self.get_reg(self.current_op.byte1.unwrap());
        let res = operand1 ^ operand2;
        // REVIEW aux flag should probably alwas be set to false
        self.flags.auxiliary_flag = false;
        self.set_flags_CZSP(false, res);
        self.set_reg(A_REG, res);
    }

    // Compare accumelator with reg or memory
    fn cmp_op(&mut self){
        let operand1 = self.get_reg(A_REG);
        //let operand2 = self.get_reg(self.current_op.byte_val);
        let operand2 = self.get_reg(self.current_op.byte1.unwrap());
        let (res, carry) = operand2.overflowing_sub(operand1);
        // REVIEW aux flag should probably be set to false
        self.flags.auxiliary_flag = false;
        self.set_flags_CZSP(carry, res);
    }

    // Immediate move
    fn mvi_op(&mut self){
        self.program_counter += 1;
        let result = self.memory[self.program_counter as usize];
        self.set_reg(self.current_op.byte1.unwrap(), result);
    }

    // Set flags depending on result.
    pub fn set_flags_CZSP(&mut self, carry: bool, res: u8){
        self.flags.carry_flag  = carry;
        self.flags.parity_flag = parity(res);
        self.flags.sign_flag   = sign(res);
        self.flags.zero_flag   = zero(res);
    }
}
