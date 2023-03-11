use crate::i8080::Processor;
use crate::disassembler;


pub fn execute(processor: &mut Processor)
{
    update_disassembler(processor);
}

fn update_disassembler(processor: &mut Processor)
{
    let mut term = disassembler::Term::default();

    term.set_flags(&processor.get_flags());
    term.set_regs(&processor.get_registers());
    term.update_instructions(processor.get_instructions());
    term.set_pc(processor.get_pc());
    term.update_dissambler()
}

