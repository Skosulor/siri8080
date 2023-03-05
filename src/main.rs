use std::io::{stdin, Write};
use termion::input::TermRead;
mod i8080;
mod disassembler;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut rom = "".to_string();

    if args.len() >= 2
    {
        rom.push_str(&env::current_dir().unwrap().to_string_lossy().to_string());  
        rom.push_str("/");
        rom.push_str(&args[1].to_string());
    }
    else
    {
        println!("Input path to ROM");
        println!("./siri8080 PATH_TO_ROM");
        return;
    }

    let mut p = i8080::Processor::from(rom);

    let mut stdin = termion::async_stdin().keys();
    let mut run = false;
    let mut bp_set = false;
    let mut bp = 0;
    let mut pc;
    let mut iteration = 0;
    const PRINT_INTERVAL: usize = 20;

    clear();
    loop
    {
        iteration += 1;
        let input = stdin.next();
        pc = p.get_pc();
        if let Some(Ok(key)) = input 
        {
            match key 
            {
                termion::event::Key::Char('s') => 
                {
                    run = false;
                    p.clock();
                    clear();
                    p.update_disassembler();
                },
                termion::event::Key::Char('q') => {break;},
                termion::event::Key::Char('r') => { clear(); p.reset_pc();},
                termion::event::Key::Char('c') => {run = true},
                termion::event::Key::Char('p') => {run = false},
                termion::event::Key::Char('b') => 
                {
                    std::mem::drop(stdin);
                    bp = get_breakpoint();
                    bp_set = true;
                    clear();
                    run = true;
                    stdin = termion::async_stdin().keys();
                },
                _ => (),
            }
        }

        if bp_set && bp == pc 
        {
            run = false;
        }

        if run
        {
            if iteration > PRINT_INTERVAL 
            {
                p.update_disassembler();
                iteration = 0;
            }
            p.clock();
            clear();
        }
        else{}
    }

}

pub fn clear()
{
        write!(
            std::io::stdout(),
            "{}",
            termion::clear::All
        ).expect("Error clearing screen!");
}


pub fn get_breakpoint() -> usize
{
    print!("BreakPoint:");
    let mut s = String::new();
    let stdin = stdin();
    std::io::stdout().flush().expect("Failed to flush stdout");
    stdin.read_line(&mut s).expect("Failed to read line");
    std::io::stdout().flush().expect("Failed to flush stdout");
    //s.parse::<i32>().unwrap()
    println!("{}",s);
    let r: usize = s.trim().parse().unwrap() ;
    r
}
