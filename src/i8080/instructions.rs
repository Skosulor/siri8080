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
    DCX,
    LDA,
    LDAX,
    STA,
    PUSH,
    POP,
    CALL,
    NOP,
    RET,
    XCHG,
    OUT,
    EI,
    DI,
    INR,
    CP,
    CNZ,
    CC,
    CNC,
    CPO,
    CPE,
    CM,
    CZ,
    RC,
    RNC,
    RZ,
    RNZ,
    RM,
    RP,
    RPE,
    RPO,
    LHLD,
    SHLD,
    Unknown,
}

const MOVE_ARITHMETIC_LOGICAL_INSTRUCTION_GROUP : u8 = 0x0;
const MOVE_INSTRUCTION_GROUP                    : u8 = 0x40;
const ARITHMETIC_LOGICAL_INSTRUCTION_GROUP      : u8 = 0x80;
const BRANCH_STACK_INSTRUCTION_GROUP            : u8 = 0xC0;

const ARITHMETIC_LOGICAL_GROUP_MASK: u8 = 0b10111000;
const OP_CODE_GROUP_MASK:            u8 = 0xC0;

const ADD_GROUP: u8 = 0b10000000;
const ADC_GROUP: u8 = 0b10001000;
const SUB_GROUP: u8 = 0b10010000;
const SBB_GROUP: u8 = 0b10011000;
const ANA_GROUP: u8 = 0b10100000;
const XRA_GROUP: u8 = 0b10101000;
const ORA_GROUP: u8 = 0b10110000;
const CMP_GROUP: u8 = 0b10111000;

#[derive(Clone)]
pub struct Instruction 
{
    pub byte_val: u8,
    name: String,
    // cycles: u8,
    adress_mode: AddressingMode,
    pub instruction_type: InstructionTypes,
    pub low_nibble: Option<u8>,
    pub high_nibble: Option<u8>,
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
            instruction_type: InstructionTypes::Unknown,
            low_nibble: None,
            high_nibble: None,
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

        match b & OP_CODE_GROUP_MASK
        {
            // Move Instructions
            // low_nibble holds the destination register
            // high_nibble holds the source register
            MOVE_INSTRUCTION_GROUP => 
            {
                self.adress_mode = AddressingMode::Direct;
                self.instruction_type = InstructionTypes::MOV;
                self.low_nibble = Some((b >> MOVE_TO) & REGISTER_MASK);
                self.high_nibble = Some((b >> MOVE_FROM) & REGISTER_MASK);
                let name = format!("MOV {},{} ", Registers::translate_to_reg(self.low_nibble.unwrap()), Registers::translate_to_reg(self.high_nibble.unwrap()));
                self.name = name;
                return
            },
            // low_nibble holds the source register
            // high_nibble is unused
            ARITHMETIC_LOGICAL_INSTRUCTION_GROUP =>
            {
                self.low_nibble = Some((b >> ARITHMETIC_WITH) & REGISTER_MASK);
                self.adress_mode = AddressingMode::Direct;
                match b & ARITHMETIC_LOGICAL_GROUP_MASK
                {
                    ADD_GROUP =>
                    {
                        self.instruction_type = InstructionTypes::ADD;
                        let name = format!("ADD {}", Registers::translate_to_reg(self.low_nibble.unwrap()));
                        self.name = name;
                    },
                    ADC_GROUP =>
                    {
                        self.instruction_type = InstructionTypes::ADC;
                        let name = format!("ADC {}", Registers::translate_to_reg(self.low_nibble.unwrap()));
                        self.name = name;
                    },
                    SUB_GROUP =>
                    {
                        self.instruction_type = InstructionTypes::SUB;
                        let name = format!("SUB {}", Registers::translate_to_reg(self.low_nibble.unwrap()));
                        self.name = name;
                    },
                    SBB_GROUP =>
                    {
                        self.instruction_type = InstructionTypes::SBB;
                        let name = format!("SBB {}", Registers::translate_to_reg(self.low_nibble.unwrap()));
                        self.name = name;
                    },
                    ANA_GROUP =>
                    {
                        self.instruction_type = InstructionTypes::ANA;
                        let name = format!("ANA {}", Registers::translate_to_reg(self.low_nibble.unwrap()));
                        self.name = name;
                    }
                    XRA_GROUP =>
                    {
                        self.instruction_type = InstructionTypes::XRA;
                        let name = format!("XRA {}", Registers::translate_to_reg(self.low_nibble.unwrap()));
                        self.name = name;
                    }
                    ORA_GROUP =>
                    {
                        self.instruction_type = InstructionTypes::ORA;
                        let name = format!("ORA {}", Registers::translate_to_reg(self.low_nibble.unwrap()));
                        self.name = name;
                    }
                    CMP_GROUP =>
                    {
                        self.instruction_type = InstructionTypes::CMP;
                        let name = format!("CMP {}", Registers::translate_to_reg(self.low_nibble.unwrap()));
                        self.name = name;
                    }
                    _ => panic!("Arithemtic does not exist!: {:X}", self.byte_val),
                }
            },
            BRANCH_STACK_INSTRUCTION_GROUP => 
            {
                match b 
                {
                    0xC0 => self.set_instruction(InstructionTypes::RNZ),
                    0xC1 | 0xD1 | 0xE1 | 0xF1 => self.decode_pop(),
                    0xC2 => self.set_instruction(InstructionTypes::JNZ),
                    0xC3 => self.set_instruction(InstructionTypes::JMP),
                    0xC4 => self.set_instruction(InstructionTypes::CNZ),
                    0xC5 | 0xD5 | 0xE5 | 0xF5 => self.decode_push(),
                    0xC6 => self.byte_to_immediate_op(),
                    0xC7 => self.name = "RST 0 NOT IMP".to_string(),
                    0xC8 => self.set_instruction(InstructionTypes::RZ),
                    0xC9 => self.set_instruction(InstructionTypes::RET),
                    0xCA => self.set_instruction(InstructionTypes::JZ),
                    0xCB => self.name = "?? NOT IMP".to_string(),
                    0xCC => self.set_instruction(InstructionTypes::CZ),
                    0xCD => self.set_instruction(InstructionTypes::CALL),
                    0xCE => self.byte_to_immediate_op(),
                    0xCF => self.name = "RST NOT IMP".to_string(),
                    0xD0 => self.set_instruction(InstructionTypes::RNC),
                    0xD2 => self.set_instruction(InstructionTypes::JNC),
                    0xD3 => self.set_instruction(InstructionTypes::OUT),
                    0xD4 => self.set_instruction(InstructionTypes::CNC),
                    0xD6 => self.byte_to_immediate_op(),
                    0xD7 => self.name = "RST NOT IMP".to_string(),
                    0xD8 => self.set_instruction(InstructionTypes::RC),
                    0xD9 => self.name = "?? NOT IMP".to_string(),
                    0xDA => self.set_instruction(InstructionTypes::JC),
                    0xDB => self.name = "IN NOT IMP".to_string(),
                    0xDC => self.set_instruction(InstructionTypes::CC),
                    0xDD => self.name = "?? NOT IMP".to_string(),
                    0xDE => self.byte_to_immediate_op(),
                    0xDF => self.name = "RST NOT IMP".to_string(),
                    0xE0 => self.set_instruction(InstructionTypes::RPO),
                    0xE2 => self.set_instruction(InstructionTypes::JPO),
                    0xE3 => self.name = "XTHL NOT IMP".to_string(),
                    0xE4 => self.set_instruction(InstructionTypes::CPO),
                    0xE6 => self.byte_to_immediate_op(),
                    0xE7 => self.name = "RST NOT IMP".to_string(),
                    0xE8 => self.set_instruction(InstructionTypes::RPE),
                    0xE9 => self.name = "PCHL NOT IMP".to_string(),
                    0xEA => self.set_instruction(InstructionTypes::JPE),
                    0xEB => self.set_instruction(InstructionTypes::XCHG),
                    0xEC => self.set_instruction(InstructionTypes::CPE),
                    0xED => self.name = "?? NOT IMP".to_string(),
                    0xEE => self.byte_to_immediate_op(),
                    0xEF => self.name = "RST NOT IMP".to_string(),
                    0xF0 => self.set_instruction(InstructionTypes::RP),
                    0xF2 => self.set_instruction(InstructionTypes::JP),
                    0xF3 => self.set_instruction(InstructionTypes::DI),
                    0xF4 => self.set_instruction(InstructionTypes::CP),
                    0xF6 => self.byte_to_immediate_op(),
                    0xF7 => self.name = "RST NOT IMP".to_string(),
                    0xF8 => self.set_instruction(InstructionTypes::RM),
                    0xF9 => self.name = "SPHL NOT IMP".to_string(),
                    0xFA => self.set_instruction(InstructionTypes::JM),
                    0xFB => self.set_instruction(InstructionTypes::EI),
                    0xFC => self.set_instruction(InstructionTypes::CM),
                    0xFD => self.name = "?? NOT IMP".to_string(),
                    0xFE => self.byte_to_immediate_op(),
                    0xFF => self.name = "RST NOT IMP".to_string(),
                    _ => 
                    {
                        println!("Byte: {:02X}", b);
                        panic!("Should not exist!");
                    }
                }
            },

            MOVE_ARITHMETIC_LOGICAL_INSTRUCTION_GROUP => 
            {
                match b & 0b00111111 
                {
                    0x00                      => self.set_instruction(InstructionTypes::NOP),
                    0x01 | 0x11 | 0x21 | 0x31 => self.decode_lxi(),
                    0x02 | 0x12               => self.name = "STAX NOT IMP".to_string(),
                    0x03 | 0x13 | 0x23 | 0x33 => self.decode_inx(),
                    0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C  => self.decode_inr(),
                    0x05 | 0x0D |  0x15 | 0x1D | 0x25 | 0x2D | 0x35 | 0x3D => self.decode_dcr(),
                    0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E  => self.byte_to_immediate_op(),
                    0x07 => self.set_instruction(InstructionTypes::RLC),
                    0x08 | 0x10 | 0x18 | 0x20 | 0x28 | 0x30 | 0x38        => self.name = "__ NOT IMP".to_string(),
                    0x09 | 0x19 | 0x29 | 0x39 => self.decode_dad(),
                    0x0B | 0x1B | 0x2B | 0x3B => self.decode_dcx(),
                    0x0F => self.set_instruction(InstructionTypes::RRC),
                    0x17 => self.set_instruction(InstructionTypes::RAL),
                    0x1A | 0x0A => self.set_instruction(InstructionTypes::LDAX),
                    0x1F => self.set_instruction(InstructionTypes::RAR),
                    0x22 => self.set_instruction(InstructionTypes::LHLD),
                    0x27 => self.name = "DAA NOT IMP".to_string(),
                    0x2A => self.set_instruction(InstructionTypes::LHLD),
                    0x2F => self.name = "CMA NOT IMP".to_string(),
                    0x32 => self.set_instruction(InstructionTypes::STA),
                    0x37 => self.name = "STC NOT IMP".to_string(),
                    0x3A => self.set_instruction(InstructionTypes::LDA),
                    0x3F => self.name = "CMC NOT IMP".to_string(),
                    _ => panic!("Misc should not exist!"),
                }
            },
            _ => panic!("This match should never happen")

        }

    }

    fn decode_pop(&mut self)
    {
        self.low_nibble = Some(( self.byte_val & 0x30 ) >> 4);
        self.name = format!("POP {}", Registers::translate_to_reg_pair(self.low_nibble.unwrap()));
        self.instruction_type = InstructionTypes::POP;
    }

    fn decode_push(&mut self)
    {
        self.low_nibble = Some(( self.byte_val & 0x30 ) >> 4);
        self.name = format!("PUSH {}", Registers::translate_to_reg_pair(self.low_nibble.unwrap()));
        self.instruction_type = InstructionTypes::PUSH;
    }

    fn decode_inx(&mut self)
    {
        self.set_instruction(InstructionTypes::INX);
        self.low_nibble = Some((self.byte_val & 0x30) >> 4);
    }

    fn decode_dcx(&mut self)
    {
        self.set_instruction(InstructionTypes::DCX);
        self.low_nibble = Some((self.byte_val & 0x30) >> 4);
    }

    fn decode_dad(&mut self)
    {
        self.set_instruction(InstructionTypes::DAD);
        self.low_nibble = Some((self.byte_val & 0x30) >> 4);
    }

    fn decode_lxi(&mut self)
    {
        self.adress_mode = AddressingMode::Direct;
        self.set_instruction(InstructionTypes::LXI);
        self.low_nibble = Some((self.byte_val & 0x30) >> 4);
    }


    fn decode_dcr(&mut self)
    {
        self.adress_mode = AddressingMode::Direct;
        self.instruction_type = InstructionTypes::DCR;
        self.low_nibble = Some((self.byte_val & 0x38) >> 3);
        self.name = format!("DCR {}", Registers::translate_to_reg(self.low_nibble.unwrap()));
    }

    fn decode_inr(&mut self)
    {
        self.instruction_type = InstructionTypes::INR;
        self.low_nibble = Some((self.byte_val & 0x38) >> 3);
        self.name = format!("INR {}", Registers::translate_to_reg(self.low_nibble.unwrap()));
    }

    fn immediate_op_helper(&mut self, name1: String, op1: InstructionTypes, name2: String, op2: InstructionTypes)
    {
        if self.byte_val & 0x0F == 0x06
        {
            self.instruction_type = op1;
            self.name = name1;
        }
        else if self.byte_val & 0x0F == 0x0E
        {
            self.instruction_type = op2;
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
                self.instruction_type = InstructionTypes::MVI;
                self.low_nibble = Some((self.byte_val & 0x38) >> 3);
                let name = format!("MVI {},d8 ", Registers::translate_to_reg(self.low_nibble.unwrap()));
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
        self.instruction_type = inst;
    }

    fn instruction_to_string<T: std::fmt::Debug>(e: T) -> String
    {
        format!("{:?}", e)
    }
}


