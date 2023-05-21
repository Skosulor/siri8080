mod test
{
    use siri8080::i8080::Processor;

    #[test]
    fn call()
    {
        let mem = vec![0xCD, 0x37, 0x13];
        let mut cpu = Processor::from_bytes(mem);
        cpu.clock();

        let pc = cpu.get_pc();
        assert_eq!(0x1337, pc);
    }
}
