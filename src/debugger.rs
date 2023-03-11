use crate::i8080::Processor;
use crate::disassembler;
use std::io::{stdin, Write};
use termion::input::TermRead;

enum DebuggerCmds
{
    Run,
    Step,
    Breakpoint(u16),
    Quit,
    Reset,
    Nop,
}

pub struct Debugger
{
    input: termion::input::Keys<termion::AsyncReader>,
    breakpoints: Vec<u16>,
}

impl Debugger
{
    pub fn default() -> Debugger
    {
        let dgb = Debugger
        {
            input: termion::async_stdin().keys(),
            breakpoints: Vec::new(),
        };
        return dgb
    }

    pub fn execute(&mut self, processor: &mut Processor)
    {
        let cmd = self.get_debug_command();
        match cmd 
        {
            DebuggerCmds::Nop => (),
            DebuggerCmds::Run => self.run_processor(processor),
            DebuggerCmds::Step => step(processor),
            DebuggerCmds::Breakpoint(b) => self.add_breakpoint(b),
            DebuggerCmds::Quit => println!("Quit"),
            DebuggerCmds::Reset => reset_processor(processor),
        }
        // update_disassembler(processor);
    }

    fn add_breakpoint(&mut self, breakpoint: u16)
    {
        self.breakpoints.push(breakpoint);
    }

    fn get_debug_command(&mut self) -> DebuggerCmds
    {
        // let mut stdin = termion::async_stdin().keys();
        let input = self.input.next();
        let mut command: DebuggerCmds = DebuggerCmds::Nop;

        if let Some(Ok(key)) = input
        {
            match key 
            {
                termion::event::Key::Char('s') => {command = DebuggerCmds::Step;},
                termion::event::Key::Char('q') => {command = DebuggerCmds::Quit;},  
                termion::event::Key::Char('r') => {command = DebuggerCmds::Reset;},
                termion::event::Key::Char('c') => {command = DebuggerCmds::Run;}, 
                termion::event::Key::Char('b') => {command = DebuggerCmds::Breakpoint(get_breakpoint());},
                _ => (), 
            }
        }
        return command
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
                update_disassembler(processor);
                break;
            }
        }
        
    }
}


fn get_breakpoint() -> u16
{
    print!("BreakPoint:");
    let mut s = String::new();
    let stdin = stdin();
    std::io::stdout().flush().expect("Failed to flush stdout");
    stdin.read_line(&mut s).expect("Failed to read line");
    std::io::stdout().flush().expect("Failed to flush stdout");
    //s.parse::<i32>().unwrap()
    println!("{}",s);
    let r: u16 = s.trim().parse().unwrap() ;
    r
}


fn step(processor: &mut Processor)
{
    processor.clock();
    update_disassembler(processor);
}

fn reset_processor(processor: &mut Processor)
{
    processor.reset_pc();
    update_disassembler(processor);
}

fn clear()
{
    write!(
        std::io::stdout(),
        "{}",
        termion::clear::All
        ).expect("Error clearing screen!");
}

fn update_disassembler(processor: &mut Processor)
{
    let mut term = disassembler::Term::default();

    term.set_flags(&processor.get_flags());
    term.set_regs(&processor.get_registers());
    term.update_instructions(processor.get_instructions());
    term.set_pc(processor.get_pc());

    clear();
    term.update_dissambler()
}
