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
            InstructionTypes::MOV => self.move_op(),
            InstructionTypes::ADD => self.add_op(false),
            InstructionTypes::ADC => self.add_op(true),
            InstructionTypes::SUB => self.sub_op(false),
            InstructionTypes::SBB => self.sub_op(true),
            InstructionTypes::MVI => self.mvi_op(),
            InstructionTypes::ORA => self.ana_op(),
            InstructionTypes::ANA => (),
            InstructionTypes::XRA => (),
            InstructionTypes::CMP => (),

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

    fn move_op(&mut self){

        let to = (self.current_op.byte_val & 0b00111000) >> MOVE_TO;
        let from = (self.current_op.byte_val & 0b00000111) >> MOVE_FROM;
        let val = self.get_reg(from);
        self.set_reg(to, val);
    }

    fn add_op(&mut self, with_carry: bool){
        let prev_acc_value = self.get_reg(A_REG);
        // Todo this should be byte one?
        let add_val = self.get_reg((self.current_op.byte_val >> ARITHMETIC_WITH) &0b111);


        let (res, carry ) = if with_carry {
            let c = if self.flags.carry_flag {1} else {0};
            add_val.overflowing_add(prev_acc_value + c)
        }else{
            add_val.overflowing_add(prev_acc_value)
        };

        self.flags.carry_flag = carry;
        self.flags.parity_flag = parity(res);
        self.flags.auxiliary_flag = auxiliary();
        self.flags.sign_flag = sign(res);
        self.flags.zero_flag = zero(res);

        self.set_reg(A_REG, res);
    }

    fn sub_op(&mut self, with_carry: bool){
        let prev_acc_value = self.get_reg(A_REG);
        // TODO this should be byte one instead of translating byte_val again?
        let sub_val = self.get_reg((self.current_op.byte_val >> ARITHMETIC_WITH) &0b111);

        let (res, carry ) = if with_carry {
            let c = if self.flags.carry_flag {1} else {0};
            sub_val.overflowing_add(prev_acc_value + c)
        }else{
            sub_val.overflowing_sub(prev_acc_value)
        };

        self.flags.carry_flag = carry;
        self.flags.parity_flag = parity(res);
        self.flags.auxiliary_flag = auxiliary();
        self.flags.sign_flag = sign(res);
        self.flags.zero_flag = zero(res);

        self.set_reg(A_REG, res);
    }

    // Logical AND
    // Bits affected: C, Z, S, P
    fn ana_op(&mut self){
        let operand1 = self.get_reg(A_REG);
        let operand2 = self.get_reg(self.current_op.byte_val);
        let res = operand1 & operand2;
        self.set_flags_CZSP(false, res);
        self.set_reg(A_REG, res);
    }

    fn mvi_op(&mut self){
        self.program_counter += 1;
        let result = self.memory[self.program_counter as usize];
        self.set_reg(self.current_op.byte1.unwrap(), result);
    }


    pub fn set_flags_CZSP(&mut self, carry: bool, res: u8){
        self.flags.carry_flag = carry;
        self.flags.parity_flag = parity(res);
        self.flags.sign_flag = sign(res);
        self.flags.zero_flag = zero(res);
    }
}
