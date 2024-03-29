use std::env;
use siri8080::*;

fn main() 
{
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

    let mut p = i8080::Processor::from_file(rom);
    let mut dgb = debugger::Debugger::default();

    dgb.execute(&mut p, true);
    loop
    {
        match dgb.execute(&mut p, false)
        {
            Some(_) => (),
            None    => break,
        }
    }
}

