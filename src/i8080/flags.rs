#[derive(Copy, Clone)]
pub struct StatusFlags 
{
    pub carry_flag: bool,
    pub auxiliary_flag: bool,
    pub sign_flag: bool,
    pub zero_flag: bool,
    pub parity_flag: bool,
}

impl StatusFlags 
{
    pub fn new() -> StatusFlags 
    {
        let f = StatusFlags 
        {
            carry_flag:     false,
            auxiliary_flag: false,
            sign_flag:      false,
            zero_flag:      false,
            parity_flag:    false,
        };
        f
    }
    pub fn get_flags_u8(&mut self) -> u8 
    {
        let mut flags:u8 = 0;

        flags |= ( self.carry_flag     as u8) << 0;
        flags |= ( self.auxiliary_flag as u8) << 1;
        flags |= ( self.sign_flag      as u8) << 2;
        flags |= ( self.zero_flag      as u8) << 3;
        flags |= ( self.parity_flag    as u8) << 4;

        return flags
    }

    pub fn set_flags_u8(&mut self, flags: u8)
    {
        self.carry_flag     = (flags & 0x1  ) == 0x1;
        self.auxiliary_flag = (flags & 0x2  ) == 0x2;
        self.sign_flag      = (flags & 0x4  ) == 0x4;
        self.zero_flag      = (flags & 0x8  ) == 0x8;
        self.parity_flag    = (flags & 0x10 ) == 0x10
    }
}


pub fn auxiliary() -> bool 
{
    // TODO
    false
}


pub fn sign(b: u8) -> bool 
{
    if b & 0b10000000 == 0b10000000 
    {
        true
    } 
    else 
    {
        false
    }
}

pub fn zero(b:u8) -> bool
{
    if b == 0 
    {
        true
    } 
    else 
    {
        false
    }
}

pub fn parity(b: u8) -> bool
{
    if b.count_ones() % 2 == 0
    {
        return true
    }
    else
    {
        return false
    }
}
