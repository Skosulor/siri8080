use crate::i8080::Processor;

mod disassembler;
use std::io::{stdin, Write, stdout};

pub struct Debugger<'a>
{
    breakpoints: Vec<u16>,
    disassembler: disassembler::Disassembler<'a>,
}

impl<'a> Debugger<'a>
{
    pub fn default() -> Debugger<'a>
    {
        let dgb = Debugger
        {
            breakpoints: Vec::new(),
            disassembler: disassembler::Disassembler::default(),
        };
        return dgb
    }

    pub fn execute(&mut self, processor: &mut Processor, first_execution: bool) -> Option<u8>
    {
        if first_execution
        {
            self.update_disassembler(processor);
            self.disassembler.set_memory(0x0, &processor.get_memory());
        }

        let mut ret: Option<u8> = Some(0);
        let inputs = get_input();
        let mut inputs = inputs.split_whitespace();
        let input;


        match inputs.next()
        {
            Some(str) => input = str,
            None => return ret,
        }

        match input
        {
            "s" | "step"       => step(processor, inputs.next()),
            "q" | "quit"       => ret = None,
            "c" | "continue"   => self.run_processor(processor),
            "b" | "breakpoint" => self.add_breakpoint(inputs.next()),
            "r" | "reset"      => reset_processor(processor),
            "m" | "mem"        => self.disassembler.set_memory(input_to_u16(inputs.next()), &processor.get_memory()),
        
            _ => (),
        }
        self.update_disassembler(processor);

        return ret
    }

    fn add_breakpoint(&mut self, breakpoint: Option<&str>)
    {

        match breakpoint
        {
            Some(breakpoint) => 
            {
                let breakpoint = match breakpoint.parse::<u16>()
                {
                    Ok(breakpoint) => breakpoint,
                    Err(_) => return,
                };
                self.breakpoints.push(breakpoint)
            },
            None => return,
        };
    }


    fn run_processor(&mut self, processor: &mut Processor)
    {
        loop
        {
            processor.clock();
            let pc = processor.get_pc();
            let found = self.breakpoints.contains(&pc);
            if found 
            {
                self.update_disassembler(processor);
                break;
            }
        }
    }

    fn update_disassembler(&mut self, processor: &mut Processor)
    {
        self.disassembler.update_instructions(get_instructions(processor));
        self.disassembler.set_stack_pointer(processor.get_stack_pointer());
        self.disassembler.set_flags(&processor.get_flags());
        self.disassembler.set_regs(&processor.get_registers());
        self.disassembler.set_pc(processor.get_pc());
        self.disassembler.set_direct_address(processor.get_direct_address());
        self.disassembler.set_immediate(processor.get_immediate());

        clear();
        self.disassembler.update_dissambler()
    }
}

fn get_input() -> String
{
    let mut input = String::new(); 
    print!("> "); 
    stdout().flush().unwrap(); 
    stdin().read_line(&mut input).expect("Failed to read line"); 
    return input;
}


fn step(processor: &mut Processor, steps: Option<&str>)
{
    match steps 
    {
        Some(steps) => 
        {
            let steps = match steps.parse::<u32>()
            {
                Ok(steps) => steps,
                Err(_) => 1,
            };

            for _ in 0..steps
            {
                processor.clock();
            }
        },
        None => processor.clock(),
    }
}

fn reset_processor(processor: &mut Processor)
{
    processor.reset();
}

fn clear()
{
    write!(
        std::io::stdout(),
        "{}",
        termion::clear::All
        ).expect("Error clearing screen!");
}


// Create a copy of the processor and clock it and read out each instruction name into a vector
// Then return the vector
fn get_instructions(processor: &mut Processor) -> Vec<String>
{
    let mut instructions: Vec<String> = Vec::new();
    let mut processor = processor.clone();
    instructions.push("".to_string());
    for _ in 1 .. 48
    {
        processor.fetch_instruction();
        let instruction = processor.get_current_op();
        let (byte, name) = instruction.get_name_byte();

        instructions.push(String::from(format!("{a:>6}:     0x{b:02X} {c:}", 
                                               a=(processor.get_pc() as usize), b=byte, c=name)));
        processor.clock();
    }
    return instructions
}

fn input_to_u16(input: Option<&str>) -> u16
{
    match input
        {
            Some(value) => 
            {
                match value.parse::<u16>()
                {
                    Ok(value) => return value,
                    Err(_) => return 0,
                };
            },
            None => return 0,
        };
}
