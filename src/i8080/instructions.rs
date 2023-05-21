use crate::i8080::registers::*;

#[derive(Clone)]
pub enum AddressingMode 
{
    Direct,
    // Pair,
    // StackPointer,
    // ImmediateOneByte,
    // ImmediateTwoBytes,
    Unknown,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum InstructionTypes 
{
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
    MVI,
    ADI,
    ACI,
    SUI,
    SBI,
    ANI,
    ORI,
    XRI,
    CPI,
    JMP,
    JNZ,
    JZ,
    JNC,
    JC,
    JPO,
    JPE,
    JP,
    JM,
    LXI,
    DCR,
    DAD,
    RRC,
    RLC,
    RAL,
    RAR,
    INX,
    LDA,
    LDAX,
    STA,
    PUSH,
    POP,
    CALL,
    NOP,
    Unknown,
}

#[derive(Clone)]
pub struct Instruction 
{
    pub byte_val: u8,
    name: String,
    // cycles: u8,
    adress_mode: AddressingMode,
    pub inst_type: InstructionTypes,
    pub byte1: Option<u8>,
    pub byte2: Option<u8>,
}

impl Instruction
{

    pub fn new() -> Instruction
    {
        let ins = Instruction 
        {
            byte_val: 0,
            name: "_".to_string(),
            // cycles: 1,
            adress_mode: AddressingMode::Unknown,
            inst_type: InstructionTypes::Unknown,
            byte1: None,
            byte2: None,
        };
        ins
    }
    pub fn from_byte(b: u8) -> Instruction 
    {
        let mut ins = Instruction::new();
        ins.byte_to_op(b);
        ins
    }
    pub fn get_name_byte(self) -> (u8, String) 
    {
        (self.byte_val, self.name.clone())
    }
    pub fn byte_to_op(&mut self, b: u8) 
    {
        self.byte_val = b;
        *self = Instruction::new();
        self.byte_val = b;

        match b & 0b11000000
        {
            // Move Instructions
            // byte1 should hold destination register
            // byte2 should hould source register
            0b01000000 => 
            {
                self.adress_mode = AddressingMode::Direct;
                self.inst_type = InstructionTypes::MOV;
                self.byte1 = Some((b >> MOVE_TO) & 0b111);
                self.byte2 = Some((b >> MOVE_FROM) & 0b111);
                let name = format!("MOV {},{} ", Registers::translate_to_reg(self.byte1.unwrap()), Registers::translate_to_reg(self.byte2.unwrap()));
                self.name = name;
                return
            },
            // Arithmetic Instruction
            // byte1 should hold source register
            // byte2 is unused
            0b10000000 =>
            {
                self.byte1 = Some((b >> ARITHMETIC_WITH) & 0b111);
                self.adress_mode = AddressingMode::Direct;
                match b & 0b10111000
                {
                    // ADD
                    0b10000000 =>
                    {
                        self.inst_type = InstructionTypes::ADD;
                        let name = format!("ADD {}", Registers::translate_to_reg(self.byte1.unwrap()));
                        self.name = name;
                    },
                    // ADC
                    0b10001000 =>
                    {
                        self.inst_type = InstructionTypes::ADC;
                        let name = format!("ADC {}", Registers::translate_to_reg(self.byte1.unwrap()));
                        self.name = name;
                    },
                    //SUB
                    0b10010000 =>
                    {
                        self.inst_type = InstructionTypes::SUB;
                        let name = format!("SUB {}", Registers::translate_to_reg(self.byte1.unwrap()));
                        self.name = name;
                    },
                    //SBB
                    0b10011000 =>
                    {
                        self.inst_type = InstructionTypes::SBB;
                        let name = format!("SBB {}", Registers::translate_to_reg(self.byte1.unwrap()));
                        self.name = name;
                    },
                    0b10100000 =>
                    {
                        self.inst_type = InstructionTypes::ANA;
                        let name = format!("ANA {}", Registers::translate_to_reg(self.byte1.unwrap()));
                        self.name = name;
                    }
                    0b10101000 =>
                    {
                        self.inst_type = InstructionTypes::XRA;
                        let name = format!("XRA {}", Registers::translate_to_reg(self.byte1.unwrap()));
                        self.name = name;
                    }
                    0b10110000 =>
                    {
                        self.inst_type = InstructionTypes::ORA;
                        let name = format!("ORA {}", Registers::translate_to_reg(self.byte1.unwrap()));
                        self.name = name;
                    }
                    0b10111000 =>
                    {
                        self.inst_type = InstructionTypes::CMP;
                        let name = format!("CMP {}", Registers::translate_to_reg(self.byte1.unwrap()));
                        self.name = name;
                    }
                    _ => panic!("Arithemtic does not exist!: {:X}", self.byte_val),
                }
            },
            // Misc instructions
            0b11000000 => 
            {
                match b 
                {
                    0xC0 => self.name = "RNZ".to_string(),
                    0xC1 | 0xD1 | 0xE1 | 0xF1 => self.decode_pop(),
                    0xC2 => self.set_instruction(InstructionTypes::JNZ),
                    0xC3 => self.set_instruction(InstructionTypes::JMP),
                    0xC4 => self.name = "CNZ".to_string(),
                    0xC5 | 0xD5 | 0xE5 | 0xF5 => self.decode_push(),
                    0xC6 => self.byte_to_immediate_op(),
                    0xC7 => self.name = "RST 0".to_string(),
                    0xC8 => self.name = "RZ".to_string(),
                    0xC9 => self.name = "RET".to_string(),
                    0xCA => self.set_instruction(InstructionTypes::JZ),
                    0xCB => self.name = "??".to_string(),
                    0xCC => self.name = "CZ".to_string(),
                    0xCD => self.set_instruction(InstructionTypes::CALL),
                    0xCE => self.byte_to_immediate_op(),
                    0xCF => self.name = "RST".to_string(),
                    0xD0 => self.name = "RNC".to_string(),
                    0xD2 => self.set_instruction(InstructionTypes::JNC),
                    0xD3 => self.name = "OUT".to_string(),
                    0xD4 => self.name = "CNC".to_string(),
                    0xD6 => self.byte_to_immediate_op(),
                    0xD7 => self.name = "RST".to_string(),
                    0xD8 => self.name = "RC".to_string(),
                    0xD9 => self.name = "??".to_string(),
                    0xDA => self.set_instruction(InstructionTypes::JC),
                    0xDB => self.name = "IN".to_string(),
                    0xDC => self.name = "CC".to_string(),
                    0xDD => self.name = "??".to_string(),
                    0xDE => self.byte_to_immediate_op(),
                    0xDF => self.name = "RST".to_string(),
                    0xE0 => self.name = "RPO".to_string(),
                    0xE2 => self.set_instruction(InstructionTypes::JPO),
                    0xE3 => self.name = "XTHL".to_string(),
                    0xE4 => self.name = "CPO".to_string(),
                    0xE6 => self.byte_to_immediate_op(),
                    0xE7 => self.name = "RST".to_string(),
                    0xE8 => self.name = "RPE".to_string(),
                    0xE9 => self.name = "PCHL".to_string(),
                    0xEA => self.set_instruction(InstructionTypes::JPE),
                    0xEB => self.name = "XCHG".to_string(),
                    0xEC => self.name = "CPE".to_string(),
                    0xED => self.name = "??".to_string(),
                    0xEE => self.byte_to_immediate_op(),
                    0xEF => self.name = "RST".to_string(),
                    0xF0 => self.name = "RP".to_string(),
                    0xF2 => self.set_instruction(InstructionTypes::JP),
                    0xF3 => self.name = "DI".to_string(),
                    0xF4 => self.name = "CP".to_string(),
                    0xF6 => self.byte_to_immediate_op(),
                    0xF7 => self.name = "RST".to_string(),
                    0xF8 => self.name = "RM".to_string(),
                    0xF9 => self.name = "SPHL".to_string(),
                    0xFA => self.set_instruction(InstructionTypes::JM),
                    0xFB => self.name = "EI".to_string(),
                    0xFC => self.name = "CM".to_string(),
                    0xFD => self.name = "??".to_string(),
                    0xFE => self.byte_to_immediate_op(),
                    0xFF => self.name = "RST".to_string(),
                    _ => 
                    {
                        println!("Byte: {:02X}", b);
                        panic!("Should not exist!");
                    }
                }
            },

            0b00000000 => 
            {
                match b & 0b00111111 
                {
                    0x00                      => self.set_instruction(InstructionTypes::NOP),
                    0x01 | 0x11 | 0x21 | 0x31 => self.decode_lxi(),
                    0x02 | 0x12               => self.name = "STAX".to_string(),
                    0x03 | 0x13 | 0x23 | 0x33 => self.decode_inx(),
                    0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C        => self.name = "INR".to_string(),
                    0x05 | 0x0D |  0x15 | 0x1D | 0x25 | 0x2D | 0x35 | 0x3D => self.decode_dcr(),
                    0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E        => self.byte_to_immediate_op(),
                    0x07 => self.set_instruction(InstructionTypes::RLC),
                    0x08 | 0x10 | 0x18 | 0x20 | 0x28 | 0x30 | 0x38        => self.name = "__".to_string(),
                    0x09 | 0x19 | 0x29 | 0x39 => self.decode_dad(),
                    0x0B | 0x1B | 0x2B | 0x3B => self.name = "DCX".to_string(),
                    0x0F => self.set_instruction(InstructionTypes::RRC),
                    0x17 => self.set_instruction(InstructionTypes::RAL),
                    0x1A | 0x0A => self.set_instruction(InstructionTypes::LDAX),
                    0x1F => self.set_instruction(InstructionTypes::RAR),
                    0x22 => self.name = "SHLD".to_string(),
                    0x27 => self.name = "DAA".to_string(),
                    0x2A => self.name = "LHLD".to_string(),
                    0x2F => self.name = "CMA".to_string(),
                    0x32 => self.set_instruction(InstructionTypes::STA),
                    0x37 => self.name = "STC".to_string(),
                    0x3A => self.set_instruction(InstructionTypes::LDA),
                    0x3F => self.name = "CMC".to_string(),
                    _ => panic!("Misc should not exist!"),
                }
            },
            _ => panic!("This match should never happen")

        }

    }

    fn decode_pop(&mut self)
    {
        self.name = format!("POP {}", Registers::translate_to_reg(self.byte1.unwrap()));
        self.inst_type = InstructionTypes::POP;
        self.byte1 = Some(self.byte_val & 0x30);
    }

    fn decode_push(&mut self)
    {
        self.name = format!("PUSH {}", Registers::translate_to_reg(self.byte1.unwrap()));
        self.inst_type = InstructionTypes::PUSH;
        self.byte1 = Some(self.byte_val & 0x30);
    }

    fn decode_inx(&mut self)
    {
        self.set_instruction(InstructionTypes::INX);
        self.byte1 = Some((self.byte_val & 0x30) >> 4);
    }

    fn decode_dad(&mut self)
    {
        self.set_instruction(InstructionTypes::DAD);
        self.byte1 = Some((self.byte_val & 0x30) >> 4);
    }

    fn decode_lxi(&mut self)
    {
        self.adress_mode = AddressingMode::Direct;
        self.set_instruction(InstructionTypes::LXI);
        self.byte1 = Some((self.byte_val & 0x30) >> 4);
    }


    fn decode_dcr(&mut self)
    {
        self.adress_mode = AddressingMode::Direct;
        self.inst_type = InstructionTypes::DCR;
        self.byte1 = Some(self.byte_val & 0x30);
        self.name = format!("DCR {}", Registers::translate_to_reg(self.byte1.unwrap()));
    }

    fn immediate_op_helper(&mut self, name1: String, op1: InstructionTypes, name2: String, op2: InstructionTypes)
    {
        if self.byte_val & 0x0F == 0x06
        {
            self.inst_type = op1;
            self.name = name1;
        }
        else if self.byte_val & 0x0F == 0x0E
        {
            self.inst_type = op2;
            self.name = name2;
        }
        else
        {
            panic!("Error, should either be {} or {}", name1, name2);
        }

    }

    fn byte_to_immediate_op(&mut self)
    {
        self.adress_mode = AddressingMode::Direct;

        match self.byte_val & 0xF0
        {
            0x00 | 0x10 | 0x20 | 0x30 =>
            {
                self.inst_type = InstructionTypes::MVI;
                self.byte1 = Some((self.byte_val & 0x38) >> 3);
                let name = format!("MVI {},d8 ", Registers::translate_to_reg(self.byte1.unwrap()));
                self.name = name;
                return
            },
            // ADI, ACI
            0xC0 => 
            {
                self.immediate_op_helper("ADI".to_string(), InstructionTypes::ADI, "ACI".to_string(), InstructionTypes::ACI);
            },
            // SUI, SBI
            0xD0 => 
            {
                self.immediate_op_helper("SUI".to_string(), InstructionTypes::SUI, "SBI".to_string(), InstructionTypes::SBI);
            },
            // ANI, XRI
            0xE0 => 
            {
                self.immediate_op_helper("ANI".to_string(), InstructionTypes::ANI, "XRI".to_string(), InstructionTypes::XRI);
            },
            // ORI, CPI
            0xF0 =>
            {
                self.immediate_op_helper("ORI".to_string(), InstructionTypes::ORI, "CPI".to_string(), InstructionTypes::CPI);
            },
            _ => (),
        }
    }
    fn set_instruction(&mut self, inst: InstructionTypes)
    {
        self.name = Self::instruction_to_string(inst.clone());
        self.inst_type = inst;
    }

    fn instruction_to_string<T: std::fmt::Debug>(e: T) -> String
    {
        format!("{:?}", e)
    }
}


