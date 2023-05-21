mod tests
{
    use siri8080::i8080::registers::*;
    use siri8080::i8080::Processor;

    #[test]
    fn xchg()
    {
        let mem = vec![0xEB, 0x00];
        let mut cpu = Processor::from_bytes(mem);
        let reg = Registers
        {
            accumulator: 0,
            b: 0,
            c: 0,
            d: 0x42, 
            e: 0x24,
            h: 0x37,
            l: 0x13,
        };

        cpu.set_all_registers(reg);
        cpu.clock();
        let r = cpu.get_registers();

        assert_eq!(reg.e, r.l);
        assert_eq!(reg.l, r.e);
        assert_eq!(reg.h, r.d);
        assert_eq!(reg.d, r.h);
    }
        

}
