use termion::event::*;
use termion::raw::IntoRawMode;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
mod i8080;
mod disassembler;

fn main() {
    println!("Hello, world!");
    let mut test: u8 = 0x3e;
    let o: u8 = 0x3e;
    let (test, overflow) = test.overflowing_sub(o);
    println!("{}, {}", test, overflow);

    if overflow {
        println!("{}", test as u8);
    }

    let mut p = i8080::Processor::from(
        "/home/ohman/projects/siri8080/rom/space_invaders".to_string(),
    );

    // println!("{}", i8080::parity(0b11110000));
    // println!("{}", i8080::parity(0b01110000));

    // for x in 1..100{
    //     println!("");
    // }

    let mut stdin = termion::async_stdin().keys();
    let mut run = false;
    let mut bp_set = false;
    let mut bp = 0;
    let mut pc = 0;
    let mut iteration = 0;
    const PRINT_INTERVAL: usize = 20;

    clear();
    loop{
        iteration += 1;
        let input = stdin.next();
        pc = p.get_pc();
        if let Some(Ok(key)) = input {
            match key {
                termion::event::Key::Char('s') => {
                    run = false;
                    p.clock(true);
                    clear();
                },
                termion::event::Key::Char('q') => {break;},
                termion::event::Key::Char('r') => { clear(); p.reset_pc();},
                termion::event::Key::Char('c') => {run = true},
                termion::event::Key::Char('p') => {run = false},
                termion::event::Key::Char('b') => {
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
        if bp_set && bp == pc {
            run = false;
        }
        if run{
            if iteration > PRINT_INTERVAL {
                p.update_disassembler();
                iteration = 0;
            }
            p.clock(true);
            clear();
        }else{
            p.update_disassembler();
        }
    }

}

pub fn clear(){
        write!(
            std::io::stdout(),
            "{}",
            termion::clear::All
        );
}


pub fn get_breakpoint() -> usize{
    print!("BreakPoint:");
    let mut s = String::new();
    let stdin = stdin();
    std::io::stdout().flush();
    stdin.read_line(&mut s);
    std::io::stdout().flush();
    //s.parse::<i32>().unwrap()
    println!("{}",s);
    let r: usize = s.trim().parse().unwrap() ;
    r
}
