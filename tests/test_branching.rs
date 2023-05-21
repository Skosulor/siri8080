mod test
{
    use siri8080::i8080::Processor;

    #[test]
    fn call()
    {
        let mut mem = vec![0; 0x1500];
        mem[2]      = 0xCD;
        mem[3]      = 0x37;
        mem[4]      = 0x13;
        mem[0x1337] = 0xC9;

        let mut cpu = Processor::from_bytes(mem);
        cpu.clock();
        cpu.clock();

        // Add three: two bytes for address 
        // and one for the next instruction
        let pc_in_stack = cpu.get_pc() + 3;
        cpu.clock();
        let pc = cpu.get_pc();
        assert_eq!(0x1337, pc);

        cpu.clock();
        let pc_after_ret = cpu.get_pc();
        assert_eq!(pc_in_stack, pc_after_ret);
    }
}
