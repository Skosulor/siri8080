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

        // Add three: two bytes for the return address 
        // and one for the next instruction
        let pc_in_stack = cpu.get_pc() + 3;
        cpu.clock();
        let pc = cpu.get_pc();
        assert_eq!(0x1337, pc);

        cpu.clock();
        let pc_after_ret = cpu.get_pc();
        assert_eq!(pc_in_stack, pc_after_ret);
    }

    #[test]
    fn jmp()
    {
        let mut mem = vec![0; 0x1500];
        mem[0]      = 0xC3;
        mem[1]      = 0x37;
        mem[2]      = 0x13;
        mem[0x1337] = 0xC9;

        let mut cpu = Processor::from_bytes(mem);
        cpu.clock();

        // Add three: two bytes for address 
        // and one for the next instruction
        let pc = cpu.get_pc();
        assert_eq!(0x1337, pc);
    }

    #[test]
    fn jnz()
    {
        let mut mem = vec![0; 0x1500];
        mem[0]      = 0xC2;
        mem[1]      = 0x37;
        mem[2]      = 0x13;
        mem[3]      = 0xC2;
        mem[4]      = 0x37;
        mem[5]      = 0x13;
        mem[0x1337] = 0xC9;

        let mut cpu = Processor::from_bytes(mem);

        cpu.set_flags_cszp(false, false, 0);
        cpu.clock();
        let pc = cpu.get_pc();
        println!("flags = {:?}", cpu.get_flags());
        assert_eq!(3, pc);

        cpu.set_flags_cszp(false, false, 1);
        cpu.clock();
        let pc = cpu.get_pc();
        assert_eq!(0x1337, pc);
    }

    #[test]
    fn jz()
    {
        let mut mem = vec![0; 0x1500];
        mem[0]      = 0xCA;
        mem[1]      = 0x37;
        mem[2]      = 0x13;
        mem[0x1337] = 0xCA;
        mem[0x1338] = 0x42;
        mem[0x1339] = 0x00;

        let mut cpu = Processor::from_bytes(mem);

        cpu.set_flags_cszp(false, false, 0);
        cpu.clock();
        let pc = cpu.get_pc();
        assert_eq!(0x1337, pc);


        cpu.set_flags_cszp(false, false, 1);
        cpu.clock();
        let pc = cpu.get_pc();
        assert_eq!(0x133a, pc);
    }

    #[test]
    fn jnc()
    {
        let mut mem = vec![0; 0x1500];
        mem[0]      = 0xD2;
        mem[1]      = 0x37;
        mem[2]      = 0x13;
        mem[0x1337] = 0xD2;
        mem[0x1338] = 0x42;
        mem[0x1339] = 0x00;

        let mut cpu = Processor::from_bytes(mem);

        cpu.set_flags_cszp(false, false, 0);
        cpu.clock();
        let pc = cpu.get_pc();
        assert_eq!(0x1337, pc);

        cpu.set_flags_cszp(true, false, 0);
        cpu.clock();
        let pc = cpu.get_pc();
        assert_eq!(0x133A, pc);
    }

    #[test]
    fn jc()
    {
        let mut mem = vec![0; 0x1500];
        mem[0]      = 0xDA;
        mem[1]      = 0x37;
        mem[2]      = 0x13;
        mem[0x1337] = 0xDA;
        mem[0x1338] = 0x42;
        mem[0x1339] = 0x00;

        let mut cpu = Processor::from_bytes(mem);

        cpu.set_flags_cszp(true, false, 0);
        cpu.clock();
        let pc = cpu.get_pc();
        assert_eq!(0x1337, pc);

        cpu.set_flags_cszp(false, false, 0);
        cpu.clock();
        let pc = cpu.get_pc();
        assert_eq!(0x133A, pc);
    }

    #[test]
    fn jpo()
    {
        let mut mem = vec![0; 0x1500];
        mem[0]      = 0xE2;
        mem[1]      = 0x37;
        mem[2]      = 0x13;
        mem[0x1337] = 0xE2;
        mem[0x1338] = 0x42;
        mem[0x1339] = 0x00;

        let mut cpu = Processor::from_bytes(mem);

        cpu.set_flags_cszp(false, false, 0);
        cpu.clock();
        println!("flags = {:?}", cpu.get_flags());
        let pc = cpu.get_pc();
        assert_eq!(0x1337, pc);

        cpu.set_flags_cszp(false, true, 1);
        cpu.clock();
        let pc = cpu.get_pc();
        assert_eq!(0x133A, pc);
    }

    #[test]
    fn jpe()
    {
        let mut mem = vec![0; 0x1500];
        mem[0]      = 0xEA;
        mem[1]      = 0x37;
        mem[2]      = 0x13;
        mem[0x1337] = 0xEA;
        mem[0x1338] = 0x42;
        mem[0x1339] = 0x00;

        let mut cpu = Processor::from_bytes(mem);

        cpu.set_flags_cszp(false, true, 1);
        cpu.clock();
        let pc = cpu.get_pc();
        assert_eq!(0x1337, pc);

        cpu.set_flags_cszp(false, false, 0);
        cpu.clock();
        let pc = cpu.get_pc();
        assert_eq!(0x133A, pc);
    }

    #[test]
    fn jp()
    {
        let mut mem = vec![0; 0x1500];
        mem[0]      = 0xF2;
        mem[1]      = 0x37;
        mem[2]      = 0x13;
        mem[0x1337] = 0xF2;
        mem[0x1338] = 0x42;
        mem[0x1339] = 0x00;

        let mut cpu = Processor::from_bytes(mem);

        cpu.set_flags_cszp(false, true, 0);
        cpu.clock();
        let pc = cpu.get_pc();
        assert_eq!(0x1337, pc);

        cpu.set_flags_cszp(true, false, 0x80);
        cpu.clock();
        let pc = cpu.get_pc();
        assert_eq!(0x133A, pc);
    }

    #[test]
    fn jm()
    {
        let mut mem = vec![0; 0x1500];
        mem[0]      = 0xFA;
        mem[1]      = 0x37;
        mem[2]      = 0x13;
        mem[0x1337] = 0xFA;
        mem[0x1338] = 0x42;
        mem[0x1339] = 0x00;

        let mut cpu = Processor::from_bytes(mem);

        cpu.set_flags_cszp(true, false, 0x80);
        cpu.clock();
        let pc = cpu.get_pc();
        assert_eq!(0x1337, pc);

        cpu.set_flags_cszp(false, true, 0);
        cpu.clock();
        let pc = cpu.get_pc();
        assert_eq!(0x133A, pc);
    }


}
