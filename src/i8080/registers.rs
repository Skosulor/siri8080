pub const B_REG: u8 = 0b000;
pub const C_REG: u8 = 0b001;
pub const D_REG: u8 = 0b010;
pub const E_REG: u8 = 0b011;
pub const H_REG: u8 = 0b100;
pub const L_REG: u8 = 0b101;
pub const A_REG: u8 = 0b111;
pub const MEM_REF: u8 = 0b110;

pub const BC_PAIR_REG: u8 = 0b00;
pub const DE_PAIR_REG: u8 = 0b01;
pub const HL_PAIR_REG: u8 = 0b10;
pub const SP_REG:      u8 = 0b11;


pub const MOVE_TO: u8 = 3;
pub const MOVE_FROM: u8 = 0;
pub const ARITHMETIC_WITH: u8 = 0;

#[derive(Copy, Clone)]
pub struct Registers 
{
  pub accumulator: u8,
  pub b: u8,
  pub c: u8,
  pub d: u8,
  pub e: u8,
  pub h: u8,
  pub l: u8,
}

impl Registers
{

    pub fn new() -> Registers
    {
        let reg = Registers
        {
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

    pub fn translate_to_reg(reg: u8) -> String
    {
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
