use crate::i8080::registers::*;

#[derive(Clone)]
pub enum AddressingMode {
    Direct,
    Pair,
    StackPointer,
    ImmediateOneByte,
    ImmediateTwoBytes,
    Unknown,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum InstructionTypes {
    MOV,
    // Arithemtic
    ADD,
    ADC,
    SUB,
    SBB,
    ANA,
    XRA,
    ORA,
    CMP,
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
    Unknown,
}

#[derive(Clone)]
pub struct Instruction {
    pub byte_val: u8,
    name: String,
    cycles: u8,
    adress_mode: AddressingMode,
    pub inst_type: InstructionTypes,
    pub byte1: Option<u8>,
    pub byte2: Option<u8>,
    //argument: u8,
    //result_locaton: u8
}

impl Instruction{

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
    pub fn byte_to_op(&mut self, b: u8) {
        self.byte_val = b;
        *self = Instruction::new();
        self.byte_val = b;

        match b & 0b11000000{
            // Move Instructions
            // byte1 should hold destination register
            // byte2 should hould source register
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
            // byte1 should hold source register
            // byte2 is unused
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
                        self.inst_type = InstructionTypes::ANA;
                        let temp = format!("ANA {}", Registers::translate_to_reg(self.byte1.unwrap()));
                        self.name = temp;
                    }
                    0b10101000 =>{
                        self.inst_type = InstructionTypes::XRA;
                        let temp = format!("XRA {}", Registers::translate_to_reg(self.byte1.unwrap()));
                        self.name = temp;
                    }
                    0b10110000 =>{
                        self.inst_type = InstructionTypes::ORA;
                        let temp = format!("ORA {}", Registers::translate_to_reg(self.byte1.unwrap()));
                        self.name = temp;
                    }
                    0b10111000 =>{
                        self.inst_type = InstructionTypes::CMP;
                        let temp = format!("CMP {}", Registers::translate_to_reg(self.byte1.unwrap()));
                        self.name = temp;
                    }
                    _ => panic!("Arithemtic does not exist!: {:X}", self.byte_val),
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

    fn immediate_op_helper(&mut self, name1: String, op1: InstructionTypes, name2: String, op2: InstructionTypes){
        if self.byte_val & 0x0F == 0x06{
            self.inst_type = op1;
            self.name = name1;
        }else if self.byte_val & 0x0F == 0x0E{
            self.inst_type = op2;
            self.name = name2;
        }else{
            panic!("Error, should either be {} or {}", name1, name2);
        }

    }

    fn byte_to_immediate_op(&mut self){
        self.adress_mode = AddressingMode::Direct;

        match self.byte_val & 0xF0{
            0x00 | 0x10 | 0x20 | 0x30 =>{
                self.inst_type = InstructionTypes::MVI;
                // "Convert" register byte to format that set_reg(reg) uses.
                if self.byte_val & 0x0F == 0x06{
                    self.byte1 = Some((self.byte_val & 0x30) >> 3);
                }else{
                    self.byte1 = Some(((self.byte_val & 0x30) >> 3) + 0x01);
                }

                let temp = format!("MVI {},d8 ", Registers::translate_to_reg(self.byte1.unwrap()));
                self.name = temp;
                (); // HLT instruction
                return
            },
            // ADI, ACI
            0xC0 => {
                self.immediate_op_helper("ADI".to_string(), InstructionTypes::ADI, "ACI".to_string(), InstructionTypes::ACI);
            },
            // SUI, SBI
            0xD0 => {
                self.immediate_op_helper("SUI".to_string(), InstructionTypes::SUI, "SBI".to_string(), InstructionTypes::SBI);
            },
            // ANI, XRI
            0xE0 => {
                self.immediate_op_helper("ANI".to_string(), InstructionTypes::ANI, "XRI".to_string(), InstructionTypes::XRI);
            },
            // ORI, CPI
            0xF0 =>{
                self.immediate_op_helper("ORI".to_string(), InstructionTypes::ORI, "CPI".to_string(), InstructionTypes::CPI);
            },
            _ => (),
        }

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
