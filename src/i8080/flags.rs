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
            carry_flag: false,
            auxiliary_flag: false,
            sign_flag: false,
            zero_flag: false,
            parity_flag: false,
        };
        f
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
