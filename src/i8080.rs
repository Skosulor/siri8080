use std::fs::File;
use std::io::Read;
use crate::disassembler;

const MEMORY_SIZE: usize = 0xFFFFF;
const MOVE_TO: u8 = 3;
const MOVE_FROM: u8 = 0;
const ARITHMETIC_WITH: u8 = 0;
const MOVE_I_TO: u8 = 4;

const B_REG: u8 = 0b000;
const C_REG: u8 = 0b001;
const D_REG: u8 = 0b010;
const E_REG: u8 = 0b011;
const H_REG: u8 = 0b100;
const L_REG: u8 = 0b101;
const A_REG: u8 = 0b111;
const MEM_REF: u8 = 0b110;

pub struct Processor {
    clock_freq: f32,
    stack_pointer: u16,
    program_counter: u16,
    memory: [u8; MEMORY_SIZE],
    registers: Registers,
    flags: StatusFlags,
    current_op: Instruction,
}
pub struct StatusFlags {
    pub carry_flag: bool,
    pub auxiliary_flag: bool,
    pub sign_flag: bool,
    pub zero_flag: bool,
    pub parity_flag: bool,
}

#[derive(Clone)]
struct Instruction {
    byte_val: u8,
    name: String,
    cycles: u8,
    adress_mode: AddressingMode,
    inst_type: InstructionTypes,
    byte1: Option<u8>,
    byte2: Option<u8>,
    //argument: u8,
    //result_locaton: u8
}


pub struct Registers {
  pub accumulator: u8,
  pub b: u8,
  pub c: u8,
  pub d: u8,
  pub e: u8,
  pub h: u8,
  pub l: u8,
}

#[derive(Clone)]
enum AddressingMode {
    Direct,
    Pair,
    StackPointer,
    ImmediateOneByte,
    ImmediateTwoBytes,
    Unknown,
}

#[derive(Clone)]
enum InstructionTypes {
    MOV,
    ADD,
    ADC,
    SUB,
    SBB,
    // Immediate
    MVI,
    ADI,
    ACI,
    SUI,
    SBI,
    ANI,
    ORI,
    XRI,
    CPI,
    ANA,
    XRA,
    ORA,
    CMP,
    Unknown,
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
            InstructionTypes::ORA => (),
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
        // Todo this should be byte one?
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

    fn mvi_op(&mut self){
        self.program_counter += 1;
        let result = self.memory[self.program_counter as usize];
        self.set_reg(self.current_op.byte1.unwrap(), result);
    }
}

pub fn auxiliary() -> bool {
    // TODO
    false
}
pub fn sign(b: u8) -> bool {
    if b & 0b10000000 == 0b10000000 {
        true
    } else {
        false
    }
}

pub fn zero(b:u8) -> bool{
    if b == 0 {
        true
    } else {
        false
    }
}

pub fn parity(b: u8) -> bool{
    if b.count_ones() % 2 == 0{
        return true
    }else{
        return false
    }
}

impl Instruction {
    pub fn new() -> Instruction{
        let mut ins = Instruction {
            byte_val: 0,
            name: "_".to_string(),
            cycles: 1,
            adress_mode: AddressingMode::Unknown,
            inst_type: InstructionTypes::Unknown,
            byte1: None,
            byte2: None,
        };
        ins
    }
    pub fn from_byte(b: u8) -> Instruction {
        let mut ins = Instruction::new();
        ins.byte_to_op(b);
        ins
    }
    pub fn get_name_byte(self) -> (u8, String) {
        (self.byte_val, self.name.clone())
    }
    fn byte_to_op(&mut self, b: u8) {
        self.byte_val = b;
        *self = Instruction::new();
        self.byte_val = b;

        match b & 0b11000000{
            // Move Instructions
            0b01000000 => {
                self.adress_mode = AddressingMode::Direct;
                self.inst_type = InstructionTypes::MOV;
                self.byte1 = Some((b >> MOVE_TO) & 0b111);
                self.byte2 = Some((b >> MOVE_FROM) & 0b111);
                let temp = format!("MOV {},{} ", Registers::translate_to_reg(self.byte1.unwrap()), Registers::translate_to_reg(self.byte2.unwrap()));
                self.name = temp;
                (); // HLT instruction
                return
            },
            // Arithmetic Instruction
            0b10000000 =>{
                self.byte1 = Some((b >> ARITHMETIC_WITH) & 0b111);
                self.adress_mode = AddressingMode::Direct;
                match b & 0b10111000{
                    // ADD
                    0b10000000 =>{
                        self.inst_type = InstructionTypes::ADD;
                        let temp = format!("ADD {}", Registers::translate_to_reg(self.byte1.unwrap()));
                        self.name = temp;
                    },
                    // ADC
                    0b10001000 =>{
                        self.inst_type = InstructionTypes::ADC;
                        let temp = format!("ADC {}", Registers::translate_to_reg(self.byte1.unwrap()));
                        self.name = temp;
                    },
                    //SUB
                    0b10010000 =>{
                        self.inst_type = InstructionTypes::SUB;
                        let temp = format!("SUB {}", Registers::translate_to_reg(self.byte1.unwrap()));
                        self.name = temp;
                    },
                    //SBB
                    0b10011000 =>{
                        self.inst_type = InstructionTypes::SBB;
                        let temp = format!("SBB {}", Registers::translate_to_reg(self.byte1.unwrap()));
                        self.name = temp;
                    },
                    0b10100000 =>{
                        // TODO
                        self.inst_type = InstructionTypes::ANA;
                        let temp = format!("ANA {}", Registers::translate_to_reg(self.byte1.unwrap()));
                        self.name = temp;
                    }
                    0b10101000 =>{
                        // TODO
                        self.inst_type = InstructionTypes::XRA;
                        let temp = format!("XRA {}", Registers::translate_to_reg(self.byte1.unwrap()));
                        self.name = temp;
                    }
                    0b10110000 =>{
                        // TODO
                        self.inst_type = InstructionTypes::ORA;
                        let temp = format!("ORA {}", Registers::translate_to_reg(self.byte1.unwrap()));
                        self.name = temp;
                    }
                    0b10111000 =>{
                        // TODO
                        self.inst_type = InstructionTypes::CMP;
                        let temp = format!("CMP {}", Registers::translate_to_reg(self.byte1.unwrap()));
                        self.name = temp;
                    }
                    _ => panic!("ARithemtic does not exist!: {:X}", self.byte_val),
                }
            },
            // Misc instructions
            0b00000000 | 0b11000000 => {
                match b & 0b00001111 {
                    0b0000 => self.name = "__".to_string(),
                    0b0001 => self.name = "__".to_string(),
                    0b0010 => self.name = "__".to_string(),
                    0b0011 => self.name = "__".to_string(),
                    0b0100 => self.name = "__".to_string(),
                    0b0101 => self.name = "__".to_string(),
                    // 6
                    0b0110 | 0b1110 =>{
                        self.name = "immediate".to_string();
                        self.byte_to_immediate_op();
                    },
                    0b0111 => self.name = "__".to_string(),
                    0b1000 => self.name = "__".to_string(),
                    0b1001 => self.name = "__".to_string(),
                    0b1010 => self.name = "__".to_string(),
                    0b1011 => self.name = "__".to_string(),
                    0b1100 => self.name = "__".to_string(),
                    0b1101 => self.name = "__".to_string(),
                    //
                    0b1111 => self.name = "__".to_string(),
                    _ => panic!("Misc should not exist!"),
                }
            },
            _ => panic!("This match should never happen")

        }

    }

    fn byte_to_immediate_op(&mut self){
        match self.byte_val & 0xF0{
            0x00 | 0x10 | 0x20 | 0x30 =>{
                self.adress_mode = AddressingMode::Direct;
                self.inst_type = InstructionTypes::MVI;


                if self.byte_val & 0x0F == 0x06{
                    self.byte1 = Some((self.byte_val & 0x30) >> 3);
                }else{
                    self.byte1 = Some(((self.byte_val & 0x30) >> 3) + 0x01);
                }

                let temp = format!("MVI {},d8 ", Registers::translate_to_reg(self.byte1.unwrap()));
                self.name = temp;
                (); // HLT instruction
                return

            }
            _ => (),
        }

    }

}


impl Registers{

    pub fn new() -> Registers{
        let reg = Registers{
            accumulator: 0,
            e: 0,
            b: 0,
            c: 0,
            d: 0,
            h: 0,
            l: 0,
        };
        reg
    }
    fn translate_to_reg(reg: u8) -> String{
        match reg & 0b111{
            B_REG   => String::from("B"),
            C_REG   => String::from("C"),
            D_REG   => String::from("D"),
            E_REG   => String::from("E"),
            H_REG   => String::from("H"),
            L_REG   => String::from("L"),
            MEM_REF => String::from("MEM"),
            A_REG   => String::from("A"),
            _ => panic!("No register {}", reg)

        }
    }
}


impl StatusFlags {
    fn new() -> StatusFlags {
        let f = StatusFlags {
            carry_flag: false,
            auxiliary_flag: false,
            sign_flag: false,
            zero_flag: false,
            parity_flag: false,
        };
        f
    }
}

















 // ADD,
 // SUB,
 // POP,
 // PUSH,
 // LXI,
 // STAX,
 // SHLD,
 // STA,
 // INX,
 // INR,
 // DCR,
 // MVI,
 // RLC,
 // RAL,
 // DAA,
 // STC,
 // DAD,
 // LDAX,
 // LHLD,
 // LDA,
 // DCX,
 // PRC,
 // RAR,
 // CMA,
 // CMC,
 // ANA,
 // XRA,
 // ORA,
 // CMP,
 // RNZ,
 // RNC,
 // RPO,
 // RP,
 // JNZ,
 // JNC,
 // JPO,
 // JP,
 // JMP,
 // OUT,
 // XTHL,
 // DI,
 // CNZ,
 // CNC,
 // CPO,
 // CP,
 // PUSH_PSW,
 // SUI,
 // ADI,
 // ANI,
 // ORI,
 // RST,
 // RZ,
 // RC,
 // RPE,
 // RM,
 // RET,
 // PCHL,
 // SPHL,
 // JZ,
 // JC,
 // JPE,
 // JM,
 // IN,
 // XCHG,
 // EI,
 // CZ,
 // CC,
 // CPE,
 // CM,
 // CALL,
 // ACI,
 // SBI,
 // XRI,
 // CPI,
 // Nop,
 // Unknown,
 // Data,

